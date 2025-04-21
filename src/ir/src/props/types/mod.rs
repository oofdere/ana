pub mod blob;
pub mod boolean;
pub mod bytes;
pub mod cid_link;
pub mod integer;
pub mod null;
pub mod string;

#[derive(Debug, PartialEq)]
pub enum Prop {
    Blob(blob::Type),
    Boolean(boolean::Type),
    Bytes(bytes::Type),
    CidLink(cid_link::Type),
    Integer(integer::Type),
    Null(null::Type),
    String(string::Type),
}
