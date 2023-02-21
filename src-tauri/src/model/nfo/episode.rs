use super::public::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "episodedetails")]
pub struct Episode {
    pub title: String,
    #[serde(rename = "originaltitle")]
    original_title: Option<String>,
    ratings: Option<Ratings>,
    #[serde(rename = "userrating")]
    user_rating: Option<String>,
    #[serde(rename = "displayepisode")]
    display_episode: Option<u64>,
    #[serde(rename = "displayseason")]
    display_season: Option<u64>,
    plot: Option<String>,
    tagline: Option<String>,
    runtime: Option<String>,
    #[serde(default)]
    thumb: Vec<Thumb>,
    playcount: Option<i64>,
    lastplayed: Option<String>,
    #[serde(rename = "uniqueid")]
    unique_id: Vec<Uniqueid>,
    #[serde(default)]
    genre: Vec<String>,
    #[serde(default)]
    credits: Vec<String>,
    #[serde(default)]
    director: Vec<String>,
    premiered: Option<String>,
    aired: Option<String>,
    #[serde(default)]
    studio: Vec<String>,
    #[serde(default)]
    actor: Vec<Actor>,
    showlink: Option<String>,
    resume: Option<Resume>,
    #[serde(rename = "dateadded")]
    date_added: Option<String>,
}

