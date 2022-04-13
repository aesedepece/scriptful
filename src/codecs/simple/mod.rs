//! A example of a very simple and compact binary codec for the `SimpleMath` operator system.
//!
//! This is somehow similar to [CBOR], but specific to the data types defined in [`Value`][Value].
//!
//! As a matter of fact, this is just as concise as CBOR, and the main difference is endianness and
//! simpler logic for encoding and decoding.
//!
//! | Value                | CBOR                                   | Simple                               |
//! |----------------------|----------------------------------------|--------------------------------------|
//! | `false`              | `F4`                                   | `00`                                 |
//! | `3.14`               | `FB40091EB851EB851F`                   | `021F85EB51B81E0940`                 |
//! | `255`                | `18FF`                                 | `03FF`                               |
//! | `999999999999999999` | `1B0DE0B6B3A763FFFF`                   | `0AFFFF63A7B3B6E00D`                 |
//! | `i128::MIN`          | `C3507FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF` | `1200000000000000000000000000000080` |
//! | `i128::MAX`          | `C2507FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF` | `12FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7F` |
//! | `""`                 | `60`                                   | `13`                                 |
//! | `"Hello, World!"`    | `6D48656C6C6F2C20576F726C6421`         | `140D48656C6C6F2C20576F726C6421`     |
//!
//! [CBOR]: https://www.rfc-editor.org/rfc/rfc8949
//! [Value]: ../../../core/value/enum.Value.html

use alloc::vec::Vec;

pub mod dec;
pub mod enc;

#[derive(Default)]
pub struct SimpleScriptCodec {
    data: Vec<u8>,
    cursor: usize,
}

impl SimpleScriptCodec {
    pub fn data(self) -> Vec<u8> {
        self.data
    }

    pub fn data_push(&mut self, input: u8) {
        self.data.push(input)
    }

    pub fn from_data(data: Vec<u8>) -> Self {
        Self { data, cursor: 0 }
    }

    pub fn bytes_left(&self) -> usize {
        self.data.len() - self.cursor
    }
}

