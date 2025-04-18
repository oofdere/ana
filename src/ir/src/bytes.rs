use lexicon::{AtpBytes, AtpTypes};
use tree_sitter::Range;

use crate::{GenericType, ParamKind, Slice};

#[derive(Debug, PartialEq)]
pub struct Type {
    pub size: Slice,
    pub loc: Range,
}

impl From<GenericType> for Type {
    fn from(t: GenericType) -> Self {
        let size = t
            .params
            .get("size")
            .map_or(Slice::empty(), |x| match x.value {
                ParamKind::Slice(s) => s,
                _ => Slice::empty(),
            });

        Type { size, loc: t.loc }
    }
}

impl Into<AtpBytes> for Type {
    fn into(self) -> AtpBytes {
        AtpBytes {
            description: None,
            min_length: self.size.start.and_then(|x| x.try_into().ok()),
            max_length: self.size.end.and_then(|x| x.try_into().ok()),
        }
    }
}

impl Into<AtpTypes> for Type {
    fn into(self) -> AtpTypes {
        AtpTypes::Bytes(self.into())
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
        let src = "@@[ Bytes ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericType::from(src, &node).unwrap();
        let integer_type = Type::from(generic_type);
        assert!(integer_type.size == Slice::empty())
    }

    #[test]
    fn size() {
        let src = "@@[ Bytes(size=4096..8192) ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericType::from(src, &node).unwrap();
        let integer_type = Type::from(generic_type);
        assert!(integer_type.size.start == Some(4096));
        assert!(integer_type.size.end == Some(8192));
    }
}
