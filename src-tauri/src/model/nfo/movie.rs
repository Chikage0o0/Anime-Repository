use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "movie")]
pub struct Movie {
    pub title: String,
    #[serde(rename = "originaltitle")]
    original_title: Option<String>,
    #[serde(rename = "sorttitle")]
    sort_title: Option<String>,
    ratings: Option<Ratings>,
    #[serde(rename = "userrating")]
    user_rating: Option<String>,
    top250: Option<i64>,
    outline: Option<String>,
    plot: Option<String>,
    tagline: Option<String>,
    runtime: Option<String>,
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
    #[serde(default)]
    set: Vec<Set>,
    #[serde(default)]
    country: Vec<String>,
    #[serde(default)]
    credits: Vec<String>,
    #[serde(default)]
    director: Vec<String>,
    premiered: Option<String>,
    #[serde(default)]
    studio: Vec<String>,
    trailer: Option<String>,
    #[serde(default)]
    actor: Vec<Actor>,
    showlink: Option<String>,
    resume: Option<Resume>,
    #[serde(rename = "dateadded")]
    date_added: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Set {
    name: String,
    overview: Option<String>,
}

impl Nfo for Movie {
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

    fn read_from_file() -> Self {
        todo!()
    }
}
impl Movie {
    pub async fn update(&mut self, lang: &str) -> Result<(), NfoGetError> {
        use crate::http::tmdb::*;
        if let Some((id, provider)) = self.get_default_id() {
            match provider {
                Provider::Known(ProviderKnown::TMDB) => {
                    log::info!("Get movie with id: {} from TMDB", id);
                    let json = get_json(TMDBClient::default().get_movie_info(&id, lang).await?)?;
                    let data: Value = serde_json::from_str(&json)?;

                    // Fall back to origin_language if no data is found
                    let mut data_fallback: Value = Value::Null;
                    if let Some(fall_back_lang) =
                        data.get("original_language").and_then(|f| f.as_str())
                    {
                        if fall_back_lang != lang {
                            data_fallback = serde_json::from_str(&get_json(
                                TMDBClient::default()
                                    .get_movie_info(&id.clone(), &fall_back_lang)
                                    .await?,
                            )?)?;
                        }
                    }

                    if let Some(title) = data.get("title").and_then(|f| f.as_str()) {
                        self.title = title.to_string();
                    }

                    if let Some(original_title) =
                        data.get("original_title").and_then(|f| f.as_str())
                    {
                        self.original_title = Some(original_title.to_string());
                    }

                    if let Some(imdb_id) = data.get("imdb_id").and_then(|f| f.as_str()) {
                        let imdb = Provider::Known(ProviderKnown::IMDB);
                        let unique_id = self.unique_id.iter().find(|f| f.r#type == imdb);
                        if unique_id.is_none() {
                            self.unique_id.push(Uniqueid {
                                r#type: imdb,
                                default: false,
                                value: imdb_id.to_string(),
                            })
                        }
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
                            .flat_map(|f| {
                                f.get("name")
                                    .and_then(|f| f.as_str())
                                    .and_then(|f| Some(f.to_string()))
                            })
                            .collect();
                    }

                    if let Some(release_date) = data.get("release_date").and_then(|f| f.as_str()) {
                        self.premiered = Some(release_date.to_string());
                    }

                    if let Some(production_countries) =
                        data.get("production_countries").and_then(|f| f.as_array())
                    {
                        self.country = production_countries
                            .into_iter()
                            .filter_map(|f| {
                                f.get("name")
                                    .and_then(|f| f.as_str())
                                    .and_then(|f| Some(f.to_string()))
                            })
                            .collect()
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

                    let mut set_logo = |logo_data: &Value| {
                        if let Some(logo) = logo_data.get("file_path").and_then(|f| f.as_str()) {
                            self.update_thumb(
                                get_img_url(logo),
                                Some("clearlogo".to_string()),
                                None,
                                None,
                                None,
                            );
                        };
                    };
                    if let Some(logos) = data
                        .get("images")
                        .and_then(|f| f.get("logos"))
                        .and_then(|f| f.as_array())
                        .and_then(|f| f.first())
                    {
                        set_logo(logos);
                    } else if let Some(logos) = data_fallback
                        .get("images")
                        .and_then(|f| f.get("logos"))
                        .and_then(|f| f.as_array())
                        .and_then(|f| f.first())
                    {
                        set_logo(logos);
                    }

                    if let Some(belongs_to_collection) = data
                        .get("belongs_to_collection")
                        .and_then(|f| f.get("name"))
                        .and_then(|f| f.as_str())
                    {
                        let collection = belongs_to_collection.to_string();
                        let set = self.set.iter().find(|f| f.name == collection);
                        if set.is_none() {
                            self.set.push(Set {
                                name: collection,
                                overview: None,
                            })
                        }
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

    pub fn get_year(&self) -> Option<u64> {
        if let Some(premiered) = &self.premiered {
            let year = premiered
                .split('-')
                .next()
                .and_then(|f| f.parse::<u64>().ok());
            year
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const NFO: &str = r#"
        <?xml version="1.0" encoding="utf-8" standalone="yes"?>
        <movie>
          <plot>昴等人打倒了诅咒的元凶——魔兽沃尔加姆，拯救了阿拉姆村的孩子们。终于到来的安稳并未持续很久，昴就瞒着所有人前去挑战某个极秘任务。但尽管昴做了变装，也很快就被以佩特拉为首的村里的孩子们识破了真实身份。开始后5秒就曝光的这一任务，是事前调查和爱蜜莉雅约会的路线……</plot>
          <dateadded>2022-12-05 17:08:48</dateadded>
          <title>Re：从零开始的异世界生活 雪之回忆</title>
          <originaltitle>Re:ゼロから始める異世界生活 Memory Snow</originaltitle>
          <director>Masaharu Watanabe</director>
          <director>Tatsuya Koyanagi</director>
          <writer>Masahiro Yokotani</writer>
          <writer>Tappei Nagatsuki</writer>
          <writer>Shinichi Otsuka</writer>
          <credits>Masahiro Yokotani</credits>
          <credits>Tappei Nagatsuki</credits>
          <credits>Shinichi Otsuka</credits>
          <trailer>plugin://plugin.video.youtube/?action=play_video&amp;videoid=Vg40gSVVI2M</trailer>
          <rating>7.2</rating>
          <year>2018</year>
          <collectionnumber>689287</collectionnumber>
          <imdbid>tt8565186</imdbid>
          <tmdbid>532321</tmdbid>
          <premiered>2018-10-06</premiered>
          <releasedate>2018-10-06</releasedate>
          <runtime>60</runtime>
          <country>日本</country>
          <genre>Animation</genre>
          <genre>Adventure</genre>
          <genre>Fantasy</genre>
          <studio>Kadokawa</studio>
          <studio>White Fox</studio>
          <tag>magic</tag>
          <tag>cold</tag>
          <tag>village</tag>
          <tag>time loop</tag>
          <tag>dating</tag>
          <tag>nature</tag>
          <tag>parallel world</tag>
          <tag>drunk</tag>
          <tag>festival</tag>
          <tag>ice sculpture</tag>
          <tag>frozen</tag>
          <tag>child</tag>
          <tag>anime</tag>
          <tmdbsetid>689287</tmdbsetid>
          <art>
            <poster>/movies/Re：从零开始的异世界生活 雪之回忆 (2018)/Re：从零开始的异世界生活 雪之回忆 (2018) 1080p FLAC-poster.jpg</poster>
            <fanart>/movies/Re：从零开始的异世界生活 雪之回忆 (2018)/Re：从零开始的异世界生活 雪之回忆 (2018) 1080p FLAC-fanart.jpg</fanart>
            <fanart>/movies/Re：从零开始的异世界生活 雪之回忆 (2018)/Re：从零开始的异世界生活 雪之回忆 (2018) 1080p FLAC-fanart.jpg</fanart>
          </art>
          <actor>
            <name>Yusuke Kobayashi</name>
            <role>Natsuki Subaru (voice)</role>
            <type>Actor</type>
            <thumb>/config/data/metadata/People/Y/Yusuke Kobayashi/folder.jpg</thumb>
          </actor>
          <actor>
            <name>Rie Takahashi</name>
            <role>Emilia (voice)</role>
            <type>Actor</type>
            <thumb>/config/data/metadata/People/R/Rie Takahashi/folder.jpg</thumb>
          </actor>
          <actor>
            <name>Inori Minase</name>
            <role>Rem (voice)</role>
            <type>Actor</type>
            <thumb>/config/data/metadata/People/I/Inori Minase/folder.jpg</thumb>
          </actor>
          <actor>
            <name>Yumi Uchiyama</name>
            <role>Puck (voice)</role>
            <type>Actor</type>
            <thumb>/config/data/metadata/People/Y/Yumi Uchiyama/folder.jpg</thumb>
          </actor>
          <actor>
            <name>Rie Murakawa</name>
            <role>Ram (voice)</role>
            <type>Actor</type>
            <thumb>/config/data/metadata/People/R/Rie Murakawa/folder.jpg</thumb>
          </actor>
          <actor>
            <name>Satomi Arai</name>
            <role>Beatrice (voice)</role>
            <type>Actor</type>
            <thumb>/config/data/metadata/People/S/Satomi Arai/folder.jpg</thumb>
          </actor>
          <actor>
            <name>Takehito Koyasu</name>
            <role>Roswaal L. Mathers (voice)</role>
            <type>Actor</type>
            <thumb>/config/data/metadata/People/T/Takehito Koyasu/folder.jpg</thumb>
          </actor>
          <id>tt8565186</id>
          <set><name>Re：从零开始的异世界生活（系列）</name></set>
          <fileinfo>
            <streamdetails>
              <video>
                <codec>hevc</codec>
                <micodec>hevc</micodec>
                <bitrate>3726455</bitrate>
                <width>1920</width>
                <height>1080</height>
                <aspect>16:9</aspect>
                <aspectratio>16:9</aspectratio>
                <framerate>23.976025</framerate>
                <scantype>progressive</scantype>
                <default>True</default>
                <forced>False</forced>
                <duration>60</duration>
                <durationinseconds>3611</durationinseconds>
              </video>
              <audio>
                <codec>flac</codec>
                <micodec>flac</micodec>
                <bitrate>589919</bitrate>
                <language>jpn</language>
                <scantype>progressive</scantype>
                <channels>2</channels>
                <samplingrate>48000</samplingrate>
                <default>True</default>
                <forced>False</forced>
              </audio>
              <subtitle>
                <codec>ass</codec>
                <micodec>ass</micodec>
                <language>chi</language>
                <scantype>progressive</scantype>
                <default>True</default>
                <forced>False</forced>
              </subtitle>
            </streamdetails>
          </fileinfo>
          <epbookmark />
          <ratings>
            <rating default="false" max="10" name="themoviedb">
              <value>7.2</value>
              <votes>157</votes>
            </rating>
          </ratings>
          <top250>0</top250>
          <thumb aspect="poster">https://image.tmdb.org/t/p/original/y7XwmyE5ue9hjk65fEWpO2hGU2B.jpg</thumb>
          <fanart>
            <thumb>https://image.tmdb.org/t/p/original/t9ifVuOtSZBvtieF9L83pnHnOcC.jpg</thumb>
          </fanart>
          <certification />
          <uniqueid default="false" type="tmdb">532321</uniqueid>
          <uniqueid default="false" type="tmdbSet">689287</uniqueid>
          <uniqueid default="true" type="imdb">tt8565186</uniqueid>
          <status />
          <code />
          <languages>Japanese</languages>
          <source>UNKNOWN</source>
          <edition>NONE</edition>
          <original_filename>Re Zero kara Hajimeru Isekai Seikatsu Memory Snow.mkv</original_filename>
          <user_note />
        </movie>"#;
    #[test]
    fn test_get_movie_info() {
        let data: Movie = quick_xml::de::from_str(NFO).unwrap();
        assert!(data.get_id(Provider::Known(ProviderKnown::TMDB)) == Some("532321".to_string()));
    }

    #[test]
    fn test_update() {
        use tauri::async_runtime::block_on;
        let mut data: Movie = Movie::new("937278", Provider::Known(ProviderKnown::TMDB));
        let _result = block_on(data.update("zh-CN"));
        dbg!(data);
    }
}
