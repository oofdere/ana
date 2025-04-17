use lexicon::{AtpBoolean, AtpTypes};
use tree_sitter::Range;

use crate::GenericType;

#[derive(Debug, PartialEq)]
pub struct Type {
    pub default: Option<bool>,
    pub loc: Range,
}

impl From<GenericType> for Type {
    fn from(t: GenericType) -> Self {
        Type {
            default: None,
            loc: t.loc,
        }
    }
}

impl Into<AtpBoolean> for Type {
    fn into(self) -> AtpBoolean {
        AtpBoolean {
            description: None,
            constant: None,
            default: None,
        }
    }
}

impl Into<AtpTypes> for Type {
    fn into(self) -> AtpTypes {
        AtpTypes::Boolean(self.into())
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
        let src = "@@[ Boolean ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericType::from(src, &node).unwrap();
        let boolean_type = Type::from(generic_type);
        assert!(boolean_type.loc.start_byte == 4);
        assert!(boolean_type.loc.end_byte == 11);
    }

    #[test]
    #[ignore] // TODO: implement bool primative in grammar
    fn default_value() {
        let src = "@@[ Boolean(default=True) ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericType::from(src, &node).unwrap();
        let boolean_type = Type::from(generic_type);
        assert!(boolean_type.loc.start_byte == 4);
        assert!(boolean_type.loc.end_byte == 8);
    }
}
