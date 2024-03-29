use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "tvshow")]
pub struct Tvshow {
    pub title: String,
    #[serde(rename = "originaltitle")]
    original_title: Option<String>,
    #[serde(rename = "sorttitle")]
    sort_title: Option<String>,
    ratings: Option<Ratings>,
    #[serde(rename = "userrating")]
    user_rating: Option<String>,
    top250: Option<i64>,
    season: Option<i64>,
    episode: Option<i64>,
    plot: Option<String>,
    tagline: Option<String>,
    #[serde(default)]
    thumb: Vec<Thumb>,
    fanart: Option<Fanart>,
    mpaa: Option<String>,
    playcount: Option<i64>,
    lastplayed: Option<String>,
    #[serde(rename = "uniqueid")]
    unique_id: Vec<Uniqueid>,
    #[serde(default)]
    genre: Vec<String>,
    #[serde(default)]
    tag: Vec<String>,
    premiered: Option<String>,
    status: Option<String>,
    #[serde(default)]
    studio: Vec<String>,
    trailer: Option<String>,
    #[serde(default)]
    actor: Vec<Actor>,
    #[serde(default, rename = "nameseason")]
    name_season: Vec<Namedseason>,
    #[serde(rename = "dateadded")]
    date_added: Option<String>,
    #[serde(skip)]
    original_language: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Namedseason {
    #[serde(rename = "@number")]
    number: i64,
    #[serde(rename = "$value")]
    value: String,
}

impl Nfo for Tvshow {
    fn new(id: &str, provider: Provider) -> Self {
        Self {
            unique_id: vec![Uniqueid {
                r#type: provider,
                default: true,
                value: id.to_string(),
            }],
            date_added: Some(get_date()),
            ..Default::default()
        }
    }

    fn get_id(&self, provider: Provider) -> Option<String> {
        self.unique_id.iter().find_map(|i| {
            if i.r#type == provider {
                Some(i.value.clone())
            } else {
                None
            }
        })
    }

