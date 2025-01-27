use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use subenum::subenum;
use serde_with::skip_serializing_none;

schema_type!(
    AtpNull,
    "null",
    {},
    r###"{
        "type": "null",
        "description": "example"
    }"###
);

schema_type!(AtpBoolean, "boolean", {
    default: Option<bool>,
    #[serde(rename="const")]
    constant: Option<bool>
}, r###"{
    "type": "boolean",
    "description": "an example",
    "default": true,
    "const": true
}"###);

schema_type!(AtpInteger, "integer", {
    minimum: Option<i32>,
    maximum: Option<i32>,
    #[serde(rename="enum")]
    enumeration: Option<Vec<i32>>,
    default: Option<i32>,
    #[serde(rename="const")]
    constant: Option<i32>
}, r###"{
    "type": "integer",
    "description": "example integer",
    "minimum": 0,
    "maximum": 1,
    "const": 0,
    "enum": [0, 1]
}"###);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum StringFormats {
    AtIdentifier,
    AtUri,
    Cid,
    Datetime,
    Did,
    Handle,
    Nsid,
    Tid,
    Uri,
    RecordKey,
    Language
}

schema_type!(AtpString, "string", {
    format: Option<StringFormats>,
    max_length: Option<u32>,
    min_length: Option<u32>,
    max_graphemes: Option<u32>,
    min_graphemes: Option<u32>,
    known_values: Option<Vec<String>>,
    #[serde(rename="enum")]
    enumeration: Option<Vec<String>>,
    default: Option<String>,
    #[serde(rename="const")]
    constant: Option<String>
}, r###"{
    "type": "string",
    "description": "an incredibly contrived string example",
    "format": "at-uri",
    "minLength": 0,
    "maxLength": 300,
    "minGraphemes": 0,
    "maxGraphemes": 300,
    "knownValues": ["hello", "world"],
    "enum": ["lexicons", "are", "pain"],
    "const": "pain"
}"###);

schema_type!(AtpBytes, "bytes", {
    min_length: Option<u32>,
    max_length: Option<u32>
}, r###"{
    "type": "bytes",
    "description": "someone please take a byte out of me", 
    "minLength": 1,
    "maxLength": 512
}"###);

schema_type!(AtpCidLink, "cid-link", {}, r###"{
    "type": "cid-link",
    "description": "idk what this really does tbqh"
}"###);

schema_type!(AtpBlob, "blob", {
    accept: Option<Vec<String>>,
    max_size: Option<u32>
}, r###"{
    "type": "blob",
    "description": "an image of the only good formats",
    "accept": ["image/qoi", "image/jxl"],
    "maxSize": 2048
}"###);

schema_type!(AtpArray, "array", {
    items: Box<AtpTypes>,
    min_length: Option<u32>,
    max_length: Option<u32>
}, r###"{
            "type": "array",
            "description": "An array of tags this image had.",
            "maxLength": 10,
            "items": {
              "type": "string",
              "maxLength": 640
            }
          }"###);

schema_type!(AtpObject, "object", {
    properties: HashMap<String, AtpTypes>,
    required: Option<Vec<String>>,
    nullable: Option<Vec<String>>
}, r###"{
        "type": "object",
        "required": ["image", "createdAt"],
        "properties": {
          "createdAt": {
            "type": "string",
            "description": "The timestamp of creation.",
            "format": "datetime"
          },
          "image": {
            "type": "ref",
            "ref": "#image"
          },
          "tags": {
            "type": "array",
            "description": "An array of tags this image had.",
            "maxLength": 10,
            "items": {
              "type": "string",
              "maxLength": 640
            }
          },
          "inResponseTo": {
            "type": "ref",
            "description": "What this oekaki post is a response to.",
            "ref": "com.atproto.repo.strongRef"
          },
          "nsfw": {
            "type": "boolean",
            "description": "Is this oekaki NSFW?"
          }
}}"###);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum ParamProps {
    Boolean(AtpBoolean),
    Integer(AtpInteger),
    String(AtpString),
    Unknown(AtpUnknown),
    Array {
        items: Box<ParamProps>,
        min_length: Option<u32>,
        max_length: Option<u32>
    }
}

schema_type!(AtpParams, "params", {
    required: Option<Vec<String>>,
    properties: HashMap<String, ParamProps>
}, r###"{
				"type": "params",
				"required": ["did", "rkey"],
				"properties": {
					"did": {
						"type": "string",
						"format": "at-identifier",
						"description": "The DID of the author."
					},
					"rkey": {
						"type": "string",
						"description": "The record key."
					}
				}
			}"###);

schema_type!(AtpToken, "token", {}, r###"{
    "type": "token",
    "description": "this is a small token of my appreciation"
}"###);

schema_type!(AtpRef, "ref", {
    #[serde(rename="ref")]
    reference: String
}, r###"{
    "type": "ref",
    "description": "a pointer to the thing I'm going to claim",
    "ref": "local.you#heart"
}"###);

schema_type!(AtpUnion, "union", {
    refs: Vec<String>,
    closed: Option<bool>
}, r###"{
    "type": "union",
    "description": "me and who",
    "refs": ["systems.dere.oof", "lady.office#goth"]
}"###);

