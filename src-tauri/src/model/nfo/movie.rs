use serde::{Deserialize, Serialize};
use serde_with::{rust::deserialize_ignore_any, skip_serializing_none};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Movie {
    #[serde(rename = "$value")]
    items: Vec<Items>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Items {
    Title(ValueString),
    Originaltitle(ValueString),
    Sorttitle(ValueString),
    Ratings(Ratings),
    Userrating(ValueString),
    Top250(ValueString),
    Outline(ValueString),
    Plot(ValueString),
    Tagline(ValueString),
    Runtime(ValueString),
    Thumb(Thumb),
    Fanart(Fanart),
    Mpaa(ValueString),
    Playcount(ValueString),
    Lastplayed(ValueString),
    Uniqueid(Uniqueid),
    Genre(ValueString),
    Tag(ValueString),
    Country(ValueString),
    Credits(ValueString),
    Director(ValueString),
    Premiered(ValueString),
    Studio(ValueString),
    Actor(Actor),
    Resume(Resume),
    Dateadded(ValueString),
    #[serde(other, deserialize_with = "deserialize_ignore_any")]
    Other,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ValueString {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Ratings {
    #[serde(rename = "$value")]
    value: Vec<Rating>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Rating {
    name: String,
    max: String,
    default: bool,
    #[serde(rename = "$value")]
    value: Vec<ValueRating>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum ValueRating {
    Value(ValueString),
    Votes(ValueString),
}
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Thumb {
    aspect: Option<String>,
    preview: Option<String>,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Fanart {
    #[serde(rename = "$value")]
    value: Vec<Thumb>,
}
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Uniqueid {
    r#type: Option<String>,
    default: Option<String>,
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Actor {
    #[serde(rename = "$value")]
    value: Vec<ValueActor>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum ValueActor {
    Name(ValueString),
    Role(ValueString),
    Order(ValueString),
    Thumb(ValueString),
    #[serde(other, deserialize_with = "deserialize_ignore_any")]
    Other,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Resume {
    #[serde(rename = "$value")]
    value: Vec<ValueResume>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum ValueResume {
    Position(ValueString),
    Total(ValueString),
    #[serde(other, deserialize_with = "deserialize_ignore_any")]
    Other,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_movie_info() {
        let document = r#"
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
          <set>Re：从零开始的异世界生活（系列）</set>
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
        let mut plate_appearance: Movie = serde_xml_rs::from_str(document).unwrap();
        let d: Vec<_> = plate_appearance
            .items
            .iter()
            .filter_map(|x| {
                if let Items::Tag(d) = x {
                    return Some(&d.value);
                }
                return None;
            })
            .collect();
        println!("{:#?}", d);

        let d1: Option<&String> = plate_appearance.items.iter().find_map(|x| {
            if let Items::Dateadded(d) = x {
                return Some(&d.value);
            }
            return None;
        });
        println!("{:#?}", d1.unwrap());
        //修改内部元素
        plate_appearance.items.iter_mut().for_each(|x| {
            if let Items::Dateadded(d) = x {
                d.value = "s".to_string()
            }
        });
    }
}