impl Nfo for Episode {
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

impl Episode {
    pub async fn update(&mut self, lang: &str, season: u64, episode: u64) {
        use crate::http::tmdb::*;
        if let Some((id, provider)) = self.get_default_id() {
            match provider {
                Provider::Known(ProviderKnown::TMDB) => {
                    let json = get_tv_episode_info(id, season, episode, lang).await;
                    let data: Value = serde_json::from_str(&json).unwrap();

                    self.display_episode = Some(episode);
                    self.display_season = Some(season);

                    if let Some(name) = data.get("name") {
                        self.title = name.as_str().unwrap().to_string();
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

                    if let Some(still_path) = data.get("still_path") {
                        self.update_thumb(
                            get_img_url(still_path.as_str().unwrap()),
                            Some("thumb".to_string()),
                            None,
                            None,
                            None,
                        );
                    }

                    if let Some(air_date) = data.get("air_date") {
                        self.aired = Some(air_date.as_str().unwrap().to_string());
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
    <episodedetails>
      <plot>立花响和雪音克利斯保护的列车遭遇了Noise军队的袭击，被夺走了所罗门之杖和威尔博士，而玛利亚和风鸣翼开始了盛大的演唱会，途中玛利亚突然召唤了大量Noise军队进入会场，通过直播代表名为菲尼的组织向全世界开战。</plot>
      <lockdata>false</lockdata>
      <dateadded>2022-12-05 20:54:04</dateadded>
      <title>Gungnir的少女</title>
      <originaltitle>ガングニールの少女</originaltitle>
      <writer>Akifumi Kaneko</writer>
      <credits>Akifumi Kaneko</credits>
      <year>2013</year>
      <imdbid>tt2859274</imdbid>
      <tmdbid>1168864</tmdbid>
      <runtime>24</runtime>
      <studio>Tokyo MX</studio>
      <studio>Encourage Films</studio>
      <art>
        <poster>/tv/Chikage's/战姬绝唱 (2012)/Season 2/战姬绝唱 - S02E01 - Gungnir的少女-thumb.jpg</poster>
      </art>
      <actor>
        <name>Aoi Yuki</name>
        <role>Hibiki Tachibana (voice)</role>
        <type>Actor</type>
        <thumb>/config/data/metadata/People/A/Aoi Yuki/folder.jpg</thumb>
      </actor>
      <actor>
        <name>Yuka Iguchi</name>
        <role>Miku Kohinata (voice)</role>
        <type>Actor</type>
        <thumb>/config/data/metadata/People/Y/Yuka Iguchi/folder.jpg</thumb>
      </actor>
      <actor>
        <name>Nana Mizuki</name>
        <role>Tsubasa Kazanari (voice)</role>
        <type>Actor</type>
        <thumb>/config/data/metadata/People/N/Nana Mizuki/folder.jpg</thumb>
      </actor>
      <actor>
        <name>Ayahi Takagaki</name>
        <role>Chris Yukine (voice)</role>
        <type>Actor</type>
        <thumb>/config/data/metadata/People/A/Ayahi Takagaki/folder.jpg</thumb>
      </actor>
      <actor>
        <name>Yoko Hikasa</name>
        <role>Maria Cadenzavna Eve (voice)</role>
        <type>Actor</type>
        <thumb>/config/data/metadata/People/Y/Yoko Hikasa/folder.jpg</thumb>
      </actor>
      <actor>
        <name>Ai Kayano</name>
        <role>Kirika Akatsuki (voice)</role>
        <type>Actor</type>
        <thumb>/config/data/metadata/People/A/Ai Kayano/folder.jpg</thumb>
      </actor>
      <actor>
        <name>Yoshino Nanjo</name>
        <role>Shirabe Tsukoyomi (voice)</role>
        <type>Actor</type>
        <thumb>/config/data/metadata/People/Y/Yoshino Nanjo/folder.jpg</thumb>
      </actor>
      <actor>
        <name>Hideo Ishikawa</name>
        <role>Genjuro Kazanari (voice)</role>
        <type>Actor</type>
        <thumb>/config/data/metadata/People/H/Hideo Ishikawa/folder.jpg</thumb>
      </actor>
      <actor>
        <name>Kenji Akabane</name>
        <role>Fujitaka Sakuya (voice)</role>
        <type>Actor</type>
        <thumb>/config/data/metadata/People/K/Kenji Akabane/folder.jpg</thumb>
      </actor>
      <actor>
        <name>Asami Seto</name>
        <role>Tomosato Aoi (voice)</role>
        <type>Actor</type>
        <thumb>/config/data/metadata/People/A/Asami Seto/folder.jpg</thumb>
      </actor>
      <actor>
        <name>Soichiro Hoshi</name>
        <role>Ogawa Shinji (voice)</role>
        <type>Actor</type>
        <thumb>/config/data/metadata/People/S/Soichiro Hoshi/folder.jpg</thumb>
      </actor>
      <actor>
        <name>Misaki Kuno</name>
        <role>Elfnein (voice)</role>
        <type>Actor</type>
        <thumb>/config/data/metadata/People/M/Misaki Kuno/folder.jpg</thumb>
      </actor>
      <actor>
        <name>Chinatsu Akasaki</name>
        <role>Itaba Yumi (voice)</role>
        <type>Actor</type>
        <thumb>/config/data/metadata/People/C/Chinatsu Akasaki/folder.jpg</thumb>
      </actor>
      <actor>
        <name>Yui Horie</name>
        <role>Serena Cadenzavna Eve</role>
        <type>Actor</type>
        <thumb>/config/data/metadata/People/Y/Yui Horie/folder.jpg</thumb>
      </actor>
      <showtitle>战姬绝唱</showtitle>
      <episode>1</episode>
      <season>2</season>
      <aired>2013-07-04</aired>
      <fileinfo>
        <streamdetails>
          <video>
            <codec>hevc</codec>
            <micodec>hevc</micodec>
            <bitrate>7055981</bitrate>
            <width>1920</width>
            <height>1080</height>
            <aspect>16:9</aspect>
            <aspectratio>16:9</aspectratio>
            <framerate>23.976025</framerate>
            <scantype>progressive</scantype>
            <default>True</default>
            <forced>False</forced>
            <duration>23</duration>
            <durationinseconds>1421</durationinseconds>
          </video>
          <audio>
            <codec>flac</codec>
            <micodec>flac</micodec>
            <bitrate>1603542</bitrate>
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
      <uniqueid default="false" type="tmdb">1168864</uniqueid>
      <uniqueid default="true" type="imdb">tt2859274</uniqueid>
      <thumb>https://image.tmdb.org/t/p/original/7hXVJqEXhD5ux007qJmUExn88Ru.jpg</thumb>
      <epbookmark />
      <ratings></ratings>
      <code />
      <source>UNKNOWN</source>
      <original_filename>1.mkv</original_filename>
      <user_note />
    </episodedetails>"#;
    #[test]
    fn test_get_episode_info() {
        let data: Episode = quick_xml::de::from_str(NFO).unwrap();
        assert!(data.get_id(Provider::Known(ProviderKnown::TMDB)) == Some(&"1168864".to_string()));
    }

    #[test]
    fn test_update() {
        use tauri::async_runtime::block_on;
        let mut data = Episode::new("63322", Provider::Known(ProviderKnown::TMDB));
        block_on(data.update("zh-CN", 1, 1));
        assert!(data.aired == Some("2012-01-06".to_string()));
    }
}
