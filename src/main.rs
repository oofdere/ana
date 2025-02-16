use std::{collections::HashMap, fs};

use clap::Parser as ArgParser;
use lexicon::{
    AtpArray, AtpBlob, AtpBoolean, AtpBytes, AtpCidLink, AtpInteger, AtpNull, AtpObject, AtpParams,
    AtpProcedure, AtpQuery, AtpRecord, AtpRef, AtpString, AtpSubscription, AtpToken, AtpTypes,
    AtpUnion, AtpUnknown, Lexicon, StringFormats, atp_format,
};

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
    println!("{}", tree.root_node().to_sexp());

    let mut cursor = tree.walk();

    // only one namespace supported for now
    let mut namespace = tree
        .root_node()
        .child(0)
        .expect("you sure this is the right file?");
    println!("tree: {}", namespace.to_sexp());

    let nsid = namespace.child_by_field_name("name").unwrap();
    let nsid = &src[nsid.byte_range()];
    println!("nsid: {}", nsid);

    let mut res = Lexicon {
        lexicon: 1,
        id: nsid.to_string(),
        revision: None,
        description: None,
        defs: HashMap::new(),
    };

    let body = namespace.children_by_field_name("body", &mut cursor);

    for def in body {
        println!("a: {}", def.to_sexp());
        match def.kind() {
            "record" => panic!("found record!"),
            "object" => {
                let name = &src[def.child_by_field_name("name").unwrap().byte_range()];
                let mut object = AtpObject {
                    description: None,
                    properties: HashMap::new(),
                    required: None,
                    nullable: None,
                };

                let mut required: Vec<String> = Vec::new();

                let mut cursor = def.walk();
                let props = def
                    .child_by_field_name("body")
                    .unwrap()
                    .children(&mut cursor);
                for prop in props {
                    println!("{}", prop.to_sexp());
                    match prop.kind() {
                        "property" => println!("found property!"),
                        "optional" => {
                            println!("found optional prop!");
                            required.push(
                                (&src[prop.child_by_field_name("name").unwrap().byte_range()])
                                    .to_string(),
                            );
                        }
                        "ref" => todo!(),
                        _ => continue,
                    };

                    let name = &src[prop.child_by_field_name("name").unwrap().byte_range()];
                    let typen = prop.child_by_field_name("type").unwrap();
                    let mut atp_type = AtpTypes::Unknown(AtpUnknown::new());
                    match typen.kind() {
                        "ref" => todo!(),
                        "type" => {
                            let name =
                                &src[typen.child_by_field_name("name").unwrap().byte_range()];
                            atp_type = match name {
                                "Null" => AtpTypes::Null(AtpNull::new()),
                                "Boolean" => AtpTypes::Boolean(AtpBoolean::new()),
                                "Integer" => AtpTypes::Integer(AtpInteger::new()),
                                "String" => AtpTypes::String(AtpString::new()),
                                "Bytes" => AtpTypes::Bytes(AtpBytes::new()),
                                "CidLink" => AtpTypes::CidLink(AtpCidLink::new()),
                                "Blob" => AtpTypes::Blob(AtpBlob::new()),
                                "Array" => AtpTypes::Array(AtpArray::new()),
                                "Object" => AtpTypes::Object(AtpObject::new()),
                                "Params" => AtpTypes::Params(AtpParams::new()),
                                "Token" => AtpTypes::Token(AtpToken::new()),
                                "Ref" => AtpTypes::Ref(AtpRef::new()),
                                "Union" => AtpTypes::Union(AtpUnion::new()),
                                "Unknown" => AtpTypes::Unknown(AtpUnknown::new()),
                                "Record" => AtpTypes::Record(AtpRecord::new()),
                                "Query" => AtpTypes::Query(AtpQuery::new()),
                                "Procedure" => AtpTypes::Procedure(AtpProcedure::new()),
                                "Subscription" => AtpTypes::Subscription(AtpSubscription::new()),
                                "AtIdentifier" => atp_format(StringFormats::AtIdentifier),
                                "AtUri" => atp_format(StringFormats::AtUri),
                                "Cid" => atp_format(StringFormats::Cid),
                                "DateTime" => atp_format(StringFormats::Datetime),
                                "Did" => atp_format(StringFormats::Did),
                                "Handle" => atp_format(StringFormats::Handle),
                                "Nsid" => atp_format(StringFormats::Nsid),
                                "Tid" => atp_format(StringFormats::Tid),
                                "RecordKey" => atp_format(StringFormats::RecordKey),
                                "Uri" => atp_format(StringFormats::Uri),
                                "Language" => atp_format(StringFormats::Language),
                                _ => panic!("unknown type {name}"),
                            };
                        }
                        "array" => todo!(),
                        "union" => todo!(),
                        _ => panic!("unknown type {name}"),
                    }
                    object.properties.insert(name.to_string(), atp_type);
                }

                res.defs.insert(name.to_string(), AtpTypes::Object(object));
            }
            "get" => panic!("found get!"),
            _ => panic!("unexpected type"),
        }
    }

    println!("res: {:#?}", res);

    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}
