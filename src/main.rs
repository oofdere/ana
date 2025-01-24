use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Lexicon {
    lexicon: i32,
    id: String,
    revision: Option<i32>,
    description: Option<String>,
    defs: HashMap<String, Primary> // switch with a more specific type later
}

// type categories don't make much sense, but they come from atproto.com/specs/lexicon so blame them!
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum Primary {
    Record {
        description: Option<String>,
        key: String,
        record: Value // "a schema definition with type object, which specifies this type of record"
    },
    Query {
        description: Option<String>,
        parameters: Option<Params>, // a schema definition with type params, describing the HTTP query parameters for this endpoint
        output: Option<Message>, // describes the HTTP response body
        errors: Option<Vec<Error>> //
    },
    Procedure {
        description: Option<String>,
        parameters: Option<Params>, // a schema definition with type params, describing the HTTP query parameters for this endpoint
        output: Option<Message>, // describes the HTTP response body
        input: Option<Message>, // describes the HTTP request body
        errors: Option<Vec<Error>> //
    },
    // subscriptions will be added later
    #[serde(rename = "object")]
    AtpObject { // don't really see an alternative to tossing this here
        properties: Value,
        required: Option<Vec<String>>,
        nullable: Option<Vec<String>>
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    description: Option<String>,
    encoding: String, // mimetype
    schema: Option<Value> // schema definition, either an object, a ref, or a union of refs. Used to describe JSON encoded responses, though schema is optional even for JSON responses.
}

#[derive(Serialize, Deserialize, Debug)]
struct Error {
    name: String,
    description: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type")]
#[serde(rename_all = "lowercase")]
#[serde(rename_all_fields = "camelCase")]
enum Fields {
    Null { 
        description: Option<String>,
    },
    Boolean {
        description: Option<String>,
        default: Option<bool>,
        #[serde(rename = "const")]
        constant: Option<bool>
    },
    Integer {
        description: Option<String>,
        minimum: Option<i32>,
        maximum: Option<i32>,
        #[serde(rename = "enum")]
        enumeration: Option<Vec<i32>>,
        default: Option<i32>,
        #[serde(rename = "const")]
        constant: Option<i32>
    },
    String {
        description: Option<String>,
        format: Option<String>, // make an enum for this
        max_length: Option<u32>,
        min_length: Option<u32>,
        max_graphemes: Option<u32>,
        min_graphemes: Option<u32>,
        known_values: Option<Vec<String>>,
        #[serde(rename = "enum")]
        enumeration: Option<Vec<String>>,
        default: Option<String>,
        #[serde(rename = "const")]
        constant: Option<String>
    },
    Bytes {
        description: Option<String>,
        max_length: Option<i32>,
        min_length: Option<i32>,
    },
    #[serde(rename="cid-link")]
    CidLink {
        description: Option<String>,
    },
    Array {
        items: Value,
        max_length: Option<i32>,
        min_length: Option<i32>,
    },
    #[serde(rename = "object")]
    AtpObject {
        properties: Value,
        required: Option<Vec<String>>,
        nullable: Option<Vec<String>>
    },
    Blob {
        accept: Option<Vec<String>>,
        max_size: u32
    },
    Params(Params),
    Token,
    Ref {
        #[serde(rename = "ref")]
        reference: String
    },
    Union {
        refs: Vec<String>,
        closed: Option<bool>
    },
    Unknown
}

#[derive(Serialize, Deserialize, Debug)]
struct Params {
    required: Option<Vec<String>>,
    properties: Value
}



mod tests {
    use super::*;

    

    #[test]
    fn test_lexicon() {
        let pinksea = r###"
        {
    "$schema": "https://internect.info/lexicon-schema.json",
    "lexicon": 1,
    "id": "com.shinolabs.pinksea.oekaki",
    "defs": {
      "main": {
        "key": "tid",
        "type": "record",
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
      },
      "image": {
        "type": "object",
        "required": ["blob", "imageLink"],
        "properties": {
          "blob": {
            "type": "blob",
            "accept": ["image/png"],
            "maxSize": 1048576,
            "description": "The actual atproto image blob."
          },
          "imageLink": {
            "type": "ref",
            "ref": "#imageLink"
          }
        }
      },
      "imageLink": {
        "type": "object",
        "description": "A link to the image, it can be either directly to the PDS or to a CDN.",
        "required": ["fullsize"],
        "properties": {
          "fullsize": {
            "type": "string",
            "format": "uri",
            "description": "Fully-qualified URL where a large version of the image can be fetched."
          },
          "alt": {
            "type": "string",
            "description": "Alt text description of the image, for accessibility."
          }
        }
      }
    }
  }
        "###;

        let a: Lexicon = serde_json::from_str(&pinksea).unwrap();

        eprintln!("{:#?}", a);
    }
}

fn main() {
    println!("Hello, world!");
}