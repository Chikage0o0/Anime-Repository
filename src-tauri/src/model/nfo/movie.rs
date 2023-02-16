use super::public::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "movie")]
struct Movie {
    title: String,
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

    fn get_id(&self, provider: Provider) -> Option<&String> {
        self.unique_id.iter().find_map(|i| {
            if i.r#type == provider {
                Some(&i.value)
            } else {
                None
            }
        })
    }

    fn get_default_id(&self) -> Option<(&String, &Provider)> {
        self.unique_id.iter().find_map(|i| {
            if i.default == true {
                Some((&i.value, &i.r#type))
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
    pub async fn update(&mut self, lang: &str) {
        use crate::http::tmdb::*;
        if let Some((id, provider)) = self.get_default_id() {
            match provider {
                Provider::Known(ProviderKnown::TMDB) => {
                    let json = get_movie_info(id, lang).await;
                    let data: Value = serde_json::from_str(&json).unwrap();

                    if let Some(title) = data.get("title") {
                        self.title = title.as_str().unwrap().to_string();
                    }

                    if let Some(original_title) = data.get("original_title") {
                        self.original_title = Some(original_title.as_str().unwrap().to_string());
                    }

                    if let Some(imdb_id) = data.get("imdb_id") {
                        let imdb = Provider::Known(ProviderKnown::IMDB);
                        let unique_id = self.unique_id.iter().find(|f| f.r#type == imdb);
                        if unique_id.is_none() {
                            self.unique_id.push(Uniqueid {
                                r#type: imdb,
                                default: false,
                                value: imdb_id.as_str().unwrap().to_string(),
                            })
                        }
                    }

                    if let Some(vote_average) = data.get("vote_average") {
                        if let Some(vote_count) = data.get("vote_count") {
                            if let Some(ratings) = &mut self.ratings {
                                let themoviedb_rating = ratings
                                    .rating
                                    .iter_mut()
                                    .find(|rating| rating.name == "themoviedb");
                                match themoviedb_rating {
                                    Some(rating) => {
                                        rating.value = vote_average.as_f64().unwrap();
                                        rating.votes = vote_count.as_i64().unwrap();
                                    }
                                    None => ratings.rating.push(Rating {
                                        name: "themoviedb".to_string(),
                                        max: 10,
                                        default: true,
                                        value: vote_average.as_f64().unwrap(),
                                        votes: vote_count.as_i64().unwrap(),
                                    }),
                                }
                            }
                        }
                    }

                    if let Some(overview) = data.get("overview") {
                        self.plot = Some(overview.as_str().unwrap().to_string());
                    }

                    if let Some(poster_path) = data.get("poster_path") {
                        self.update_thumb(
                            get_img_url(poster_path.as_str().unwrap()),
                            Some("poster".to_string()),
                            None,
                            None,
                            None,
                        );
                    }

                    if let Some(genres) = data.get("genres") {
                        self.genre = genres
                            .as_array()
                            .unwrap()
                            .into_iter()
                            .map(|f| f["name"].as_str().unwrap().to_string())
                            .collect();
                    }

                    if let Some(release_date) = data.get("release_date") {
                        self.premiered = Some(release_date.as_str().unwrap().to_string());
                    }

                    if let Some(production_countries) = data.get("production_countries") {
                        self.country = production_countries
                            .as_array()
                            .unwrap()
                            .into_iter()
                            .map(|f| f["name"].as_str().unwrap().to_string())
                            .collect()
                    }

                    if let Some(production_companies) = data.get("production_companies") {
                        self.studio = production_companies
                            .as_array()
                            .unwrap()
                            .into_iter()
                            .map(|f| f["name"].as_str().unwrap().to_string())
                            .collect();
                    }

                    if let Some(backdrop_path) = data.get("backdrop_path") {
                        self.fanart = Some(Fanart {
                            thumb: vec![Thumb {
                                aspect: None,
                                r#type: None,
                                season: None,
                                preview: None,
                                value: get_img_url(backdrop_path.as_str().unwrap()),
                            }],
                        });
                    }

                    if let Some(images) = data.get("images") {
                        if let Some(logos) = images.get("logos") {
                            if let Some(logo) = logos.as_array().unwrap().get(0) {
                                self.update_thumb(
                                    get_img_url(logo["file_path"].as_str().unwrap()),
                                    Some("clearlogo".to_string()),
                                    None,
                                    None,
                                    None,
                                );
                            }
                        }
                    }

                    if let Some(belongs_to_collection) = data.get("belongs_to_collection") {
                        let collection =
                            belongs_to_collection["name"].as_str().unwrap().to_string();
                        let set = self.set.iter().find(|f| f.name == collection);
                        if set.is_none() {
                            self.set.push(Set {
                                name: collection,
                                overview: None,
                            })
                        }
                    }
                }
                _ => todo!(),
            }
        }
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
        assert!(data.get_id(Provider::Known(ProviderKnown::TMDB)) == Some(&"532321".to_string()));
    }

    #[test]
    fn test_update() {
        use tauri::async_runtime::block_on;
        let mut data: Movie = Movie::new("532321", Provider::Known(ProviderKnown::TMDB));
        block_on(data.update("zh-CN"));
        println!("{:#?}", data);
        assert!(data.premiered == Some("2018-10-06".to_string()))
    }
}
