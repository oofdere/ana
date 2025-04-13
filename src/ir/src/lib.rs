use lexicon::{AtpString, AtpTypes, StringFormats};
use std::collections::HashMap;
use tree_sitter::{Node, Range};

trait NodeHelpers {
    fn str(&self, src: &str) -> String;
}

impl NodeHelpers for Node<'_> {
    fn str(&self, src: &str) -> String {
        let start = self.start_byte();
        let end = self.end_byte();
        String::from(&src[start..end])
    }
}

pub fn extract_string(src: &str, node: &Node) -> Option<String> {
    match node.kind() {
        "string" => {
            let start = node.start_byte();
            let end = node.end_byte();
            Some(String::from(&src[start + 1..end - 1]))
        }
        _ => None,
    }
}

pub fn extract_integer(src: &str, node: &Node) -> Option<u32> {
    match node.kind() {
        "integer" => Some(node.str(&src).parse().unwrap()),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Slice {
    pub start: Option<u32>,
    pub end: Option<u32>,
    pub loc: Option<Range>,
}

impl Slice {
    fn from(src: &str, node: &Node) -> Slice {
        // TODO fix slice start and end bytes, take from min and max instead of node
        // slice can be an anonymous node so we don't match on it and everything explodes instead
        // this means any node can be passed in and you get a start and end range always
        let min = node
            .child_by_field_name("min")
            .and_then(|x| extract_integer(&src, &x));
        let max = node
            .child_by_field_name("max")
            .and_then(|x| extract_integer(&src, &x));

        Slice {
            start: min,
            end: max,
            loc: Some(node.range()),
        }
    }

    fn empty() -> Slice {
        Slice {
            start: None,
            end: None,
            loc: None,
        }
    }
}

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub value: ParamKind,
    pub loc: Range,
}

#[derive(Debug, PartialEq)]
pub enum ParamKind {
    String(String),
    Integer(u32),
    Slice(Slice),
}

impl Param {
    pub fn from(src: &str, node: &Node) -> Result<Param, ()> {
        match node.kind() {
            "param" => {
                let name = node.named_child(0).unwrap().str(&src);
                let value = node.named_child(1).unwrap();
                let kind = match value.kind() {
                    "string" => Ok(ParamKind::String(extract_string(&src, &value).unwrap())),
                    "integer" => Ok(ParamKind::Integer(extract_integer(&src, &value).unwrap())),
                    "slice" => Ok(ParamKind::Slice(Slice::from(&src, &value))),
                    _ => Err(()),
                };

                Ok(Param {
                    name,
                    value: kind?,
                    loc: node.range(),
                })
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct GenericType {
    pub name: String,
    pub params: HashMap<Box<str>, Param>,
    pub slice: Slice,
    pub loc: Range,
}

impl GenericType {
    pub fn from(src: &str, node: &Node) -> Result<GenericType, ()> {
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

                Ok(GenericType {
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

pub trait Type {
    /// convert from a GenericType to Self, returns an Option with None if there are any errors during conversion
    fn from_generic(t: GenericType) -> Self;
}

pub struct StringType {
    pub format: Option<StringFormats>,
    pub length: Slice,
    pub graphemes: Slice,
    //known_values: todo!(),
    //enumeration: todo!(),
    pub default: Option<String>,
    pub constant: Option<String>,
    pub loc: Range,
}

impl From<GenericType> for StringType {
    fn from(t: GenericType) -> Self {
        //let format = t.params.get("format");
        let length = t
            .params
            .get("len")
            .map_or(Slice::empty(), |x| match x.value {
                ParamKind::Slice(s) => s,
                _ => Slice::empty(),
            });
        let graphemes = t
            .params
            .get("graphemes")
            .map_or(Slice::empty(), |x| match x.value {
                ParamKind::Slice(s) => s,
                _ => Slice::empty(),
            });

        StringType {
            format: None,
            length,
            graphemes,
            default: None,
            constant: None,
            loc: t.loc,
        }
    }
}

impl Into<AtpString> for StringType {
    fn into(self) -> AtpString {
        AtpString {
            description: None,
            format: self.format,
            min_length: self.length.start,
            max_length: self.length.end,
            min_graphemes: self.graphemes.start,
            max_graphemes: self.graphemes.end,
            known_values: None,
            enumeration: None,
            default: self.default,
            constant: self.constant,
        }
    }
}

impl Into<AtpTypes> for StringType {
    fn into(self) -> AtpTypes {
        AtpTypes::String(self.into())
    }
}

#[derive(Debug)]
pub struct Prop {
    pub name: String,
    pub value: String,
    pub loc: Range,
}

impl Prop {
    pub fn from(src: &str, node: &Node) -> Result<Prop, ()> {
        // TODO: handle optional properties
        match node.kind() {
            "property" => {
                let name = node.named_child(0).unwrap().str(&src);
                let value = node.named_child(1).unwrap();
                let kind = value.str(src);

                Ok(Prop {
                    name,
                    value: kind,
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
    fn extract_string_test() {
        let src = "@@[ \"wow\" ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        assert!("wow" == extract_string(src, &node).unwrap());
    }

    #[test]
    fn extract_integer_test() {
        let src = "@@[ 42 ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        assert!(42 == extract_integer(src, &node).unwrap());
    }

    #[test]
    fn slice_from_test() {
        let src = "@@[ 1..2 ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        assert!(1 == Slice::from(src, &node).start.unwrap());
        assert!(2 == Slice::from(src, &node).end.unwrap());
    }

    #[test]
    fn param_from_test() {
        let src = "@@[ foo=42 ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let param = Param::from(src, &node).unwrap();
        assert!(param.value == ParamKind::Integer(42));
        assert!(param.loc.start_byte == 4);
        assert!(param.loc.end_byte == 10);
    }

    #[test]
    fn prop_from_test() {
        let src = "@@[ foo: #ref ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let prop = Prop::from(src, &node).unwrap();
        assert!(prop.name == "foo");
        assert!(prop.value == "#ref");
        assert!(prop.loc.start_byte == 4);
        assert!(prop.loc.end_byte == 13);
    }

    #[test]
    fn generic_type_from_test() {
        let src = "@@[ String(len=1..10, format=\"did\") ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericType::from(src, &node).unwrap();
        assert!(generic_type.name == "String");
        assert!(generic_type.params.len() == 2);
        match generic_type.params.get("len").unwrap().value {
            ParamKind::Slice(slice) => {
                assert!(slice.start == Some(1));
                assert!(slice.end == Some(10));
            }
            _ => panic!("Unexpected value type"),
        };
        assert!(
            generic_type.params.get("format").unwrap().value
                == ParamKind::String("did".to_string())
        );
    }

    #[test]
    fn extract_string_type_test() {
        let src = "@@[ String(len=42..69, format=\"did\") ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericType::from(src, &node).unwrap();
        let string_type = StringType::from(generic_type);
        //assert!(string_type.format == Some(StringFormats::Did));
        assert!(string_type.length.start == Some(42));
        assert!(string_type.length.end == Some(69));
        assert!(string_type.graphemes.start == None);
        assert!(string_type.graphemes.end == None);
    }
}
