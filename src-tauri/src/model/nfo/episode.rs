use super::public::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "episodedetails")]
struct Episode {
    title: String,
    #[serde(rename = "originaltitle")]
    original_title: Option<String>,
    ratings: Option<Ratings>,
    #[serde(rename = "userrating")]
    user_rating: Option<String>,
    #[serde(rename = "displayepisode")]
    display_episode: Option<String>,
    #[serde(rename = "displayseason")]
    display_season: Option<String>,
    plot: Option<String>,
    tagline: Option<String>,
    runtime: Option<String>,
    #[serde(default)]
    thumb: Vec<Thumb>,
    playcount: Option<i8>,
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

impl Default for Episode {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            original_title: None,
            ratings: None,
            user_rating: None,
            display_episode: None,
            display_season: None,
            plot: None,
            tagline: None,
            runtime: None,
            thumb: Vec::new(),
            playcount: None,
            lastplayed: None,
            unique_id: Vec::new(),
            genre: Vec::new(),
            credits: Vec::new(),
            director: Vec::new(),
            premiered: None,
            aired: None,
            studio: Vec::new(),
            actor: Vec::new(),
            showlink: None,
            resume: None,
            date_added: Some(get_date()),
        }
    }
}

impl Episode {
    pub fn new(id: &str) -> Self {
        Self {
            unique_id: vec![Uniqueid {
                r#type: "tmdb".to_string(),
                default: true,
                value: id.to_string(),
            }],
            ..Default::default()
        }
    }

    fn get_id(&self) -> Option<&String> {
        self.unique_id.iter().find_map(|i| {
            if i.r#type == "tmdb".to_string() {
                Some(&i.value)
            } else {
                None
            }
        })
    }

    pub fn update(&mut self) {
        todo!()
    }

    pub fn read_from_file() -> Episode {
        todo!()
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
        let plate_appearance: Episode = quick_xml::de::from_str(NFO).unwrap();
        println!("{:#?}", &plate_appearance);
        let se = quick_xml::se::to_string(&plate_appearance).unwrap();
        println!("{}", &se);
        // let d: Vec<_> = plate_appearance
        //     .items
        //     .iter()
        //     .filter_map(|x| {
        //         if let Items::Plot(d) = x {
        //             return Some(&d.value);
        //         }
        //         return None;
        //     })
        //     .collect();
        // println!("{:#?}", d);

        // let d1: Option<&String> = plate_appearance.items.iter().find_map(|x| {
        //     if let Items::Dateadded(d) = x {
        //         return Some(&d.value);
        //     }
        //     return None;
        // });
        // println!("{:#?}", d1.unwrap());
        // //修改内部元素
        // plate_appearance.items.iter_mut().for_each(|x| {
        //     if let Items::Dateadded(d) = x {
        //         d.value = "s".to_string();
        //     }
        // });
    }
}