/// Tells how many significant bytes a number takes.
///
/// This operation equates to counting how many zeroed bytes it has in its LSB side.
///
/// This is useful for compressing numbers in binary serialization formats.
fn significant_bytes_count(input: i128) -> usize {
    let mut dividend = input.saturating_abs();
    let mut counter = 0;

    while dividend > 256 {
        dividend >>= 8;
        counter += 1;
    }

    counter
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::core::value::Value;
    use crate::encoding::codecs::simple::SimpleScriptCodec;
    use crate::encoding::dec::Decode;
    use crate::encoding::enc::Encode;

    #[test]
    fn test_boolean_false_codec() {
        let value = Value::Boolean(false);
        let mut codec = SimpleScriptCodec::default();
        value.encode(&mut codec);
        let encoded = codec.data();
        let expected = Vec::<u8>::from([0]);

        assert_eq!(encoded, expected);

        codec = SimpleScriptCodec::from_data(expected);
        let decoded = Value::decode(&mut &mut codec).unwrap();

        assert_eq!(decoded, value);
    }

    #[test]
    fn test_boolean_true_codec() {
        let value = Value::Boolean(true);
        let mut codec = SimpleScriptCodec::default();
        value.encode(&mut codec);
        let encoded = codec.data();
        let expected = Vec::<u8>::from([1]);

        assert_eq!(encoded, expected);

        codec = SimpleScriptCodec::from_data(expected);
        let decoded = Value::decode(&mut &mut codec).unwrap();

        assert_eq!(decoded, value);
    }

    #[test]
    fn test_float_codec() {
        let value = Value::Float(3.14);
        let mut codec = SimpleScriptCodec::default();
        value.encode(&mut codec);
        let encoded = codec.data();
        let expected = Vec::<u8>::from([2, 31, 133, 235, 81, 184, 30, 9, 64]);

        assert_eq!(encoded, expected);

        codec = SimpleScriptCodec::from_data(expected);
        let decoded = Value::decode(&mut &mut codec).unwrap();

        assert_eq!(decoded, value);
    }

    #[test]
    fn test_integer_small_codec() {
        let value = Value::Integer(255);
        let mut codec = SimpleScriptCodec::default();
        value.encode(&mut codec);
        let encoded = codec.data();
        let expected = Vec::<u8>::from([3, 255]);

        assert_eq!(encoded, expected);

        codec = SimpleScriptCodec::from_data(expected);
        let decoded = Value::decode(&mut &mut codec).unwrap();

        assert_eq!(decoded, value);
    }

    #[test]
    fn test_integer_big_codec() {
        let value = Value::Integer(999999999999999999);
        let mut codec = SimpleScriptCodec::default();
        value.encode(&mut codec);
        let encoded = codec.data();
        let expected = Vec::<u8>::from([10, 255, 255, 99, 167, 179, 182, 224, 13]);

        assert_eq!(encoded, expected);

        codec = SimpleScriptCodec::from_data(expected);
        let decoded = Value::decode(&mut &mut codec).unwrap();

        assert_eq!(decoded, value);
    }

    #[test]
    fn test_integer_max_codec() {
        let value = Value::Integer(i128::MAX);
        let mut codec = SimpleScriptCodec::default();
        value.encode(&mut codec);
        let encoded = codec.data();
        let expected = Vec::<u8>::from([
            18, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 127,
        ]);

        assert_eq!(encoded, expected);

        codec = SimpleScriptCodec::from_data(expected);
        let decoded = Value::decode(&mut &mut codec).unwrap();

        assert_eq!(decoded, value);
    }

    #[test]
    fn test_integer_min_codec() {
        let value = Value::Integer(i128::MIN);
        let mut codec = SimpleScriptCodec::default();
        value.encode(&mut codec);
        let encoded = codec.data();
        let expected = Vec::<u8>::from([18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128]);

        assert_eq!(encoded, expected);

        codec = SimpleScriptCodec::from_data(expected);
        let decoded = Value::decode(&mut &mut codec).unwrap();

        assert_eq!(decoded, value);
    }

    #[test]
    fn test_string_empty_codec() {
        let value = Value::String("".into());
        let mut codec = SimpleScriptCodec::default();
        value.encode(&mut codec);
        let encoded = codec.data();
        let expected = Vec::<u8>::from([19]);

        assert_eq!(encoded, expected);

        codec = SimpleScriptCodec::from_data(expected);
        let decoded = Value::decode(&mut &mut codec).unwrap();

        assert_eq!(decoded, value);
    }

    #[test]
    fn test_string_regular_codec() {
        let value = Value::String("Hello, World!".into());
        let mut codec = SimpleScriptCodec::default();
        value.encode(&mut codec);
        let encoded = codec.data();
        let expected = Vec::<u8>::from([
            20, 13, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33,
        ]);

        assert_eq!(encoded, expected);

        codec = SimpleScriptCodec::from_data(expected);
        let decoded = Value::decode(&mut &mut codec).unwrap();

        assert_eq!(decoded, value);
    }

    #[test]
    fn test_string_massive_codec() {
        let value = Value::String(
            r#"
En un lugar de la Mancha, de cuyo nombre no quiero acordarme, no ha mucho tiempo que vivía un
hidalgo de los de lanza en astillero, adarga antigua, rocín flaco y galgo corredor. Una olla de algo
más vaca que carnero, salpicón las más noches, duelos y quebrantos los sábados, lentejas los
viernes, algún palomino de añadidura los domingos, consumían las tres partes de su hacienda. El
resto della concluían sayo de velarte, calzas de velludo para las fiestas con sus pantuflos de lo
mismo, los días de entre semana se honraba con su vellori de lo más fino. Tenía en su casa una ama
que pasaba de los cuarenta, y una sobrina que no llegaba a los veinte, y un mozo de campo y plaza,
que así ensillaba el rocín como tomaba la podadera. Frisaba la edad de nuestro hidalgo con los
cincuenta años, era de complexión recia, seco de carnes, enjuto de rostro; gran madrugador y amigo
de la caza. Quieren decir que tenía el sobrenombre de Quijada o Quesada (que en esto hay alguna
diferencia en los autores que deste caso escriben), aunque por conjeturas verosímiles se deja
entender que se llama Quijana; pero esto importa poco a nuestro cuento; basta que en la narración
dél no se salga un punto de la verdad."#
                .into(),
        );
        let mut codec = SimpleScriptCodec::default();
        value.encode(&mut codec);
        let encoded = codec.data();
        let expected = Vec::<u8>::from([
            21, 198, 4, 10, 69, 110, 32, 117, 110, 32, 108, 117, 103, 97, 114, 32, 100, 101, 32,
            108, 97, 32, 77, 97, 110, 99, 104, 97, 44, 32, 100, 101, 32, 99, 117, 121, 111, 32,
            110, 111, 109, 98, 114, 101, 32, 110, 111, 32, 113, 117, 105, 101, 114, 111, 32, 97,
            99, 111, 114, 100, 97, 114, 109, 101, 44, 32, 110, 111, 32, 104, 97, 32, 109, 117, 99,
            104, 111, 32, 116, 105, 101, 109, 112, 111, 32, 113, 117, 101, 32, 118, 105, 118, 195,
            173, 97, 32, 117, 110, 10, 104, 105, 100, 97, 108, 103, 111, 32, 100, 101, 32, 108,
            111, 115, 32, 100, 101, 32, 108, 97, 110, 122, 97, 32, 101, 110, 32, 97, 115, 116, 105,
            108, 108, 101, 114, 111, 44, 32, 97, 100, 97, 114, 103, 97, 32, 97, 110, 116, 105, 103,
            117, 97, 44, 32, 114, 111, 99, 195, 173, 110, 32, 102, 108, 97, 99, 111, 32, 121, 32,
            103, 97, 108, 103, 111, 32, 99, 111, 114, 114, 101, 100, 111, 114, 46, 32, 85, 110, 97,
            32, 111, 108, 108, 97, 32, 100, 101, 32, 97, 108, 103, 111, 10, 109, 195, 161, 115, 32,
            118, 97, 99, 97, 32, 113, 117, 101, 32, 99, 97, 114, 110, 101, 114, 111, 44, 32, 115,
            97, 108, 112, 105, 99, 195, 179, 110, 32, 108, 97, 115, 32, 109, 195, 161, 115, 32,
            110, 111, 99, 104, 101, 115, 44, 32, 100, 117, 101, 108, 111, 115, 32, 121, 32, 113,
            117, 101, 98, 114, 97, 110, 116, 111, 115, 32, 108, 111, 115, 32, 115, 195, 161, 98,
            97, 100, 111, 115, 44, 32, 108, 101, 110, 116, 101, 106, 97, 115, 32, 108, 111, 115,
            10, 118, 105, 101, 114, 110, 101, 115, 44, 32, 97, 108, 103, 195, 186, 110, 32, 112,
            97, 108, 111, 109, 105, 110, 111, 32, 100, 101, 32, 97, 195, 177, 97, 100, 105, 100,
            117, 114, 97, 32, 108, 111, 115, 32, 100, 111, 109, 105, 110, 103, 111, 115, 44, 32,
            99, 111, 110, 115, 117, 109, 195, 173, 97, 110, 32, 108, 97, 115, 32, 116, 114, 101,
            115, 32, 112, 97, 114, 116, 101, 115, 32, 100, 101, 32, 115, 117, 32, 104, 97, 99, 105,
            101, 110, 100, 97, 46, 32, 69, 108, 10, 114, 101, 115, 116, 111, 32, 100, 101, 108,
            108, 97, 32, 99, 111, 110, 99, 108, 117, 195, 173, 97, 110, 32, 115, 97, 121, 111, 32,
            100, 101, 32, 118, 101, 108, 97, 114, 116, 101, 44, 32, 99, 97, 108, 122, 97, 115, 32,
            100, 101, 32, 118, 101, 108, 108, 117, 100, 111, 32, 112, 97, 114, 97, 32, 108, 97,
            115, 32, 102, 105, 101, 115, 116, 97, 115, 32, 99, 111, 110, 32, 115, 117, 115, 32,
            112, 97, 110, 116, 117, 102, 108, 111, 115, 32, 100, 101, 32, 108, 111, 10, 109, 105,
            115, 109, 111, 44, 32, 108, 111, 115, 32, 100, 195, 173, 97, 115, 32, 100, 101, 32,
            101, 110, 116, 114, 101, 32, 115, 101, 109, 97, 110, 97, 32, 115, 101, 32, 104, 111,
            110, 114, 97, 98, 97, 32, 99, 111, 110, 32, 115, 117, 32, 118, 101, 108, 108, 111, 114,
            105, 32, 100, 101, 32, 108, 111, 32, 109, 195, 161, 115, 32, 102, 105, 110, 111, 46,
            32, 84, 101, 110, 195, 173, 97, 32, 101, 110, 32, 115, 117, 32, 99, 97, 115, 97, 32,
            117, 110, 97, 32, 97, 109, 97, 10, 113, 117, 101, 32, 112, 97, 115, 97, 98, 97, 32,
            100, 101, 32, 108, 111, 115, 32, 99, 117, 97, 114, 101, 110, 116, 97, 44, 32, 121, 32,
            117, 110, 97, 32, 115, 111, 98, 114, 105, 110, 97, 32, 113, 117, 101, 32, 110, 111, 32,
            108, 108, 101, 103, 97, 98, 97, 32, 97, 32, 108, 111, 115, 32, 118, 101, 105, 110, 116,
            101, 44, 32, 121, 32, 117, 110, 32, 109, 111, 122, 111, 32, 100, 101, 32, 99, 97, 109,
            112, 111, 32, 121, 32, 112, 108, 97, 122, 97, 44, 10, 113, 117, 101, 32, 97, 115, 195,
            173, 32, 101, 110, 115, 105, 108, 108, 97, 98, 97, 32, 101, 108, 32, 114, 111, 99, 195,
            173, 110, 32, 99, 111, 109, 111, 32, 116, 111, 109, 97, 98, 97, 32, 108, 97, 32, 112,
            111, 100, 97, 100, 101, 114, 97, 46, 32, 70, 114, 105, 115, 97, 98, 97, 32, 108, 97,
            32, 101, 100, 97, 100, 32, 100, 101, 32, 110, 117, 101, 115, 116, 114, 111, 32, 104,
            105, 100, 97, 108, 103, 111, 32, 99, 111, 110, 32, 108, 111, 115, 10, 99, 105, 110, 99,
            117, 101, 110, 116, 97, 32, 97, 195, 177, 111, 115, 44, 32, 101, 114, 97, 32, 100, 101,
            32, 99, 111, 109, 112, 108, 101, 120, 105, 195, 179, 110, 32, 114, 101, 99, 105, 97,
            44, 32, 115, 101, 99, 111, 32, 100, 101, 32, 99, 97, 114, 110, 101, 115, 44, 32, 101,
            110, 106, 117, 116, 111, 32, 100, 101, 32, 114, 111, 115, 116, 114, 111, 59, 32, 103,
            114, 97, 110, 32, 109, 97, 100, 114, 117, 103, 97, 100, 111, 114, 32, 121, 32, 97, 109,
            105, 103, 111, 10, 100, 101, 32, 108, 97, 32, 99, 97, 122, 97, 46, 32, 81, 117, 105,
            101, 114, 101, 110, 32, 100, 101, 99, 105, 114, 32, 113, 117, 101, 32, 116, 101, 110,
            195, 173, 97, 32, 101, 108, 32, 115, 111, 98, 114, 101, 110, 111, 109, 98, 114, 101,
            32, 100, 101, 32, 81, 117, 105, 106, 97, 100, 97, 32, 111, 32, 81, 117, 101, 115, 97,
            100, 97, 32, 40, 113, 117, 101, 32, 101, 110, 32, 101, 115, 116, 111, 32, 104, 97, 121,
            32, 97, 108, 103, 117, 110, 97, 10, 100, 105, 102, 101, 114, 101, 110, 99, 105, 97, 32,
            101, 110, 32, 108, 111, 115, 32, 97, 117, 116, 111, 114, 101, 115, 32, 113, 117, 101,
            32, 100, 101, 115, 116, 101, 32, 99, 97, 115, 111, 32, 101, 115, 99, 114, 105, 98, 101,
            110, 41, 44, 32, 97, 117, 110, 113, 117, 101, 32, 112, 111, 114, 32, 99, 111, 110, 106,
            101, 116, 117, 114, 97, 115, 32, 118, 101, 114, 111, 115, 195, 173, 109, 105, 108, 101,
            115, 32, 115, 101, 32, 100, 101, 106, 97, 10, 101, 110, 116, 101, 110, 100, 101, 114,
            32, 113, 117, 101, 32, 115, 101, 32, 108, 108, 97, 109, 97, 32, 81, 117, 105, 106, 97,
            110, 97, 59, 32, 112, 101, 114, 111, 32, 101, 115, 116, 111, 32, 105, 109, 112, 111,
            114, 116, 97, 32, 112, 111, 99, 111, 32, 97, 32, 110, 117, 101, 115, 116, 114, 111, 32,
            99, 117, 101, 110, 116, 111, 59, 32, 98, 97, 115, 116, 97, 32, 113, 117, 101, 32, 101,
            110, 32, 108, 97, 32, 110, 97, 114, 114, 97, 99, 105, 195, 179, 110, 10, 100, 195, 169,
            108, 32, 110, 111, 32, 115, 101, 32, 115, 97, 108, 103, 97, 32, 117, 110, 32, 112, 117,
            110, 116, 111, 32, 100, 101, 32, 108, 97, 32, 118, 101, 114, 100, 97, 100, 46,
        ]);

        assert_eq!(encoded, expected);

        codec = SimpleScriptCodec::from_data(expected);
        let decoded = Value::decode(&mut &mut codec).unwrap();

        assert_eq!(decoded, value);
    }
}
