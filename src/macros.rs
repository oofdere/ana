#[macro_export]
macro_rules! create_type_test {
    ($type:ty, $example:expr) => {
        #[cfg(test)]
        paste::paste! {
            mod [<test_ $type:lower>] {
                use super::*;
                use pretty_assertions::assert_eq;

                const EXAMPLE: &str = $example;

                #[test]
                fn deserialize() {
                    let a: $type = serde_json::from_str(EXAMPLE).unwrap();
                    let expected = serde_json::from_str::<$type>(EXAMPLE).unwrap();
                    assert_eq!(a, expected);
                }

                #[test]
                fn serialize() {
                    let a: $type = serde_json::from_str(EXAMPLE).unwrap();
                    let serialized = serde_json::to_string(&a).unwrap();
                    let expected_json: serde_json::Value = serde_json::from_str(EXAMPLE).unwrap();
                    let actual_json: serde_json::Value = serde_json::from_str(&serialized).unwrap();
                    assert_eq!(expected_json, actual_json);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! schema_type {
    ($name:ident, $type:literal, { $($body:tt)* }, $example:literal) => {
        #[skip_serializing_none]
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
        #[serde(tag = "type")]
        #[serde(rename = $type)]
        #[serde(rename_all = "camelCase")]
        //#[serde(deny_unknown_fields)] I'd like to but this breaks tagging :death:
        pub struct $name {
            /// short, usually only a sentence or two
            description: Option<String>,
            $($body)*
        }

        create_type_test!($name, $example);
    };
}
