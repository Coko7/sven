use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "dictionary")]
pub struct Dictionary {
    #[serde(rename = "@comment")]
    pub comment: Option<String>,

    #[serde(rename = "@created")]
    pub created: Option<String>,

    #[serde(rename = "@last-changed")]
    pub last_changed: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@source-language")]
    pub source_language: Option<String>,

    #[serde(rename = "@target-language")]
    pub target_language: Option<String>,

    #[serde(rename = "@version")]
    pub version: Option<String>,

    #[serde(rename = "@license")]
    pub license: Option<String>,

    #[serde(rename = "@licenseComment")]
    pub license_comment: Option<String>,

    #[serde(rename = "@originURL")]
    pub origin_url: Option<String>,

    #[serde(rename = "word")]
    pub words: Vec<Word>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    #[serde(rename = "@class")]
    pub class: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,

    #[serde(rename = "@lang")]
    pub lang: Option<String>,

    #[serde(rename = "@value")]
    pub value: String,

    #[serde(rename = "translation", default)]
    pub translations: Vec<Translation>,

    #[serde(rename = "synonym", default)]
    pub synonyms: Vec<Synonym>,

    #[serde(rename = "phonetic", default)]
    pub phonetics: Vec<Phonetic>,

    #[serde(rename = "inflection", default)]
    pub inflections: Vec<Inflection>,

    #[serde(default)]
    pub sees: Vec<See>,

    #[serde(rename = "example", default)]
    pub examples: Vec<Example>,

    #[serde(rename = "related", default)]
    pub related_words: Vec<RelatedWord>,

    #[serde(rename = "definition", default)]
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedWord {
    #[serde(rename = "@type")]
    pub word_type: String,

    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Phonetic {
    #[serde(rename = "@soundFile")]
    pub sound_file: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct See {
    #[serde(rename = "@type")]
    pub see_type: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Example {
    #[serde(rename = "@value")]
    pub value: Option<String>,

    #[serde(rename = "translation")]
    pub translation: Option<Translation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Definition {
    #[serde(rename = "@value")]
    pub value: Option<String>,

    #[serde(rename = "translation")]
    pub translation: Option<Translation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Translation {
    #[serde(rename = "@comment")]
    pub comment: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Synonym {
    #[serde(rename = "@level")]
    pub level: Option<String>,

    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Inflection {
    #[serde(rename = "@value")]
    pub value: String,
}
