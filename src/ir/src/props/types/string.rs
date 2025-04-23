use lexicon::{AtpString, AtpTypes, StringFormats};
use tree_sitter::Range;

use crate::{ParamKind, Slice, props::GenericProp};

#[derive(Debug, PartialEq)]
pub struct Type {
    pub format: Option<StringFormats>,
    pub length: Slice,
    pub graphemes: Slice,
    //known_values: todo!(),
    //enumeration: todo!(),
    pub default: Option<String>,
    pub constant: Option<String>,
    pub loc: Range,
}

impl From<GenericProp> for Type {
    fn from(t: GenericProp) -> Self {
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
        let format = t.params.get("format").map_or(None, |x| match &x.value {
            ParamKind::String(s) => Some(s),
            _ => None,
        });

        // TODO: impl default
        // TODO: impl constant

        Type {
            format: format.and_then(|x| StringFormats::from_str(x)),
            length,
            graphemes,
            default: None,
            constant: None,
            loc: t.loc,
        }
    }
}

impl Into<AtpString> for Type {
    fn into(self) -> AtpString {
        AtpString {
            description: None,
            format: self.format,
            min_length: self.length.start.and_then(|x| x.try_into().ok()),
            max_length: self.length.end.and_then(|x| x.try_into().ok()),
            min_graphemes: self.graphemes.start.and_then(|x| x.try_into().ok()),
            max_graphemes: self.graphemes.end.and_then(|x| x.try_into().ok()),
            known_values: None,
            enumeration: None,
            default: self.default,
            constant: self.constant,
        }
    }
}

impl Into<AtpTypes> for Type {
    fn into(self) -> AtpTypes {
        AtpTypes::String(self.into())
    }
}

#[cfg(test)]
mod tests {
    use tree_sitter::{Node, Parser, Tree};

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
    fn base() {
        let src = "@@[ String ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericProp::from(src, &node).unwrap();
        let string_type = Type::from(generic_type);
        assert!(string_type.format == None);
        assert!(string_type.length.start == None);
        assert!(string_type.length.end == None);
        assert!(string_type.graphemes.start == None);
        assert!(string_type.graphemes.end == None);
    }

    #[test]
    fn len() {
        let src = "@@[ String(len=42..69) ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericProp::from(src, &node).unwrap();
        let string_type = Type::from(generic_type);
        assert!(string_type.length.start == Some(42));
        assert!(string_type.length.end == Some(69));
        assert!(string_type.graphemes.start == None);
        assert!(string_type.graphemes.end == None);
    }

    #[test]
    fn graphemes() {
        let src = "@@[ String(graphemes=42..69) ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericProp::from(src, &node).unwrap();
        let string_type = Type::from(generic_type);
        assert!(string_type.graphemes.start == Some(42));
        assert!(string_type.graphemes.end == Some(69));
        assert!(string_type.length.start == None);
        assert!(string_type.length.end == None);
    }

    #[test]
    fn format() {
        let src = "@@[ String(format=\"did\") ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericProp::from(src, &node).unwrap();
        let string_type = Type::from(generic_type);
        assert!(string_type.format == Some(StringFormats::Did));
    }

    #[test]
    fn complex() {
        let src = "@@[ String(len=42..69, format=\"did\") ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericProp::from(src, &node).unwrap();
        let string_type = Type::from(generic_type);
        assert!(string_type.format == Some(StringFormats::Did));
        assert!(string_type.length.start == Some(42));
        assert!(string_type.length.end == Some(69));
        assert!(string_type.graphemes.start == None);
        assert!(string_type.graphemes.end == None);
    }
}
