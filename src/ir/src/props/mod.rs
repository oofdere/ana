use std::collections::HashMap;

use tree_sitter::{Node, Range};
use types::string;

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
    String(string::Type),
    Null, //Number(NumberType),
          //Boolean(BooleanType),
}

impl From<GenericProp> for PropKind {
    fn from(value: GenericProp) -> Self {
        PropKind::String(string::Type::from(value)) // handle all types here in a match
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

    use crate::props::GenericProp;

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
            // Prop is a string type
        } else {
            panic!("Expected string type");
        }
        assert!(prop.loc.start_byte == 4);
        assert!(prop.loc.end_byte == 15);
    }

    #[test]
    fn generic_type_from_test() {
        let src = "@@[ String(len=1..10, format=\"did\") ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericProp::from(src, &node).unwrap();
        assert!(generic_type.name == "String");
        assert!(generic_type.params.len() == 2);
        match generic_type.params.get("len").unwrap().value {
            crate::ParamKind::Slice(slice) => {
                assert!(slice.start == Some(1));
                assert!(slice.end == Some(10));
            }
            _ => panic!("Unexpected value type"),
        };
        assert!(
            generic_type.params.get("format").unwrap().value
                == crate::ParamKind::String("did".to_string())
        );
    }
}
