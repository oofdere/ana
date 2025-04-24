use std::collections::HashMap;

use tree_sitter::{Node, Range};
use types::{blob, boolean, integer, null, string};

use crate::{NodeHelpers, Param, Slice};

pub mod types;

#[derive(Debug)]
pub struct Prop {
    // e.g. foo: String
    pub name: String,
    pub value: PropKind,
    pub loc: Range,
}

#[derive(Debug, PartialEq)]
pub enum PropKind {
    // these are the concrete atproto types for the most part
    Blob(blob::Type),
    Boolean(boolean::Type),
    String(string::Type),
    Integer(integer::Type),
    Null(null::Type),
}

impl From<GenericProp> for PropKind {
    fn from(value: GenericProp) -> Self {
        match value.name.to_lowercase().as_str() {
            "blob" => PropKind::Blob(blob::Type::from(value)),
            "boolean" => PropKind::Boolean(boolean::Type::from(value)),
            "string" => PropKind::String(string::Type::from(value)),
            "integer" => PropKind::Integer(integer::Type::from(value)),
            "null" => PropKind::Null(null::Type::from(value)),
            _ => panic!("unknown type {}", value.name),
        }
    }
}

impl Prop {
    pub fn from(src: &str, node: &Node) -> Result<Prop, ()> {
        // TODO: handle optional properties
        match node.kind() {
            "property" => {
                let name = node.named_child(0).unwrap().str(&src);

                let value =
                    PropKind::from(GenericProp::from(&src, &node.named_child(1).unwrap()).unwrap());

                Ok(Prop {
                    name,
                    value,
                    loc: node.range(),
                })
            }
            _ => Err(()),
        }
    }
}

pub fn parse_properties(src: &str, node: &Node) -> HashMap<String, Prop> {
    let mut cursor = node.walk();
    let params = node
        .named_children(&mut cursor)
        //.inspect(|x| panic!("{:?}", x.to_sexp()))
        .map(|x| Prop::from(&src, &x).unwrap())
        .map(|x| (x.name.clone(), x))
        .collect();

    params
}

#[derive(Debug)]
pub struct GenericProp {
    pub name: String,
    pub params: HashMap<Box<str>, Param>,
    pub slice: Slice,
    pub loc: Range,
}

impl GenericProp {
    pub fn from(src: &str, node: &Node) -> Result<GenericProp, ()> {
        // this function extracts generic type info from a type node
        // generic types are not part of the IR and should be converted to a specific type

        match node.kind() {
            "type" => {
                let mut cursor = node.walk();
                let name = node.named_child(0).unwrap().str(&src);
                let params = node
                    .children_by_field_name("param", &mut cursor)
                    .map(|x| Param::from(&src, &x).unwrap())
                    .map(|x| (x.name.clone().into_boxed_str(), x));
                let slice = Slice::from(&src, &node);

                Ok(GenericProp {
                    name,
                    params: params.collect(),
                    slice,
                    loc: node.range(),
                })
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use tree_sitter::{Parser, Tree};

    use super::*;

    fn unwrap_harness(tree: &Tree) -> Node {
        tree.root_node().child(1).unwrap()
    }

    fn parse(src: &str) -> Tree {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_ana::LANGUAGE.into())
            .expect("error loading ana grammar");
        parser.parse(src, None).unwrap()
    }

    #[test]
    fn prop_from_test() {
        let src = "@@[ foo: String ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let prop = Prop::from(src, &node).unwrap();
        assert!(prop.name == "foo");
        if let PropKind::String(_) = prop.value {
            // Prop is a string type :)
        } else {
            panic!("Expected string type");
        }
        assert!(prop.loc.start_byte == 4);
        assert!(prop.loc.end_byte == 15);
    }

    #[test]
    fn prop_from_test_with_params() {
        let src = "@@[ foo: String(len=42..69, graphemes=2..4, format=\"did\", default=\"this is not a valid did lol\", ) ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let prop = Prop::from(src, &node).unwrap();
        assert!(prop.name == "foo");
        if let PropKind::String(s) = prop.value {
            assert!(s.length.start == Some(42));
            assert!(s.length.end == Some(69));
            assert!(s.graphemes.end == Some(4));
            assert!(s.graphemes.start == Some(2));
            assert!(s.graphemes.end == Some(4));
            assert!(s.format == Some(lexicon::StringFormats::Did));
            // assert!(s.default == Some("this is not a valid did lol".to_string())); default not parsed yet
        } else {
            panic!("Expected string type");
        }
    }
}
