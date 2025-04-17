use lexicon::{AtpNull, AtpTypes};
use tree_sitter::Range;

use crate::GenericType;

#[derive(Debug, PartialEq)]
pub struct Type {
    pub loc: Range,
}

impl From<GenericType> for Type {
    fn from(t: GenericType) -> Self {
        Type { loc: t.loc }
    }
}

impl Into<AtpNull> for Type {
    fn into(self) -> AtpNull {
        AtpNull { description: None }
    }
}

impl Into<AtpTypes> for Type {
    fn into(self) -> AtpTypes {
        AtpTypes::Null(self.into())
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
    fn extract() {
        let src = "@@[ Null ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericType::from(src, &node).unwrap();
        let null_type = Type::from(generic_type);
        assert!(null_type.loc.start_byte == 4);
        assert!(null_type.loc.end_byte == 8);
    }
}
