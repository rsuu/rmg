use crate::utils::types::{Names, SelfResult};
use miniserde::{json, Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

pub trait TransTag
where
    Self: Sized,
{
    fn to_string(&self) -> String;
    fn from_str(&self, data: &str) -> SelfResult<Self>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagArtist {
    name: Names,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagCharacter {
    name: Names,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagCosplayer {
    name: Names,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagGroup {
    name: Names,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagFemale {
    name: Names,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagMale {
    name: Names,
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum TagReclass {
    Doujinshi = 1_u8,
    Manga = 2,

    Imageset = 3,
    Artistcg = 4,
    Gamecg = 5,

    Western = 6,
    Asian = 7,

    Misc = 8,
}

// https://zh.wikipedia.org/zh-hant/電影分級制度#現行制度
#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum TagLevel {
    L0 = 1_u8,
    L6 = 2,
    L12 = 3,
    L15 = 4,
    L18 = 5,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum TagLanguage {
    TagLanguage,
    TagLanguageOther,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum TagLanguageOther {
    Speechless,  // 无言
    TextCleaned, // 文字清除
    Translated,  // 翻译
    Rewrite,     // 重写
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum TagLanguageId {
    Afrikaans,  // 南非语
    Albanian,   // 阿尔巴尼亚语
    Arabic,     // 阿拉伯语
    Aramaic,    // 阿拉姆语
    Armenian,   // 亚美尼亚语
    Bengali,    // 孟加拉语
    Bosnian,    // 波斯尼亚语
    Bulgarian,  // 保加利亚语
    Burmese,    // 缅甸语
    Catalan,    // 加泰罗尼亚语
    Cebuano,    // 宿务语
    Chinese,    // 汉语
    Cree,       // 克里语
    Creole,     // 克里奥尔语
    Croatian,   // 克罗地亚语
    Czech,      // 捷克语
    Danish,     // 丹麦语
    Dutch,      // 荷兰语
    English,    // 英语
    Esperanto,  // 世界语
    Estonian,   // 爱沙尼亚语
    Finnish,    // 芬兰语
    French,     // 法语
    Georgian,   // 格鲁吉亚语
    German,     // 德语
    Greek,      // 希腊语
    Gujarati,   // 古吉拉特语
    Hebrew,     // 希伯来语
    Hindi,      // 印地语
    Hmong,      // 苗语
    Hungarian,  // 匈牙利语
    Icelandic,  // 冰岛语
    Indonesian, // 印尼语
    Irish,      // 爱尔兰语
    Italian,    // 意大利语
    Japanese,   // 日语
    Javanese,   // 爪哇語
    Kannada,    // 卡纳达语
    Kazakh,     // 哈萨克语
    Khmer,      // 高棉语
    Korean,     // 韩语
    Kurdish,    // 库尔德语
    Ladino,     // 犹太西班牙语
    Lao,        // 老挝语
    Latin,      // 拉丁语
    Latvian,    // 拉脱维亚语
    Marathi,    // 马拉地语
    Mongolian,  // 蒙古语
    Ndebele,    // 恩德贝莱语
    Nepali,     // 尼泊尔语
    Norwegian,  // 挪威语
    Oromo,      // 奥罗莫语
    Papiamento, // 帕皮阿门托语
    Pashto,     // 普什图语
    Persian,    // 波斯语
    Polish,     // 波兰语
    Portuguese, // 葡萄牙语
    Punjabi,    // 旁遮普语
    Romanian,   // 罗马尼亚语
    Russian,    // 俄语
    Sango,      // 桑戈语
    Sanskrit,   // 梵语
    Serbian,    // 塞尔维亚语
    Shona,      // 绍纳语
    Slovak,     // 斯洛伐克语
    Slovenian,  // 斯洛文尼亚语
    Somali,     // 索马里语
    Spanish,    // 西班牙语
    Swahili,    // 斯瓦希里语
    Swedish,    // 瑞典语
    Tagalog,    // 他加禄语
    Tamil,      // 泰米尔语
    Telugu,     // 泰卢固语
    Thai,       // 泰语
    Tibetan,    // 藏语
    Tigrinya,   // 提格雷尼亚语
    Turkish,    // 土耳其语
    Ukrainian,  // 乌克兰语
    Urdu,       // 乌尔都语
    Vietnamese, // 越南语
    Welsh,      // 威尔士语
    Yiddish,    // 意第绪语
    Zulu,       // 祖鲁语
}

macro_rules! M_TransTag {
     ($($t:ty,)*) => ($(
         impl TransTag for $t where Self: Sized {
             fn to_string(&self) -> String {
                 json::to_string(self)
             }

             fn from_str(&self, data: &str) -> SelfResult<Self> {
                 Ok(json::from_str(data)?)
             }
         }
     )*)
 }

M_TransTag! {
    Names,
    TagArtist,
    TagCharacter,
    TagCosplayer,
    TagFemale,
    TagGroup,
    TagLanguage,
    TagLanguageId,
    TagLevel,
    TagMale,
    TagReclass,
}

impl TryFrom<u8> for TagReclass {
    type Error = ();

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        use self::TagReclass::*;

        let res = if n == Doujinshi as u8 {
            Doujinshi
        } else if n == Manga as u8 {
            Manga
        } else if n == Imageset as u8 {
            Imageset
        } else if n == Artistcg as u8 {
            Artistcg
        } else if n == Gamecg as u8 {
            Gamecg
        } else if n == Western as u8 {
            Western
        } else if n == Asian as u8 {
            Asian
        } else if n == Misc as u8 {
            Misc
        } else {
            return Err(());
        };

        Ok(res)
    }
}

impl TryFrom<TagReclass> for u8 {
    type Error = ();

    fn try_from(n: TagReclass) -> Result<Self, Self::Error> {
        use self::TagReclass::*;

        let res = match n {
            Doujinshi => Doujinshi as u8,
            Manga => Manga as u8,
            Imageset => Imageset as u8,
            Artistcg => Artistcg as u8,
            Gamecg => Gamecg as u8,
            Western => Western as u8,
            Asian => Asian as u8,
            Misc => Misc as u8,

            _ => {
                return Err(());
            }
        };

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn te2() {
        let v: Names = vec![TagLevel::L0.to_string(), TagReclass::Misc.to_string()];

        debug_assert_eq!(v.to_string(), r#"["\"L0\"","\"Misc\""]"#);
    }

    #[test]
    fn te1() {
        let k = TagReclass::Doujinshi;

        match u8::try_from(k) {
            _ => {}
        }

        debug_assert_eq!(u8::try_from(k).unwrap(), 1_u8);
    }
}

// macro_rules! back_to_enum {
//     ($(#[$meta:meta])* $vis:vis enum $name:ident {
//         $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
//     }) => {
//         $(#[$meta])*
//         $vis enum $name {
//             $($(#[$vmeta])* $vname $(= $val)?,)*
//         }
//
//         impl std::convert::TryFrom<i32> for $name {
//             type Error = ();
//
//             fn try_from(v: i32) -> Result<Self, Self::Error> {
//                 match v {
//                     $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
//                     _ => Err(()),
//                 }
//             }
//         }
//     }
// }
//
// back_to_enum! {
//     enum MyEnum {
//         A = 1,
//         B,
//         C,
//     }
// }

// REF
// https://stackoverflow.com/questions/28028854/how-do-i-match-enum-values-with-an-integer