schema_type!(AtpUnknown, "unknown", {}, r###"{
    "type": "unknown",
    "description": "me"
}"###);

schema_type!(AtpRecord, "record", {
    key: String, // def make an enum for this
    record: AtpObject
}, r###"{
      "type": "record",
      "key": "tid",
      "description": "An oekaki post.",
      "record": {
        "type": "object",
        "required": ["image", "createdAt"],
        "properties": {
          "createdAt": {
            "type": "string",
            "description": "The timestamp of creation.",
            "format": "datetime"
          },
          "image": {
            "type": "ref",
            "ref": "#image"
          },
          "tags": {
            "type": "array",
            "description": "An array of tags this image had.",
            "maxLength": 10,
            "items": {
              "type": "string",
              "maxLength": 640
            }
          },
          "inResponseTo": {
            "type": "ref",
            "description": "What this oekaki post is a response to.",
            "ref": "com.atproto.repo.strongRef"
          },
          "nsfw": {
            "type": "boolean",
            "description": "Is this oekaki NSFW?"
          }
        }
      }
    }"###);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]

enum RpcSchema {
    Object(AtpObject),
    Ref(AtpRef),
    Union(AtpUnion)
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct RpcIO {
    description: Option<String>,
    encoding: String,
    schema: Option<RpcSchema>
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct RpcError {
    name: String,
    description: Option<String>,
}

    schema_type!(AtpQuery, "query", {
    parameters: Option<AtpParams>,
    output: Option<RpcIO>,
    errors: Option<Vec<RpcError>>
}, r###"{
      "type": "query",
      "description": "Get a blob associated with a given account. Returns the full blob as originally uploaded. Does not require auth; implemented by PDS.",
      "parameters": {
        "type": "params",
        "required": ["did", "cid"],
        "properties": {
          "did": {
            "type": "string",
            "format": "did",
            "description": "The DID of the account."
          },
          "cid": {
            "type": "string",
            "format": "cid",
            "description": "The CID of the blob to fetch"
          }
        }
      },
      "output": {
        "encoding": "*/*"
      },
      "errors": [
        { "name": "BlobNotFound" },
        { "name": "RepoNotFound" },
        { "name": "RepoTakendown" },
        { "name": "RepoSuspended" },
        { "name": "RepoDeactivated" }
      ]
    }"###);

schema_type!(AtpProcedure, "procedure", {
    parameters: Option<AtpParams>,
    output: Option<RpcIO>,
    input: Option<RpcIO>,
    errors: Option<Vec<RpcError>>
}, r###"{
      "type": "procedure",
      "description": "Update an account's email.",
      "input": {
        "encoding": "application/json",
        "schema": {
          "type": "object",
          "required": ["email"],
          "properties": {
            "email": { "type": "string" },
            "emailAuthFactor": { "type": "boolean" },
            "token": {
              "type": "string",
              "description": "Requires a token from com.atproto.sever.requestEmailUpdate if the account's email has been confirmed."
            }
          }
        }
      },
      "errors": [
        { "name": "ExpiredToken" },
        { "name": "InvalidToken" },
        { "name": "TokenRequired" }
      ]
    }"###);

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct RpcMessage {
    description: Option<String>,
    schema: AtpUnion
}

schema_type!(AtpSubscription, "subscription", {
    parameters: Option<AtpParams>,
    message: RpcMessage,
    errors: Option<Vec<RpcError>>
}, r###"{
      "type": "subscription",
      "description": "Repository event stream, aka Firehose endpoint. Outputs repo commits with diff data, and identity update events, for all repositories on the current server. See the atproto specifications for details around stream sequencing, repo versioning, CAR diff format, and more. Public and does not require auth; implemented by PDS and Relay.",
      "parameters": {
        "type": "params",
        "properties": {
          "cursor": {
            "type": "integer",
            "description": "The last known event seq number to backfill from."
          }
        }
      },
      "message": {
        "schema": {
          "type": "union",
          "refs": [
            "#commit",
            "#identity",
            "#account",
            "#handle",
            "#migrate",
            "#tombstone",
            "#info"
          ]
        }
      },
      "errors": [
        { "name": "FutureCursor" },
        {
          "name": "ConsumerTooSlow",
          "description": "If the consumer of the stream can not keep up with events, and a backlog gets too large, the server will drop the connection."
        }
      ]
    }"###);

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
    Subscription(AtpSubscription),
}

mod test {
    use serde::{Serialize, Deserialize};

    use crate::types::{AtpBoolean, Field};

    use super::AtpTypes;

    #[test]
    fn integ() {
        let a: Field = serde_json::from_str(
            r###"{
              "type": "boolean",
              "description": "Is this oekaki NSFW?"
            }"###,
        )
        .unwrap();

        assert_eq!(
            a,
            AtpTypes::Boolean(AtpBoolean {
                description: Some(String::from("Is this oekaki NSFW?")),
                constant: None,
                default: None
            })
        );
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    #[serde(tag = "type")]
    #[serde(deny_unknown_fields)]
    struct Test {
      description: Option<String>,
    }

    #[test]
    fn test() {
        let test = Test { description: None };
        let test_string = serde_json::to_string(&test).unwrap();
        let _: Test = serde_json::from_str(&test_string).unwrap();
    }
}
