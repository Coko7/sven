use std::ffi::OsString;

use clap::{Parser, Subcommand};

use crate::common::Lexicon;

#[derive(Debug, Parser)]
#[command(name = "sven")]
#[command(
    about = "Search the Folkets Lexikon XML file for words containing a given string.",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Source language to use
    #[arg(short, long, alias = "lang", value_name = "LANGUAGE", default_value_t = Lexicon::EnglishToSwedish, value_enum)]
    pub language: Lexicon,

    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(alias = "look")]
    Lookup {
        value: OsString,
    },
    Search {
        value: OsString,
    },
    Play,
}
