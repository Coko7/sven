use anyhow::{bail, Result};
use clap::ValueEnum;
use log::{debug, info};
use quick_xml::de::from_reader;
use reqwest::{blocking::Client, header::USER_AGENT, Url};
use std::{
    fmt::Display,
    fs::{self, File},
    io::{BufReader, Write},
    path::PathBuf,
};

use crate::lexicon::Dictionary;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Lexicon {
    #[value(name = "english", alias = "en", alias = "english-to-swedish")]
    EnglishToSwedish,
    #[value(name = "swedish", alias = "sv", alias = "swedish-to-english")]
    SwedishToEnglish,
}

impl Display for Lexicon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayed = match self {
            Lexicon::EnglishToSwedish => "English to Swedish",
            Lexicon::SwedishToEnglish => "Swedish to English",
        };
        write!(f, "{}", displayed)
    }
}

impl Lexicon {
    pub fn name_id(&self) -> &str {
        match self {
            Lexicon::EnglishToSwedish => "folkets_en_sv_public",
            Lexicon::SwedishToEnglish => "folkets_sv_en_public",
        }
    }

    pub fn file_url(&self) -> Result<Url> {
        let base_url = Url::parse(FOLKETS_LEXICON_BASE_URL)?;
        Ok(base_url.join(&self.xml_file_name())?)
    }

    pub fn xml_file_path(&self) -> Result<PathBuf> {
        let data_dir = PathBuf::from(CACHE_PATH);
        Ok(data_dir.join(self.xml_file_name()))
    }

    pub fn json_file_path(&self) -> Result<PathBuf> {
        let data_dir = PathBuf::from(CACHE_PATH);
        Ok(data_dir.join(self.json_file_name()))
    }

    fn xml_file_name(&self) -> String {
        format!("{}.xml", self.name_id())
    }

    fn json_file_name(&self) -> String {
        format!("{}.json", self.name_id())
    }
}

const FOLKETS_LEXICON_BASE_URL: &str = "https://folkets-lexikon.csc.kth.se/folkets/";
const CACHE_PATH: &str = "/home/coco/Workspace/personal/Programming/rust/clis/sven/lexicons";
// const CACHE_PATH: &str = "/home/coco/.config/local/share/sven";

pub fn prepare_lexicons() -> Result<()> {
    let lexicons = [Lexicon::EnglishToSwedish, Lexicon::SwedishToEnglish];

    if lexicons
        .iter()
        .any(|lex| !lex.json_file_path().unwrap().exists())
    {
        info!("preparing lexicon data, this will only happen once...");
        println!("preparing lexicon data, this will only happen once...",);

        for lexicon in lexicons {
            if !lexicon.json_file_path()?.exists() {
                fetch_and_convert_lexicon(&lexicon)?;
            }
        }

        println!("preparations done.",);
    }

    Ok(())
}

fn fetch_and_convert_lexicon(lexicon: &Lexicon) -> Result<()> {
    println!("downloading xml data for lexicon: {}", lexicon);
    download_folkets_lexicon(lexicon)?;
    println!("download finished, starting JSON conversion...");
    convert_lexicon_to_json(lexicon)
}

fn download_folkets_lexicon(lexicon: &Lexicon) -> Result<()> {
    let save_path = lexicon.xml_file_path()?;
    fs::create_dir_all(CACHE_PATH)?;

    let url = lexicon.file_url()?;
    debug!("downloading lexicon from: {url}");
    let client = Client::builder().user_agent(USER_AGENT).build()?;
    let mut response = client.get(url.to_string()).send()?;

    if !response.status().is_success() {
        bail!("failed to download file: http {}", response.status());
    }

    let mut file = File::create(&save_path)?;
    std::io::copy(&mut response, &mut file)?;

    debug!("xml file downloaded and saved to {}", save_path.display());
    Ok(())
}

fn convert_lexicon_to_json(lexicon: &Lexicon) -> Result<()> {
    let xml_path = lexicon.xml_file_path()?;
    let file = File::open(xml_path)?;
    let reader = BufReader::new(file);

    let root: Dictionary = from_reader(reader)?;
    debug!("deserialized xml lexicon into a struct");
    let json = serde_json::to_string(&root)?;
    debug!("serialized lexicon struct into json");

    let json_path = lexicon.json_file_path()?;
    let mut file = File::create(&json_path)?;

    file.write_all(json.as_bytes())?;
    debug!(
        "wrote serialized lexicon as json to: {}",
        json_path.display()
    );
    Ok(())
}

// fn find_word_by_value(target: &str) -> Result<Option<String>> {
//     let xml_file_path =
//         Path::new("/home/coco/Workspace/personal/Programming/rust/clis/sven/folkets.xml");
//     let file = File::open(xml_file_path)?;
//     debug!("file loaded");
//     let file = BufReader::new(file);
//     let mut reader = Reader::from_reader(file);
//
//     let mut buf = Vec::new();
//
//     loop {
//         match reader.read_event_into(&mut buf) {
//             Ok(Event::Start(ref e)) if e.name() == quick_xml::name::QName(b"word") => {
//                 // Check attributes of <word>
//                 for attr in e.attributes() {
//                     if let Ok(attr) = attr {
//                         if attr.key == quick_xml::name::QName(b"value") {
//                             if let Ok(val) = attr.unescape_value() {
//                                 if val == target {
//                                     return Ok(Some(val.to_string()));
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             Ok(Event::Eof) => break,
//             Err(e) => {
//                 println!("Error: {:?}", e);
//                 break;
//             }
//             _ => {}
//         }
//     }
//     Ok(None)
// }
