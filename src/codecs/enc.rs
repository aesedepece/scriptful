//! Traits for encoding of scripts, operators and values.
//!
//! These are modeled after `serde`'s `Serializer` and `Serialize` traits.
//!
//! - `Encoder` shall be implemented by different encoding and decoding mechanisms (aka **codecs**).
//! - `Encode` shall be implemented for every data structure that can be encoded using any codec.

use alloc::vec::Vec;
use core::marker::Sized;

use crate::prelude::*;

pub trait Encode {
    fn encode<E>(&self, encoder: E) -> E::Ok
    where
        E: Encoder;
}

pub trait EncodeSequence {
    type Ok;

    fn encode_element<T: Sized>(&mut self, value: &T)
    where
        T: Encode;

    fn end(self) -> Self::Ok;
}

pub trait Encoder: Sized {
    type Ok;
    type EncodeSequence: EncodeSequence<Ok = Self::Ok>;

    fn to_vec<Op, Val>(input: &Script<Op, Val>) -> Vec<u8>
    where
        Op: core::fmt::Debug + Encode,
        Val: core::fmt::Debug + Encode;

    fn write_u8(self, input: u8) -> Self::Ok;
    fn write_bytes(self, input: &[u8]) -> Self::Ok;

    fn encode_item<Op, Val>(self, input: &Item<Op, Val>) -> Self::Ok
    where
        Op: core::fmt::Debug + Encode,
        Val: core::fmt::Debug + Encode;
    fn encode_seq(self) -> Self::EncodeSequence;
}
