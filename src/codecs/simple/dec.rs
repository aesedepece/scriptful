use alloc::string::String;
use alloc::vec::Vec;

use crate::{
    core::value::Value,
    encoding::{
        codecs::simple::SimpleScriptCodec,
        dec::{Decode, Decoder},
        DecodingError,
    },
    op_systems::simple_math::MathOperator,
    prelude::*,
};

impl<'a> Decoder for &'a mut SimpleScriptCodec {
    type Error = DecodingError;

    fn decode_i128(&mut self) -> Result<i128, Self::Error> {
        let length = self.read_byte()? as usize - 0x02;
        let significant_bytes = self.read_bytes(length)?;
        let mut sixteen_bytes = [0u8; 16];
        sixteen_bytes[..length].copy_from_slice(&significant_bytes);
        let integer = i128::from_le_bytes(sixteen_bytes);

        Ok(integer)
    }

    fn decode_f64(&mut self) -> Result<f64, Self::Error> {
        self.read_byte()?;
        let bytes = self.read_bytes(8)?;
        let mut eight_bytes = [0u8; 8];
        eight_bytes.copy_from_slice(&bytes);
        let float = f64::from_le_bytes(eight_bytes);

        Ok(float)
    }

    fn decode_string(&mut self) -> Result<String, Self::Error> {
        let length_length = self.read_byte()? as usize - 0x13;
        let length_bytes = self.read_bytes(length_length)?;
        let mut eight_length_bytes = [0u8; 8];
        eight_length_bytes[..length_length].copy_from_slice(&length_bytes);
        let length = usize::from_le_bytes(eight_length_bytes);
        let bytes = self.read_bytes(length)?;
        let string = String::from_utf8(bytes.into())
            .map_err(|_| DecodingError::from_str("Not a valid UTF-8 string"));

        string
    }

    fn decode_item<Op, Val>(&mut self) -> Result<Item<Op, Val>, Self::Error>
    where
        Op: core::fmt::Debug + Decode,
        Val: core::fmt::Debug + Decode,
    {
        let byte = self.peek_byte()?;
        if *byte < 0x80 {
            Val::decode(&mut *self).map(Item::Value)
        } else {
            Op::decode(&mut *self).map(Item::Operator)
        }
    }

    fn decode_script<Op, Val>(&mut self) -> Result<Script<Op, Val>, Self::Error>
    where
        Op: core::fmt::Debug + Decode,
        Val: core::fmt::Debug + Decode,
    {
        let mut script = Script::<Op, Val>::new();

        while self.bytes_left() > 0 {
            let item = self.decode_item().unwrap();
            script.push(item);
        }

        Ok(script)
    }

    fn from_vec<Op, Val>(input: Vec<u8>) -> Result<Script<Op, Val>, Self::Error>
    where
        Op: core::fmt::Debug + Decode,
        Val: core::fmt::Debug + Decode,
    {
        let mut codec = SimpleScriptCodec::from_data(input);
        let script = <&mut SimpleScriptCodec as Decoder>::decode_script(&mut &mut codec);

        script
    }

    fn peek_byte(&self) -> Result<&u8, Self::Error> {
        self.data
            .get(self.cursor)
            .ok_or_else(|| DecodingError::from_str("Decoder cursor hit end of vector"))
    }

    fn read_byte(&mut self) -> Result<u8, Self::Error> {
        if self.cursor < self.data.len() {
            let byte = self.data[self.cursor];
            self.cursor += 1;

            Ok(byte)
        } else {
            Err(DecodingError::from_str(
                "Decoder cursor hit end of vector when reading a single byte",
            ))
        }
    }

    fn read_bytes(&mut self, length: usize) -> Result<&[u8], Self::Error> {
        if self.cursor + length <= self.data.len() {
            let bytes = &self.data[self.cursor..self.cursor + length];
            self.cursor += length;

            Ok(bytes)
        } else {
            Err(DecodingError::from_str(&alloc::format!(
                "Decoder cursor hit end of vector when reading {} bytes, while the decoder only had {} in its data vector",
                length, self.bytes_left()
            )))
        }
    }
}

impl Decode for MathOperator {
    fn decode<D>(decoder: &mut D) -> Result<Self, <D as Decoder>::Error>
    where
        D: Decoder,
    {
        let discriminant = decoder.read_byte()? - 0x80;

        match discriminant {
            0x00 => Ok(MathOperator::Add),
            0x01 => Ok(MathOperator::Equal),
            0x02 => Ok(MathOperator::Mul),
            0x03 => Ok(MathOperator::Not),
            0x04 => Ok(MathOperator::Sub),
            x => Err(<D as Decoder>::Error::from_str(&alloc::format!(
                "Unsupported MathOperator {}",
                x
            ))),
        }
    }
}

impl Decode for Value {
    fn decode<D>(decoder: &mut D) -> Result<Self, <D as Decoder>::Error>
    where
        D: Decoder,
    {
        let discriminant = decoder.peek_byte()?;

        match discriminant {
            0x00 => decoder.read_byte().map(|_| Value::Boolean(false)),
            0x01 => decoder.read_byte().map(|_| Value::Boolean(true)),
            0x02 => decoder.decode_f64().map(Value::Float),
            0x03..=0x012 => decoder.decode_i128().map(Value::Integer),
            0x13..=0x79 => decoder.decode_string().map(Value::String),
            x => Err(<D as Decoder>::Error::from_str(&alloc::format!(
                "Unsupported value discriminant {}",
                x
            ))),
        }
    }
}