    fn get_default_id(&self) -> Option<(String, Provider)> {
        self.unique_id.iter().find_map(|i| {
            if i.default == true {
                Some((i.value.clone(), i.r#type.clone()))
            } else {
                None
            }
        })
    }
}
impl Tvshow {
    pub async fn update(&mut self, lang: &str) -> Result<(), NfoGetError> {
        use crate::http::tmdb::*;
        if let Some((id, provider)) = self.get_default_id() {
            match provider {
                Provider::Known(ProviderKnown::TMDB) => {
                    log::info!("Get tvshow with id: {} from TMDB", &id);
                    let json = get_json(TMDBClient::default().get_tvshow_info(&id, lang).await?)?;
                    let data: Value = serde_json::from_str(&json)?;

                    // Fall back to origin_language if no data is found
                    let mut data_fallback: Value = Value::Null;
                    if let Some(fall_back_lang) =
                        data.get("original_language").and_then(|f| f.as_str())
                    {
                        if fall_back_lang != lang {
                            self.original_language = Some(fall_back_lang.to_string());
                            data_fallback = serde_json::from_str(&get_json(
                                TMDBClient::default()
                                    .get_tvshow_info(&id.clone(), &fall_back_lang)
                                    .await?,
                            )?)?;
                        }
                    }

                    if let Some(name) = data.get("name").and_then(|f| f.as_str()) {
                        self.title = name.to_string();
                    }

                    if let Some(original_name) = data.get("original_name").and_then(|f| f.as_str())
                    {
                        self.original_title = Some(original_name.to_string());
                    }

                    if let Some(vote_average) = data.get("vote_average").and_then(|f| f.as_f64()) {
                        if let Some(vote_count) = data.get("vote_count").and_then(|f| f.as_i64()) {
                            if let Some(ratings) = &mut self.ratings {
                                let themoviedb_rating = ratings
                                    .rating
                                    .iter_mut()
                                    .find(|rating| rating.name == "themoviedb");
                                match themoviedb_rating {
                                    Some(rating) => {
                                        rating.value = vote_average;
                                        rating.votes = vote_count;
                                    }
                                    None => ratings.rating.push(Rating {
                                        name: "themoviedb".to_string(),
                                        max: 10,
                                        default: true,
                                        value: vote_average,
                                        votes: vote_count,
                                    }),
                                }
                            } else {
                                self.ratings = Some(Ratings {
                                    rating: vec![Rating {
                                        name: "themoviedb".to_string(),
                                        max: 10,
                                        default: true,
                                        value: vote_average,
                                        votes: vote_count,
                                    }],
                                })
                            }
                        }
                    }

                    // Fall back to origin_language if no data is found
                    if let Some(overview) = data.get("overview").and_then(|f| f.as_str()) {
                        if overview != "" {
                            self.plot = Some(overview.to_string());
                        } else {
                            if let Some(overview) =
                                data_fallback.get("overview").and_then(|f| f.as_str())
                            {
                                self.plot = Some(overview.to_string());
                            }
                        }
                    }

                    if let Some(poster_path) = data.get("poster_path").and_then(|f| f.as_str()) {
                        self.update_thumb(
                            get_img_url(poster_path),
                            Some("poster".to_string()),
                            None,
                            None,
                            None,
                        );
                    }

                    if let Some(genres) = data.get("genres").and_then(|f| f.as_array()) {
                        self.genre = genres
                            .into_iter()
                            .filter_map(|f| {
                                f.get("name")
                                    .and_then(|f| f.as_str())
                                    .and_then(|f| Some(f.to_string()))
                            })
                            .collect();
                    }

                    if let Some(first_air_date) =
                        data.get("first_air_date").and_then(|f| f.as_str())
                    {
                        if first_air_date != &Value::Null {
                            self.premiered = Some(first_air_date.to_string());
                        }
                    }

                    if let Some(production_companies) =
                        data.get("production_companies").and_then(|f| f.as_array())
                    {
                        self.studio = production_companies
                            .into_iter()
                            .filter_map(|f| {
                                f.get("name")
                                    .and_then(|f| f.as_str())
                                    .and_then(|f| Some(f.to_string()))
                            })
                            .collect();
                    }

                    if let Some(seasons) = data.get("seasons").and_then(|f| f.as_array()) {
                        self.name_season = seasons
                            .into_iter()
                            .filter_map(|f| {
                                f.get("season_number").and_then(|f| f.as_i64()).and_then(
                                    |season_number| {
                                        if let Some(poster_path) =
                                            f.get("poster_path").and_then(|f| f.as_str())
                                        {
                                            self.update_thumb(
                                                get_img_url(poster_path),
                                                Some("poster".to_string()),
                                                Some("season".to_string()),
                                                Some(season_number),
                                                None,
                                            );
                                        }
                                        if let Some(name) = f.get("name").and_then(|f| f.as_str()) {
                                            Some(Namedseason {
                                                number: season_number,
                                                value: name.to_string(),
                                            })
                                        } else {
                                            Some(Namedseason {
                                                number: season_number,
                                                value: format!("Season {}", season_number),
                                            })
                                        }
                                    },
                                )
                            })
                            .collect();
                    }

                    if let Some(backdrop_path) = data.get("backdrop_path").and_then(|f| f.as_str())
                    {
                        if backdrop_path != &Value::Null {
                            self.fanart = Some(Fanart {
                                thumb: vec![Thumb {
                                    aspect: None,
                                    r#type: None,
                                    season: None,
                                    preview: None,
                                    value: get_img_url(backdrop_path),
                                }],
                            });
                        }
                    }

                    if let Some(logo_url) = get_logo(&data, &data_fallback) {
                        self.update_thumb(
                            logo_url,
                            Some("clearlogo".to_string()),
                            None,
                            None,
                            None,
                        );
                    }

                    if let Some(cast) = data
                        .get("credits")
                        .and_then(|f| f.get("cast"))
                        .and_then(|f| f.as_array())
                    {
                        for actor in cast {
                            let name = actor
                                .get("name")
                                .and_then(|f| f.as_str())
                                .unwrap_or("Unknown")
                                .to_string();
                            let role = actor
                                .get("character")
                                .and_then(|f| f.as_str())
                                .unwrap_or("Unknown")
                                .to_string();
                            let order = actor.get("order").and_then(|f| f.as_u64());
                            let thumb = actor
                                .get("profile_path")
                                .and_then(|f| f.as_str())
                                .map(|f| get_img_url(f));
                            self.actor.push(Actor {
                                name,
                                role,
                                order,
                                thumb,
                            });
                        }
                    }
                }
                _ => todo!(),
            }
        }
        Ok(())
    }

    fn update_thumb(
        &mut self,
        img_path: String,
        aspect: Option<String>,
        r#type: Option<String>,
        season: Option<i64>,
        preview: Option<String>,
    ) {
        let poster_thumb = self.thumb.iter_mut().find(|thumb| {
            thumb.aspect == aspect && thumb.r#type == r#type && thumb.season == season
        });
        match poster_thumb {
            Some(thumb) => {
                thumb.value = img_path;
            }
            None => self.thumb.push(Thumb {
                aspect,
                r#type,
                season,
                preview,
                value: img_path,
            }),
        }
    }

    pub fn get_thumb<P: AsRef<Path>>(&self, path: P) -> HashMap<PathBuf, String> {
        let path = path.as_ref();
        let mut thumbs: HashMap<PathBuf, String> = HashMap::new();
        // get clearlogo
        if let Some(clearlogo) = self
            .thumb
            .iter()
            .find(|thumb| thumb.aspect == Some("clearlogo".to_string()))
        {
            thumbs.insert(
                path.join("clearlogo".to_string() + &thumb_extension(&clearlogo.value, "png")),
                clearlogo.value.clone(),
            );
        }

        //get poster
        if let Some(poster) = self
            .thumb
            .iter()
            .find(|thumb| thumb.aspect == Some("poster".to_string()) && thumb.r#type == None)
        {
            thumbs.insert(
                path.join("poster".to_string() + &thumb_extension(&poster.value, "jpg")),
                poster.value.clone(),
            );
        }
        // get season poster
        for thumb in self.thumb.iter().filter(|thumb| {
            thumb.r#type == Some("season".to_string())
                && thumb.aspect == Some("poster".to_string())
                && thumb.season != None
        }) {
            let season_poster = if thumb.season.unwrap() == 0 {
                path.join(
                    "season-specials-poster".to_string() + &thumb_extension(&thumb.value, "jpg"),
                )
            } else {
                path.join(format!(
                    "season{:02}-poster{}",
                    thumb.season.unwrap(),
                    thumb_extension(&thumb.value, "jpg")
                ))
            };
            thumbs.insert(season_poster, thumb.value.clone());
        }

        //get fanart
        if let Some(fanart) = &self.fanart {
            for thumb in &fanart.thumb {
                thumbs.insert(
                    path.join("fanart".to_string() + &thumb_extension(&thumb.value, "jpg")),
                    thumb.value.clone(),
                );
            }
        }

        thumbs
    }

    pub fn get_fallback_lang(&self) -> String {
        if let Some(fallback_lang) = &self.original_language {
            fallback_lang.clone()
        } else {
            "en-US".to_string()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    const NFO: &str = r#"
        <?xml version="1.0" encoding="utf-8" standalone="yes"?>
        <tvshow>
          <plot>男高中生五条新菜梦想着成为制作雏人形面部的「头师」。全身心地投入到制作雏人形后，他离同龄人的流行话题越来越远，以至于迟迟无法融入班级。对于这样的新菜来说，一直是班级中心人物的喜多川海梦简直像是生活在完全不同世界的人。然而有一天，一件意想不到的事情让他与海梦拥有了共同的秘密……！？两人原本毫无交集的世界因此产生了联系！</plot>
          <outline>男高中生五条新菜梦想着成为制作雏人形面部的「头师」。全身心地投入到制作雏人形后，他离同龄人的流行话题越来越远，以至于迟迟无法融入班级。对于这样的新菜来说，一直是班级中心人物的喜多川海梦简直像是生活在完全不同世界的人。然而有一天，一件意想不到的事情让他与海梦拥有了共同的秘密……！？两人原本毫无交集的世界因此产生了联系！</outline>
          <lockdata>false</lockdata>
          <dateadded>2023-01-26 23:57:49</dateadded>
          <title>更衣人偶坠入爱河</title>
          <originaltitle>その着せ替え人形は恋をする</originaltitle>
          <trailer>plugin://plugin.video.youtube/?action=play_video&amp;videoid=8oveGY6h6T8</trailer>
          <rating>8.6</rating>
          <year>2022</year>
          <mpaa>US:TV-14</mpaa>
          <imdb_id>tt15765670</imdb_id>
          <tmdbid>123249</tmdbid>
          <premiered>2022-01-09</premiered>
          <releasedate>2022-01-09</releasedate>
          <runtime>24</runtime>
          <country>日本</country>
          <genre>Animation</genre>
          <genre>Comedy</genre>
          <genre>Drama</genre>
          <studio>Gunma TV</studio>
          <studio>Tokyo MX</studio>
          <studio>BS11</studio>
          <studio>Tochigi TV</studio>
          <studio>Aniplex</studio>
          <studio>Movic</studio>
          <studio>SQUARE ENIX</studio>
          <studio>STUDIO MAUSU</studio>
          <studio>Nippon BS Broadcasting</studio>
          <studio>CloverWorks</studio>
          <tag>adolescence</tag>
          <tag>romance</tag>
          <tag>slice of life</tag>
          <tag>school</tag>
          <tag>based on manga</tag>
          <tag>cosplay</tag>
          <tag>ecchi</tag>
          <tag>anime</tag>
          <tag>otaku</tag>
          <tag>erotic</tag>
          <tvdbid>401233</tvdbid>
          <art>
            <poster>/tv/Chikage's/更衣人偶坠入爱河 (2022)/poster.jpg</poster>
            <fanart>/tv/Chikage's/更衣人偶坠入爱河 (2022)/fanart.jpg</fanart>
          </art>
          <actor>
            <name>Shoya Ishige</name>
            <role>Wakana Gojo (voice)</role>
            <type>Actor</type>
            <thumb>/config/data/metadata/People/S/Shoya Ishige/folder.jpg</thumb>
          </actor>
          <actor>
            <name>Hina Suguta</name>
            <role>Marin Kitagawa (voice)</role>
            <type>Actor</type>
            <thumb>/config/data/metadata/People/H/Hina Suguta/folder.jpg</thumb>
          </actor>
          <actor>
            <name>Atsumi Tanezaki</name>
            <role>Sajuna Inui (voice)</role>
            <type>Actor</type>
            <thumb>/config/data/metadata/People/A/Atsumi Tanezaki/folder.jpg</thumb>
          </actor>
          <actor>
            <name>Hina Yomiya</name>
            <role>Shinju Inui (voice)</role>
            <type>Actor</type>
            <thumb>/config/data/metadata/People/H/Hina Yomiya/folder.jpg</thumb>
          </actor>
          <actor>
            <name>Atsushi Ono</name>
            <role>Kaoru Gojo (voice)</role>
            <type>Actor</type>
            <thumb>/config/data/metadata/People/A/Atsushi Ono/folder.jpg</thumb>
          </actor>
          <id>401233</id>
          <episodeguide>
            <url cache="401233.xml">http://www.thetvdb.com/api/1D62F2F90030C444/series/401233/all/zh.zip</url>
          </episodeguide>
          <season>-1</season>
          <episode>-1</episode>
          <status>Continuing</status>
          <showtitle>更衣人偶坠入爱河</showtitle>
          <ratings>
            <rating  default="false" max="10" name="themoviedb">
              <value>8.6</value>
              <votes>340</votes>
            </rating>
          </ratings>
          <thumb aspect="poster">https://image.tmdb.org/t/p/original/w09TpdruCEhZcIPGDMhM6sGDhg7.jpg</thumb>
          <namedseason number="1">更衣人偶坠入爱河</namedseason>
          <thumb aspect="poster" season="1" type="season">https://image.tmdb.org/t/p/original/gta2TihFZxwrCLmoqVDpuMpruYC.jpg</thumb>
          <fanart>
            <thumb>https://image.tmdb.org/t/p/original/gWPK2RIVJ6i3myf7Xdw8DqlznT8.jpg</thumb>
          </fanart>
          <certification>US:TV-14</certification>
          <uniqueid default="false" type="tmdb">123249</uniqueid>
          <uniqueid  type="imd">tt15765670</uniqueid>
          <uniqueid default="true" type="tvdb">401233</uniqueid>
          <user_note />
        </tvshow>"#;

    #[test]
    fn test_get_tvshow_info() {
        let data: Tvshow = quick_xml::de::from_str(NFO).unwrap();
        assert!(data.get_id(Provider::Known(ProviderKnown::TMDB)) == Some("123249".to_string()));
    }

    #[tokio::test]
    async fn test_update() {
        let mut data: Tvshow = Tvshow::new("123249", Provider::Known(ProviderKnown::TMDB));
        data.update("zh-CN").await.unwrap();
        dbg!(&data);
        assert!(data.premiered == Some("2022-01-09".to_string()))
    }
}
