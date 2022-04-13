//! Traits for decoding of scripts, operators and values.
//!
//! These are modeled after `serde`'s `Deserializer` and `Deserialize` traits.
//!
//! - `Decoder` shall be implemented by different encoding and decoding mechanisms (aka **codecs**).
//! - `Decode` shall be implemented for every data structure that can be decoded using any codec.

use alloc::string::String;
use alloc::vec::Vec;

use crate::prelude::*;

pub trait Decoder: Sized {
    type Error: Error;

    fn decode_i128(&mut self) -> Result<i128, Self::Error>;

    fn decode_f64(&mut self) -> Result<f64, Self::Error>;

    fn decode_string(&mut self) -> Result<String, Self::Error>;

    fn decode_item<Op, Val>(&mut self) -> Result<Item<Op, Val>, Self::Error>
    where
        Op: core::fmt::Debug + Decode,
        Val: core::fmt::Debug + Decode;

    fn decode_script<Op, Val>(&mut self) -> Result<Script<Op, Val>, Self::Error>
    where
        Op: core::fmt::Debug + Decode,
        Val: core::fmt::Debug + Decode;

    fn from_vec<Op, Val>(input: Vec<u8>) -> Result<Script<Op, Val>, Self::Error>
    where
        Op: core::fmt::Debug + Decode,
        Val: core::fmt::Debug + Decode;

    fn peek_byte(&self) -> Result<&u8, Self::Error>;

    fn read_byte(&mut self) -> Result<u8, Self::Error>;

    fn read_bytes(&mut self, length: usize) -> Result<&[u8], Self::Error>;
}

pub trait Decode: Sized {
    fn decode<D>(decoder: &mut D) -> Result<Self, D::Error>
    where
        D: Decoder;
}
