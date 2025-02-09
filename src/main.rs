use format_serde_error::SerdeError;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

use clap::Parser as ArgParser;
use lexicon::{AtpNull, AtpObject, AtpString, AtpTypes, AtpUnknown, Lexicon, StringFormats};

use tree_sitter::{InputEdit, Language, Node, Parser, Point, TreeCursor};

#[derive(clap::Parser, Debug)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();

    let src = fs::read_to_string(&args.path).unwrap();

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_ana::LANGUAGE.into())
        .expect("Error loading ana grammar");

    let mut tree = parser.parse(&src, None).unwrap();

    // program -> scope
    let mut program = tree.root_node().child(0).unwrap();
    println!("tree: {}", program.to_sexp());

    let nsid = program.child_by_field_name("id").unwrap();
    let nsid = &src[nsid.byte_range()];
    println!("nsid: {}", nsid);

    let mut res = Lexicon {
        lexicon: 1,
        id: format!("{nsid}"),
        revision: None,
        description: None,
        defs: HashMap::new(),
    };

    let mut defs = program.child(1).unwrap();

    let mut cursor = defs.walk();
    let scopes = defs.children(&mut cursor);

    for scope in scopes {
        match scope.kind() {
            "record" => (),
            "scope" => parse_scope(&mut res.defs, &src, scope),
            "{" | "}" => continue,
            _ => println!("unexpected node {}", &src[scope.byte_range()]),
        }
    }

    println!("res: {:#?}", res);

    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}

fn parse_scope(kv: &mut HashMap<String, AtpTypes>, src: &String, scope: Node) {
    let mut cursor = scope.walk();
    let id = scope.child_by_field_name("id").unwrap();
    let id = &src[id.byte_range()];
    println!("{id}");

    let mut required: Vec<String> = vec![];
    let mut properties: HashMap<String, AtpTypes> = HashMap::new();

    println!("{}", scope.child(1).unwrap().to_sexp());

    for param in scope.child(1).unwrap().children(&mut cursor) {
        match param.kind() {
            "param" => {
                let name = match param.child(0).unwrap().kind() {
                    "optional" => {
                        println!("param: {}", param.to_sexp());
                        (&src[param.child(0).unwrap().byte_range()])
                            .split_once('?')
                            .unwrap()
                            .0
                    }
                    _ => {
                        let name = &src[param.child(0).unwrap().byte_range()];
                        required.push(name.to_string());
                        dbg!(&required);
                        name
                    }
                };
                println!("{}", &src[param.child(1).unwrap().byte_range()]);
                properties.insert(
                    name.to_string(),
                    match &src[param.child(2).unwrap().byte_range()] {
                        "String" => AtpTypes::String(AtpString {
                            description: None,
                            format: None,
                            max_length: None,
                            min_length: None,
                            max_graphemes: None,
                            min_graphemes: None,
                            known_values: None,
                            enumeration: None,
                            default: None,
                            constant: None,
                        }),
                        "Uri" => AtpTypes::String(AtpString {
                            description: None,
                            format: Some(StringFormats::Uri),
                            max_length: None,
                            min_length: None,
                            max_graphemes: None,
                            min_graphemes: None,
                            known_values: None,
                            enumeration: None,
                            default: None,
                            constant: None,
                        }),
                        _ => AtpTypes::Unknown(AtpUnknown {
                            description: Some(String::from("failed to parse field")),
                        }),
                    },
                );
            }
            _ => continue,
        }
        let key = &src[param.child(0).unwrap().byte_range()];
    }

    let mut res = AtpObject {
        description: None,
        properties: properties,
        required: Some(required),
        nullable: None,
    };

    kv.insert(id.to_string(), AtpTypes::Object(res));

    println!("object: {}", &src[scope.byte_range()]);
}

fn parse_param() {}

fn _old_main() {
    let args = Args::parse();

    let src = fs::read_to_string(&args.path).unwrap();
    let a: Result<Lexicon, SerdeError> =
        serde_json::from_str(&src).map_err(|err| SerdeError::new(src, err));

    let a = dbg!(a);

    match a {
        Ok(a) => println!("{a:#?}"),
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
