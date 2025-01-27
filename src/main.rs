use std::{collections::HashMap, fs};

use ariadne::{Color, Config, Label, Report, ReportKind, Source};
use serde::{Serialize, Deserialize};
use serde_json::Error;
use format_serde_error::SerdeError;

#[macro_use]
mod macros;

pub mod types;

use clap::Parser;
use types::AtpTypes;

#[derive(Parser, Debug)]
struct Args {
    path: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
/// Lexicons are JSON files associated with a single NSID. A file contains one or more definitions, each with a distinct short name. A definition with the name `main` optionally describes the "primary" definition for the entire file. A Lexicon with zero definitions is invalid.
pub struct Lexicon {
    /// indicates Lexicon language version. In this version, a fixed value of `1`
    lexicon: i32,
    /// the NSID of the Lexicon
    id: String,
    /// indicates the version of this Lexicon, if changes have occurred
    revision: Option<i32>,
    /// short overview of the Lexicon, usually one or two sentences
    description: Option<String>,
    /// set of definitions, each with a distinct name (key)
    defs: HashMap<String, AtpTypes>
}

fn main() {
    let args = Args::parse();

    let src = fs::read_to_string(&args.path).unwrap();
    let a: Result<Lexicon, SerdeError> = serde_json::from_str(&src).map_err(|err| SerdeError::new(src, err));

    let a = dbg!(a);

    match a {
        Ok(a) => println!("{a:?}"),
        Err(err) => eprintln!("{err}"),
    };
    // match a {
    //     Ok(b) => println!("{b:?}"),
    //     Err(e) => {
    //         Report::build(ReportKind::Error, (args.path.as_str(), e.line()-1..e.line()+1))
    //             .with_message(format!("{} {}", e, e.line()))
    //             .with_label(
    //                 Label::new((args.path.as_str(), e.column()..e.column()+1)).with_message("here!").with_color(Color::Red)
    //             )
    //             .finish()
    //             .eprint((args.path.as_str(), Source::from(src)))
    //             .unwrap();

    //         dbg!(e);
    //     }
    //}
}
