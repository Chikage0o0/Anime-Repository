use super::public::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
    top250: Option<i16>,
    outline: Option<String>,
    plot: Option<String>,
    tagline: Option<String>,
    runtime: Option<String>,
    #[serde(default)]
    thumb: Vec<Thumb>,
    fanart: Option<Fanart>,
    mpaa: Option<String>,
    playcount: Option<i8>,
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

impl Default for Movie {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            original_title: None,
            ratings: None,
            user_rating: None,
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
            studio: Vec::new(),
            actor: Vec::new(),
            showlink: None,
            resume: None,
            date_added: Some(get_date()),
            sort_title: None,
            top250: None,
            outline: None,
            fanart: None,
            mpaa: None,
            tag: Vec::new(),
            set: Vec::new(),
            country: Vec::new(),
            trailer: None,
        }
    }
}

impl Movie {
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

    pub fn read_from_file() -> Movie {
        todo!()
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
        let plate_appearance: Movie = quick_xml::de::from_str(NFO).unwrap();
        println!("{:#?}", &plate_appearance);
        let se = quick_xml::se::to_string(&plate_appearance).unwrap();
        println!("{}", &se);
        // let d: Vec<_> = plate_appearance
        //     .items
        //     .iter()
        //     .filter_map(|x| {
        //         if let Items::Tag(d) = x {
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
        //         d.value = "s".to_string()
        //     }
        // });
    }
}
