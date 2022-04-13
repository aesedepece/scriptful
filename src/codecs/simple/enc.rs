use alloc::vec::Vec;

use crate::{
    core::value::Value,
    encoding::{
        codecs::simple::{significant_bytes_count, SimpleScriptCodec},
        enc::{Encode, EncodeSequence, Encoder},
    },
    op_systems::simple_math::MathOperator,
    prelude::*,
};

impl<'a> Encoder for &'a mut SimpleScriptCodec {
    type Ok = ();
    type EncodeSequence = Self;

    fn to_vec<Op, Val>(input: &Script<Op, Val>) -> Vec<u8>
    where
        Op: core::fmt::Debug + Encode,
        Val: core::fmt::Debug + Encode,
    {
        let mut codec = SimpleScriptCodec::default();
        input.encode(&mut codec);

        codec.data()
    }

    fn write_u8(self, input: u8) -> Self::Ok {
        self.data_push(input);
    }

    fn write_bytes(self, input: &[u8]) -> Self::Ok {
        for byte in input {
            self.write_u8(*byte);
        }
    }

    fn encode_item<Op, Val>(self, input: &Item<Op, Val>) -> Self::Ok
    where
        Op: core::fmt::Debug + Encode,
        Val: core::fmt::Debug + Encode,
    {
        input.encode(self);
    }

    fn encode_seq(self) -> Self::EncodeSequence {
        self
    }
}

impl<'a> EncodeSequence for &'a mut SimpleScriptCodec {
    type Ok = ();

    fn encode_element<T: Sized>(&mut self, value: &T)
    where
        T: Encode,
    {
        value.encode(&mut **self)
    }

    fn end(self) -> Self::Ok {
        ()
    }
}

impl<Op, Val> Encode for Script<Op, Val>
where
    Op: core::fmt::Debug + Encode,
    Val: core::fmt::Debug + Encode,
{
    fn encode<E>(&self, encoder: E) -> <E as Encoder>::Ok
    where
        E: Encoder,
    {
        let mut seq = encoder.encode_seq();
        for item in self {
            seq.encode_element(item);
        }
        seq.end()
    }
}

impl<Op, Val> Encode for Item<Op, Val>
where
    Op: core::fmt::Debug + Encode,
    Val: core::fmt::Debug + Encode,
{
    fn encode<E>(&self, encoder: E) -> <E as Encoder>::Ok
    where
        E: Encoder,
    {
        match self {
            Item::Operator(op) => op.encode(encoder),
            Item::Value(val) => val.encode(encoder),
        }
    }
}

impl Encode for crate::op_systems::simple_math::MathOperator {
    fn encode<E>(&self, encoder: E) -> <E as Encoder>::Ok
    where
        E: Encoder,
    {
        let discriminant = match self {
            MathOperator::Add => 0x00,
            MathOperator::Equal => 0x01,
            MathOperator::Mul => 0x02,
            MathOperator::Not => 0x03,
            MathOperator::Sub => 0x04,
        };

        encoder.write_u8(discriminant + 0x80)
    }
}

impl Encode for Value {
    fn encode<E>(&self, encoder: E) -> <E as Encoder>::Ok
    where
        E: Encoder,
    {
        let bytes: Vec<u8> = match self {
            Value::Boolean(val) => match val {
                false => Vec::from([0x00]),
                true => Vec::from([0x01]),
            },
            Value::Float(val) => {
                let num_bytes = val.to_le_bytes();
                let first_byte = 0x02;
                [&[first_byte], &num_bytes[..]].concat()
            }
            Value::Integer(val) => {
                let num_bytes = val.to_le_bytes();
                let significant_bytes_count = significant_bytes_count(*val);
                let first_byte = 0x03 + significant_bytes_count as u8;

                [&[first_byte], &num_bytes[..significant_bytes_count + 1]].concat()
            }
            Value::String(val) => {
                if val.len() == 0 {
                    Vec::from([0x13])
                } else {
                    let str_bytes = val.as_bytes();
                    let str_bytes_len_as_bytes = str_bytes.len().to_le_bytes();
                    let str_bytes_len_sbc = 1 + significant_bytes_count(str_bytes.len() as i128);
                    let first_byte = 0x13 + str_bytes_len_sbc as u8;

                    [
                        &[first_byte],
                        &str_bytes_len_as_bytes[..str_bytes_len_sbc],
                        &str_bytes[..],
                    ]
                    .concat()
                }
            }
        };

        encoder.write_bytes(&bytes)
    }
}
