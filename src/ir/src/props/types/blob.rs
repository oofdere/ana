use lexicon::{AtpBlob, AtpTypes};
use tree_sitter::Range;

use crate::{ParamKind, props::GenericProp};

#[derive(Debug, PartialEq)]
pub struct Type {
    pub accept: Option<Vec<String>>,
    pub size: Option<i32>,
    pub loc: Range,
}

impl From<GenericProp> for Type {
    fn from(t: GenericProp) -> Self {
        let accept = t.params.get("accept").map_or(None, |x| {
            match &x.value {
                ParamKind::String(x) => Some(vec![x.clone()]), // todo: scalar supports
                _ => None,
            }
        });

        let size = t.params.get("size").map_or(None, |x| match x.value {
            ParamKind::Integer(x) => Some(x),
            _ => None,
        });

        Type {
            accept,
            size,
            loc: t.loc,
        }
    }
}

impl Into<AtpBlob> for Type {
    fn into(self) -> AtpBlob {
        AtpBlob {
            description: None,
            accept: self.accept,
            max_size: self.size.and_then(|x| x.try_into().ok()),
        }
    }
}

impl Into<AtpTypes> for Type {
    fn into(self) -> AtpTypes {
        AtpTypes::Blob(self.into())
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
        let src = "@@[ Blob ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericProp::from(src, &node).unwrap();
        let blob_type = Type::from(generic_type);
        assert!(blob_type.size == None)
    }

    #[test]
    fn size() {
        let src = "@@[ Blob(size=1024) ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericProp::from(src, &node).unwrap();
        let blob_type = Type::from(generic_type);
        assert!(blob_type.size == Some(1024));
    }

    #[test]
    fn accept() {
        let src = "@@[ Blob(accept=\"image/jxl\") ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let generic_type = GenericProp::from(src, &node).unwrap();
        let blob_type = Type::from(generic_type);
        assert!(blob_type.accept == Some(vec!["image/jxl".to_string()]));
    }
}
