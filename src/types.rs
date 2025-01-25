use serde::{Deserialize, Serialize};
use subenum::subenum;

schema_type!(AtpNull, "null", {}, 
    r###"{
        "type": "null",
        "description": "example"
    }"###);

schema_type!(AtpBoolean, "boolean", {
    default: Option<bool>,
    #[serde(rename="const")]
    constant: Option<bool>
}, r###"{
    "type": "boolean",
    "description": "an example",
    "default": true,
    "const": "true"
}"###);

schema_type!(AtpInteger, "integer", {}, r###""###);

schema_type!(AtpString, "string", {}, r###""###);

schema_type!(AtpBytes, "bytes", {}, r###""###);

schema_type!(AtpCidLink, "cid-link", {}, r###""###);
schema_type!(AtpBlob, "blob", {}, r###""###);
schema_type!(AtpArray, "array", {}, r###""###);
schema_type!(AtpObject, "object", {}, r###""###);
schema_type!(AtpParams, "params", {}, r###""###);
schema_type!(AtpToken, "token", {}, r###""###);
schema_type!(AtpRef, "ref", {}, r###""###);
schema_type!(AtpUnion, "union", {}, r###""###);
schema_type!(AtpUnknown, "unknown", {}, r###""###);
schema_type!(AtpRecord, "record", {}, r###""###);
schema_type!(AtpQuery, "query", {}, r###""###);
schema_type!(AtpProcedure, "procedure", {}, r###""###);
schema_type!(AtpSubscription, "subscription", {}, r###""###);

#[subenum(Field)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum AtpTypes {
    #[subenum(Field)]
    Null(AtpNull),
    #[subenum(Field)]
    Boolean(AtpBoolean),
    #[subenum(Field)]
    Integer(AtpInteger),
    #[subenum(Field)]
    String(AtpString),
    #[subenum(Field)]
    Bytes(AtpBytes),
    #[subenum(Field)]
    CidLink(AtpCidLink),
    #[subenum(Field)]
    Blob(AtpBlob),
    #[subenum(Field)]
    Array(AtpArray),
    #[subenum(Field)]
    Object(AtpObject),
    #[subenum(Field)]
    Params(AtpParams),
    #[subenum(Field)]
    Token(AtpToken),
    #[subenum(Field)]
    Ref(AtpRef),
    #[subenum(Field)]
    Union(AtpUnion),
    #[subenum(Field)]
    Unknown(AtpUnknown),
    Record(AtpRecord),
    Query(AtpQuery),
    Procedure(AtpProcedure),
    Subscription(AtpSubscription)
}

mod test {
    use crate::types::{AtpBoolean, Field};

    use super::AtpTypes;

    #[test]
    fn integ() {
        let a: Field = serde_json::from_str(r###"{
              "type": "boolean",
              "description": "Is this oekaki NSFW?"
            }"###).unwrap();

        assert_eq!(a, AtpTypes::Boolean(AtpBoolean { description: Some(String::from("Is this oekaki NSFW?")), constant: None, default: None }));
    }
}