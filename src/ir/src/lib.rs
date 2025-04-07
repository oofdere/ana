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

pub fn extract_integer(src: &str, node: &Node) -> Option<isize> {
    match node.kind() {
        "integer" => Some(node.str(&src).parse().unwrap()),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
pub struct Slice<T> {
    pub start: Option<T>,
    pub end: Option<T>,
    pub loc: Range,
}

pub fn extract_slice(src: &str, node: &Node) -> Slice<isize> {
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
        loc: node.range(),
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
    Integer(isize),
    Slice(Slice<isize>),
}

pub fn extract_param(src: &str, node: &Node) -> Result<Param, ()> {
    match node.kind() {
        "param" => {
            let name = node.named_child(0).unwrap().str(&src);
            let value = node.named_child(1).unwrap();
            let kind = match value.kind() {
                "string" => Ok(ParamKind::String(extract_string(&src, &value).unwrap())),
                "integer" => Ok(ParamKind::Integer(extract_integer(&src, &value).unwrap())),
                "slice" => Ok(ParamKind::Slice(extract_slice(&src, &value))),
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

pub enum TypeKind {
    Ref(String),
    Type(String),
    Array(String),
    Union(String),
}

#[derive(Debug)]
pub struct Prop {
    pub name: String,
    pub value: String, //TypeKind,
    pub loc: Range,
}

pub fn extract_prop(src: &str, node: &Node) -> Result<Prop, ()> {
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
    fn extract_slice_test() {
        let src = "@@[ 1..2 ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        assert!(1 == extract_slice(src, &node).start.unwrap());
        assert!(2 == extract_slice(src, &node).end.unwrap());
    }

    #[test]
    fn extract_param_test() {
        let src = "@@[ foo=42 ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let param = extract_param(src, &node).unwrap();
        assert!(param.value == ParamKind::Integer(42));
        assert!(param.loc.start_byte == 4);
        assert!(param.loc.end_byte == 10);
    }

    #[test]
    fn extract_prop_test() {
        let src = "@@[ foo: #ref ]@@";
        let tree = parse(&src);
        let node = unwrap_harness(&tree);
        let prop = extract_prop(src, &node).unwrap();
        assert!(prop.name == "foo");
        assert!(prop.value == "#ref");
        assert!(prop.loc.start_byte == 4);
        assert!(prop.loc.end_byte == 13);
    }
}
