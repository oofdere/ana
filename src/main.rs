use ariadne::{Color, Config, Label, Report, ReportKind, Source};
use serde_json::Error;

#[macro_use]
mod macros;

pub mod types;

fn main() {
    const SOURCE: &str = "a b c d e f";
    // also supports labels with no messages to only emphasis on some areas
    Report::build(ReportKind::Error, 2..3)
        .with_message("Incompatible types")
        .with_config(Config::default().with_compact(true))
        .with_label(Label::new(0..1).with_color(Color::Red))
        .with_label(
            Label::new(2..3)
                .with_color(Color::Blue)
                .with_message("`b` for banana")
                .with_order(1),
        )
        .with_label(Label::new(4..5).with_color(Color::Green))
        .with_label(
            Label::new(7..9)
                .with_color(Color::Cyan)
                .with_message("`e` for emerald"),
        )
        .finish()
        .print(Source::from(SOURCE))
        .unwrap();

    let src = r###"{"type": "boolean",}"###;
    let a: Result<types::AtpTypes, Error> = serde_json::from_str(src);

    match a {
        Ok(b) => println!("{b:?}"),
        Err(e) => {
            Report::build(ReportKind::Error, 1..1)
                .with_code(0)
                .with_message(format!("{:?} Error", e.classify()))
                .with_label(Label::new(e.column() - 2..e.column() - 1).with_color(Color::Red).with_message(format!("here")))
                .with_note(e.to_string())
                .finish()
                .print(Source::from(src))
                .unwrap();

                dbg!(e);

        }
    }
}
