use crate::utils::types::{Names};
use speedy::{Readable, Writable};


#[derive(PartialEq, Eq, Debug, Clone, Readable, Writable)]
pub struct TagArtist {
    pub name: Names,
}

#[derive(PartialEq, Eq, Debug, Clone, Readable, Writable)]
pub struct TagCharacter {
    pub name: Names,
}

#[derive(PartialEq, Eq, Debug, Clone, Readable, Writable)]
pub struct TagCosplayer {
    pub name: Names,
}

#[derive(PartialEq, Eq, Debug, Clone, Readable, Writable)]
pub struct TagGroup {
    pub name: Names,
}

#[derive(PartialEq, Eq, Debug, Clone, Readable, Writable)]
pub struct TagParody {
    pub name: Names,
}

#[derive(PartialEq, Eq, Debug, Clone, Readable, Writable)]
pub struct TagMixed {
    pub name: Names,
}

#[derive(PartialEq, Eq, Debug, Clone, Readable, Writable)]
pub struct TagFemale {
    pub name: Names,
}

#[derive(PartialEq, Eq, Debug, Clone, Readable, Writable)]
pub struct TagMale {
    pub name: Names,
}

#[repr(u8)]
#[derive(PartialEq, Eq, Debug, Readable, Writable, Copy, Clone)]
pub enum TagReclass {
    Doujinshi,
    Manga,

    Imageset,
    Artistcg,
    Gamecg,

    Western,
    Asian,

    Misc,

    Level(TagReclassLevel),
}

// https://zh.wikipedia.org/zh-hant/電影分級制度#現行制度
#[repr(u8)]
#[derive(PartialEq, Eq, Debug, Readable, Writable, Copy, Clone)]
pub enum TagReclassLevel {
    L0,
    L6,
    L12,
    L15,
    L18,
}

#[derive(PartialEq, Eq, Debug, Readable, Writable, Copy, Clone)]
pub enum TagLanguage {
    TagLanguage,
    TagLanguageOther,
}

#[derive(PartialEq, Eq, Debug, Readable, Writable, Copy, Clone)]
pub enum TagLanguageOther {
    Speechless,  // 无言
    TextCleaned, // 文字清除
    Translated,  // 翻译
    Rewrite,     // 重写
}

#[derive(PartialEq, Eq, Debug, Readable, Writable, Copy, Clone)]
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

#[cfg(test)]
mod tests {
    
}

