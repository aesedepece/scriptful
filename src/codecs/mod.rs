//! Codecs are different ways to encode and decode scripts, normally into and from compact binary
//! formats.
//!
//! The traits and implementations found in this module are really similar to those used by `serde`.
//! However, these are intentionally not compatible, to avoid all the `std` overhead that `serde`
//! brings with itself.

use alloc::string::String;

use crate::prelude::*;

pub mod dec;
pub mod enc;

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::codecs::codecs::simple;
    use crate::codecs::dec::Decoder;
    use crate::codecs::enc::Encoder;
    use crate::core::item::Item::*;
    use crate::core::value::Value::*;
    use crate::op_systems::simple_math::MathOperator::{self, *};
    use crate::prelude::*;

    fn example_bytes() -> Vec<u8> {
        Vec::from([
            3, 1, 6, 255, 224, 245, 5, 128, 2, 31, 133, 235, 81, 184, 30, 9, 64, 130, 20, 13, 72,
            101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33, 19,
        ])
    }

    fn example_script() -> Script<MathOperator> {
        Vec::from([
            Value(Integer(1)),
            Value(Integer(99999999)),
            Operator(Add),
            Value(Float(3.14)),
            Operator(Mul),
            Value(String("Hello, World!".into())),
            Value(String("".into())),
        ])
    }

    #[test]
    fn test_encoding() {
        let decoded: Script<MathOperator> = example_script();

        let encoded = <&mut simple::SimpleScriptCodec>::to_vec(&decoded);
        let expected = example_bytes();

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_decoding() {
        let encoded = example_bytes();

        let decoded = <&mut simple::SimpleScriptCodec>::from_vec(encoded).unwrap();
        let expected = example_script();

        assert_eq!(decoded, expected);
    }
}

#[derive(Debug, PartialEq)]
pub struct EncodingError(String);

#[derive(Debug, PartialEq)]
pub struct DecodingError(String);

impl Error for EncodingError {
    fn from_str(input: &str) -> Self {
        EncodingError(input.into())
    }
}

impl Error for DecodingError {
    fn from_str(input: &str) -> Self {
        DecodingError(input.into())
    }
}
