use serde::{Deserialize, Serialize};
use serde_with::{rust::deserialize_ignore_any, skip_serializing_none};

use super::public::*;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
struct Tvshow {
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
    Season(ValueString),
    Episode(ValueString),
    Plot(ValueString),
    Tagline(ValueString),
    Runtime(ValueString),
    Thumb(Thumb),
    Fanart(Fanart),
    Mpaa(ValueString),
    Playcount(ValueString),
    Lastplayed(ValueString),
    Episodeguide(ValueString),
    Uniqueid(Uniqueid),
    Genre(ValueString),
    Tag(ValueString),
    Premiered(ValueString),
    Status(ValueString),
    Studio(ValueString),
    Trailer(ValueString),
    Actor(Actor),
    Namedseason(Namedseason),
    Dateadded(ValueString),
    #[serde(other, deserialize_with = "deserialize_ignore_any")]
    Other,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Namedseason {
    number: String,
    #[serde(rename = "$value", default)]
    value: String,
}

impl Tvshow {
    pub fn new(id: &str) -> Self {
        Self {
            items: vec![Items::Uniqueid(Uniqueid {
                r#type: Some("tmdb".to_string()),
                default: Some(true),
                value: id.to_string(),
            })],
        }
    }

    fn get_id(&self) -> Option<&String> {
        self.items.iter().find_map(|i| {
            if let Items::Uniqueid(j) = i {
                if let Some(h) = &j.r#type {
                    if h == "tmdb" {
                        return Some(&j.value);
                    }
                }
            }
            None
        })
    }

    pub async fn update(&mut self, lang: &str) {
        use crate::http::tmdb::*;
        use serde_json::Value;
        if let Some(id) = self.get_id() {
            let json = get_tvshow_info(id, lang).await;
            let data: Value = serde_json::from_str(&json).unwrap();
            println!("{:#?},{}", data, json)
        }
    }

    pub fn read_from_file() -> Tvshow {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_tvshow_info() {
        let document = r#"
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
            <rating default="false" max="10" name="themoviedb">
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
          <uniqueid default="false" type="imdb">tt15765670</uniqueid>
          <uniqueid default="true" type="tvdb">401233</uniqueid>
          <user_note />
        </tvshow>"#;
        let mut plate_appearance: Tvshow = serde_xml_rs::from_str(document).unwrap();
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

        println!("{:#?}", plate_appearance.get_id().unwrap());
    }

    #[test]
    fn test_update() {
        use tauri::async_runtime::block_on;
        block_on(Tvshow::new("123249").update("zh-CN"))
    }
}
