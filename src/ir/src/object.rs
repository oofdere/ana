use std::collections::HashMap;

use tree_sitter::{Node, Range};

use crate::{
    NodeHelpers,
    props::{Prop, parse_properties},
};

pub fn parse_object(src: &str, node: &Node) -> Result<Object, ()> {
    // this function extracts generic type info from a type node
    // generic types are not part of the IR and should be converted to a specific type

    match node.kind() {
        "object" => {
            let name = node.named_child(0).unwrap().str(&src);
            //panic!("{:#?}", &node.named_child(1).unwrap().to_sexp());
            let props = parse_properties(&src, &node.named_child(1).unwrap());
            //panic!("{:#?}", props);

            Ok(Object {
                name,
                props,
                loc: node.range(),
            })
        }
        _ => panic!("{:?}", node.kind()),
    }
}

#[derive(Debug)]
pub struct Object {
    pub name: String,
    pub props: HashMap<String, Prop>,
    pub loc: Range,
}

#[cfg(test)]
mod tests {
    use tree_sitter::{Node, Parser, Tree};

    use crate::props::PropKind;

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
        let src = "@@[ image { foo: String(default=\"bar\") } ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let object = parse_object(src, &node).unwrap();
        assert!(object.name == "image");
        //panic!("{:#?}", object);
        assert!(object.props.contains_key("foo"));
        assert!(match &object.props.get("foo").unwrap().value {
            PropKind::String(_x) => true,
            _ => false,
        });
    }
}
