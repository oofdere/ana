use lexicon::{AtpInteger, AtpTypes};
use tree_sitter::Range;

use crate::{ParamKind, Slice, props::GenericType};

#[derive(Debug, PartialEq)]
pub struct Type {
    pub range: Slice,
    pub default: Option<i32>,
    pub loc: Range,
}

impl From<GenericType> for Type {
    fn from(t: GenericType) -> Self {
        let range = t
            .params
            .get("range")
            .map_or(Slice::empty(), |x| match x.value {
                ParamKind::Slice(s) => s,
                _ => Slice::empty(),
            });
        let default = t.params.get("default").map_or(None, |x| match x.value {
            ParamKind::Integer(i) => Some(i),
            _ => None,
        });

        Type {
            default,
            range,
            loc: t.loc,
        }
    }
}

impl Into<AtpInteger> for Type {
    fn into(self) -> AtpInteger {
        AtpInteger {
            description: None,
            constant: None,
            default: self.default,
            minimum: self.range.start,
            maximum: self.range.end,
            enumeration: None, // todo
        }
    }
}

impl Into<AtpTypes> for Type {
    fn into(self) -> AtpTypes {
        AtpTypes::Integer(self.into())
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
        let src = "@@[ Integer ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericType::from(src, &node).unwrap();
        let integer_type = Type::from(generic_type);
        assert!(integer_type.loc.start_byte == 4);
        assert!(integer_type.loc.end_byte == 11);
    }

    #[test]
    fn default() {
        let src = "@@[ Integer(default=1) ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericType::from(src, &node).unwrap();
        let integer_type = Type::from(generic_type);
        assert!(integer_type.loc.start_byte == 4);
        assert!(integer_type.loc.end_byte == 22);
        assert!(integer_type.default == Some(1))
    }

    #[test]
    fn range() {
        let src = "@@[ Integer(range=42..69) ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericType::from(src, &node).unwrap();
        let integer_type = Type::from(generic_type);
        assert!(integer_type.loc.start_byte == 4);
        assert!(integer_type.loc.end_byte == 25);
        assert!(integer_type.range.start == Some(42));
        assert!(integer_type.range.end == Some(69));
    }
}
