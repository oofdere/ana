use lexicon::{AtpString, AtpTypes, StringFormats};
use tree_sitter::Range;

use crate::{GenericType, ParamKind, Slice};

pub trait Type {
    /// convert from a GenericType to Self, returns an Option with None if there are any errors during conversion
    fn from_generic(t: GenericType) -> Self;
}

#[derive(Debug, PartialEq)]
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
