use crate::decode::Decode;

pub trait BinaryData: Decode {
    const TYPE_ID: i16;
}
