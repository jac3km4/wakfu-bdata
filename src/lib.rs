use decode::Decode;
use std::marker::PhantomData;

mod decode;
pub mod document;
pub mod types;

pub trait BinaryData: Decode {
    fn id(phantom: PhantomData<Self>) -> i32;
}
