use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use subenum::subenum;

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
    /// a default value for this field
    default: Option<bool>,
    #[serde(rename="const")]
    /// a fixed (constant) value for this field
    constant: Option<bool>
}, r###"{
    "type": "boolean",
    "description": "an example",
    "default": true,
    "const": true
}"###);

schema_type!(AtpInteger, "integer", {
    /// minimum acceptable value
    minimum: Option<i32>,
    /// maximum acceptable value
    maximum: Option<i32>,
    /// a closed set of allowed values
    #[serde(rename="enum")]
    enumeration: Option<Vec<i32>>,
    /// a default value for this field
    default: Option<i32>,
    /// a fixed (constant) value for this field
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, derive_display_from_debug::Display)]
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
    Language,
}

schema_type!(AtpString, "string", {
    /// string format restriction
    format: Option<StringFormats>,
    /// maximum length of value, in UTF-8 bytes
    max_length: Option<u32>,
    /// minimum length of value, in UTF-8 bytes
    min_length: Option<u32>,
    /// maximum length of value, counted as Unicode Grapheme Clusters
    max_graphemes: Option<u32>,
    /// minimum length of value, counted as Unicode Grapheme Clusters
    min_graphemes: Option<u32>,
    /// a set of suggested or common values for this field. Values are not limited to this set (aka, not a closed enum).
    known_values: Option<Vec<String>>,
    /// a closed set of allowed values
    #[serde(rename="enum")]
    enumeration: Option<Vec<String>>,
    /// a default value for this field
    default: Option<String>,
    /// a fixed (constant) value for this field
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
    /// minimum size of value, as raw bytes with no encoding
    min_length: Option<u32>,
    /// maximum size of value, as raw bytes with no encoding
    max_length: Option<u32>
}, r###"{
    "type": "bytes",
    "description": "someone please take a byte out of me", 
    "minLength": 1,
    "maxLength": 512
}"###);

schema_type!(
    AtpCidLink,
    "cid-link",
    {},
    r###"{
    "type": "cid-link",
    "description": "idk what this really does tbqh"
}"###
);

schema_type!(AtpBlob, "blob", {
    /// list of acceptable MIME types. Each may end in `*` as a glob pattern (eg, `image/*`). Use `*/*` to indicate that any MIME type is accepted.
    accept: Option<Vec<String>>,
    /// maximum size in bytes
    max_size: Option<u32>
}, r###"{
    "type": "blob",
    "description": "an image of the only good formats",
    "accept": ["image/qoi", "image/jxl"],
    "maxSize": 2048
}"###);

schema_type!(AtpArray, "array", {
    /// describes the schema elements of this array
    items: Box<AtpTypes>,
    /// minimum count of elements in array
    min_length: Option<u32>,
    /// maximum count of elements in array
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
    /// defines the properties (fields) by name, each with their own schema
    properties: HashMap<String, AtpTypes>,
    /// indicates which properties are required
    required: Option<Vec<String>>,
    /// indicates which properties can have null as a value
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, derive_display_from_debug::Display)]
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
        max_length: Option<u32>,
    },
}

schema_type!(AtpParams, "params", {
    /// same semantics as field on `object`
    required: Option<Vec<String>>,
    /// similar to properties under `object`, but can only include the types `boolean`, `integer`, `string`, and `unknown`; or an `array` of one of these types
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

schema_type!(
    AtpToken,
    "token",
    {},
    r###"{
    "type": "token",
    "description": "this is a small token of my appreciation"
}"###
);

schema_type!(AtpRef, "ref", {
    /// reference to another schema definition
    #[serde(rename="ref")]
    reference: String
}, r###"{
    "type": "ref",
    "description": "a pointer to the thing I'm going to claim",
    "ref": "local.you#heart"
}"###);

schema_type!(AtpUnion, "union", {
    /// references to schema definitions
    refs: Vec<String>,
    /// indicates if a union is "open" or "closed". defaults to `false` (open union)
    closed: Option<bool>
}, r###"{
    "type": "union",
    "description": "me and who",
    "refs": ["systems.dere.oof", "lady.office#goth"]
}"###);

schema_type!(
    AtpUnknown,
    "unknown",
    {},
    r###"{
    "type": "unknown",
    "description": "me"
}"###
);

schema_type!(AtpRecord, "record", {
    /// specifies the Record Key `type` (e.g. tid)
    key: String, // def make an enum for this
    /// a schema definition with type `object`, which specifies this type of record
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, derive_display_from_debug::Display)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
/// schema definition, either an object, a ref, or a union of refs. Used to describe JSON encoded responses, though schema is optional even for JSON responses.
enum RpcSchema {
    Object(AtpObject),
    Ref(AtpRef),
    Union(AtpUnion),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, derive_display_from_debug::Display)]
struct RpcIO {
    description: Option<String>,
    encoding: String,
    schema: Option<RpcSchema>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, derive_display_from_debug::Display)]
struct RpcError {
    /// short name for the error type, with no whitespace
    name: String,
    /// short description, one or two sentences
    description: Option<String>,
}

schema_type!(AtpQuery, "query", {
    /// a schema definition with type `params`, describing the HTTP query parameters for this endpoint
    parameters: Option<AtpParams>,
    /// describes the HTTP response body
    output: Option<RpcIO>,
    /// set of string error codes which might be returned
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
    /// a schema definition with type `params`, describing the HTTP query parameters for this endpoint
    parameters: Option<AtpParams>,
    /// describes the HTTP response body
    output: Option<RpcIO>,
    /// describes HTTP request body schema, with the same format as the `output` field
    input: Option<RpcIO>,
    /// set of string error codes which might be returned
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, derive_display_from_debug::Display)]
struct RpcMessage {
    description: Option<String>,
    schema: AtpUnion,
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, derive_display_from_debug::Display)]
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
    use serde::{Deserialize, Serialize};

    #[test]
    fn integ() {
        use crate::lexicon::*;
        let a: Field = serde_json::from_str(
            r###"{
              "type": "boolean",
              "description": "Is this oekaki NSFW?"
            }"###,
        )
        .unwrap();

        println!("{a}");

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
