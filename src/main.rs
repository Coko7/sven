use anyhow::{bail, Context, Result};
use clap::Parser;
use log::error;
use std::{
    ffi::OsString,
    fs::{self},
    process::ExitCode,
};

use crate::{
    cli::Cli,
    common::{prepare_lexicons, Lexicon},
    lexicon::{Dictionary, Word},
};

mod cli;
mod common;
mod lexicon;

fn main() -> ExitCode {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    match process_args(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            error!("{}", e);
            ExitCode::FAILURE
        }
    }
}

fn process_args(args: Cli) -> Result<()> {
    prepare_lexicons()?;

    let source_lang = args.language;

    match args.command {
        cli::Commands::Lookup { value } => print_translations(source_lang, value),
        cli::Commands::Search { value } => search_words(source_lang, value),
        cli::Commands::Play => todo!(),
    }
}

fn search_words(lexicon: Lexicon, search_value: OsString) -> Result<()> {
    let search_value = search_value
        .to_str()
        .context("invalid unicode string supplied")?;

    let dictionary = load_from_json(lexicon)?;
    let results = dictionary
        .words
        .iter()
        .filter(|word| word.value.contains(search_value))
        .collect::<Vec<_>>();
    for word in results.iter() {
        println!("{}", word.value);
    }
    Ok(())
}

fn load_from_json(lexicon: Lexicon) -> Result<Dictionary> {
    let json_path = lexicon.json_file_path()?;
    let json_str = fs::read_to_string(json_path)?;
    let root: Dictionary = serde_json::from_str(&json_str)?;
    Ok(root)
}

fn pretty_print_word_entry(word: &Word) -> Result<()> {
    // println!("{:#?}", word);
    // return Ok(());
    // println!("{:#?}", word);
    println!("Word: {}", word.value);
    if let Some(class) = &word.class {
        println!("Class: {}", class);
    }

    if let Some(comment) = &word.comment {
        println!("Comment: {}", comment);
    }

    if !word.inflections.is_empty() {
        let inflections = word
            .inflections
            .iter()
            .map(|inf| inf.value.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        println!("Inflections: {}", inflections);
    }

    if !word.translations.is_empty() {
        println!("Translations:");
        for translation in word.translations.iter() {
            let comment = translation.comment.clone().unwrap_or_default();
            println!("- {}", translation.value.clone().unwrap_or_default());
            if !comment.is_empty() {
                println!("Comment: {}", comment);
            }
        }
    }

    if !word.synonyms.is_empty() {
        println!("Synonyms:");
        for synonym in word.synonyms.iter() {
            // let level = synonym.level.clone().unwrap_or_default();
            // if !level.is_empty() {
            //     println!("Level: {}", level);
            // }
            println!("- {}", synonym.value);
        }
    }

    if !word.examples.is_empty() {
        println!("Examples:");
        for example in word.examples.iter() {
            let example_orig = example.value.clone().unwrap_or_default();
            if let Some(translation) = &example.translation {
                let example_trans = translation.value.clone().unwrap_or_default();
                println!("- {} ({})", example_orig, example_trans);
            } else {
                println!("- {}", example_orig);
            }
        }
    }

    Ok(())
}

fn print_translations(lexicon: Lexicon, src_word: OsString) -> Result<()> {
    let src_word = src_word
        .to_str()
        .context("invalid unicode string supplied")?;
    let dictionary = load_from_json(lexicon)?;

    for word in dictionary.words.iter() {
        if word.value == src_word || word.inflections.iter().any(|inf| inf.value == src_word) {
            return pretty_print_word_entry(word);
        }
    }

    bail!("no such word in lexicon: {src_word}")
}
