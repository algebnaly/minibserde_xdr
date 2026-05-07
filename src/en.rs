use std::io::Write;

use minibserde::{
    Discriminant, Encode, Encoder, MapEncoder, SeqEncoder, StructEncoder, TupleEncoder,
};

use crate::error::Error;

pub fn write_value<T: Encode>(value: &T, writer: &mut impl Write) -> Result<(), Error> {
    let mut encoder = XDREncoder::new(writer);
    value.encode(&mut encoder)
}

pub fn to_bytes<T: Encode>(value: &T) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    write_value(value, &mut buf)?;
    Ok(buf)
}

pub struct XDREncoder<W>
where
    W: Write,
{
    writer: W,
}

impl<W> XDREncoder<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        XDREncoder { writer }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

fn write_pad(writer: &mut impl Write, len: usize) -> Result<(), Error> {
    let padding = len.wrapping_neg() & 3; // the same as (4 - (len % 4)) % 4
    if padding > 0 {
        writer.write_all(&[0u8; 3][..padding])?;
    }
    Ok(())
}

impl<W> Encoder for &mut XDREncoder<W>
where
    W: Write,
{
    type Error = Error;
    type SeqEncoder = Self;
    type MapEncoder = Self;
    type StructEncoder = Self;
    type TupleEncoder = Self;
    fn encode_unit(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn encode_bool(self, value: bool) -> Result<(), Self::Error> {
        let val: u32 = if value { 1 } else { 0 };
        self.writer.write_all(&val.to_be_bytes())?;
        Ok(())
    }

    fn encode_u8(self, value: u8) -> Result<(), Self::Error> {
        self.writer.write_all(&(value as u32).to_be_bytes())?;
        Ok(())
    }

    fn encode_u16(self, value: u16) -> Result<(), Self::Error> {
        self.writer.write_all(&(value as u32).to_be_bytes())?;
        Ok(())
    }

    fn encode_u32(self, value: u32) -> Result<(), Self::Error> {
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    fn encode_u64(self, value: u64) -> Result<(), Self::Error> {
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    fn encode_u128(self, value: u128) -> Result<(), Self::Error> {
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    fn encode_i8(self, value: i8) -> Result<(), Self::Error> {
        self.writer.write_all(&(value as u32).to_be_bytes())?;
        Ok(())
    }

    fn encode_i16(self, value: i16) -> Result<(), Self::Error> {
        self.writer.write_all(&(value as u32).to_be_bytes())?;
        Ok(())
    }

    fn encode_i32(self, value: i32) -> Result<(), Self::Error> {
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    fn encode_i64(self, value: i64) -> Result<(), Self::Error> {
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    fn encode_i128(self, value: i128) -> Result<(), Self::Error> {
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    fn encode_f32(self, value: f32) -> Result<(), Self::Error> {
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    fn encode_f64(self, value: f64) -> Result<(), Self::Error> {
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    fn encode_bytes(self, value: &[u8]) -> Result<(), Self::Error> {
        let len = value.len() as u32;
        self.writer.write_all(&len.to_be_bytes())?;
        self.writer.write_all(value)?;
        write_pad(&mut self.writer, value.len())?;
        Ok(())
    }

    fn encode_string(self, value: &str) -> Result<(), Self::Error> {
        self.encode_bytes(value.as_bytes())
    }

    fn encode_byte_array<const N: usize>(self, value: &[u8; N]) -> Result<(), Self::Error> {
        self.writer.write_all(value)?;
        write_pad(&mut self.writer, N)?;
        Ok(())
    }

    fn encode_option<T: minibserde::Encode>(self, value: Option<&T>) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_u32(1)?;
            value.encode(self)?;
        } else {
            self.encode_u32(0)?;
        }
        Ok(())
    }

    fn encode_map(self, len: usize) -> Result<Self::MapEncoder, Self::Error> {
        self.encode_u32(len as u32)?;
        Ok(self)
    }

    fn encode_seq(self, len: usize) -> Result<Self::SeqEncoder, Self::Error> {
        self.encode_u32(len as u32)?;
        Ok(self)
    }

    fn encode_struct(self, _len: usize) -> Result<Self::StructEncoder, Self::Error> {
        Ok(self)
    }

    fn encode_tuple(self, _len: usize) -> Result<Self::TupleEncoder, Self::Error> {
        Ok(self)
    }

    fn encode_variant<T: minibserde::Encode>(
        self,
        discriminant: minibserde::Discriminant,
        value: &T,
    ) -> Result<(), Self::Error> {
        match discriminant {
            Discriminant::U8(d) => self.encode_u8(d)?,
            Discriminant::U16(d) => self.encode_u16(d)?,
            Discriminant::U32(d) => self.encode_u32(d)?,
            Discriminant::U64(d) => self.encode_u64(d)?,
            Discriminant::U128(d) => self.encode_u128(d)?,
            Discriminant::USize(d) => self.encode_u32(d as u32)?,
            Discriminant::I8(d) => self.encode_i8(d)?,
            Discriminant::I16(d) => self.encode_i16(d)?,
            Discriminant::I32(d) => self.encode_i32(d)?,
            Discriminant::I64(d) => self.encode_i64(d)?,
            Discriminant::I128(d) => self.encode_i128(d)?,
            Discriminant::ISize(d) => self.encode_i32(d as i32)?,
        }
        value.encode(self)?;
        Ok(())
    }
}

impl<W> MapEncoder for &mut XDREncoder<W>
where
    W: Write,
{
    type Error = Error;
    fn encode_key<K: minibserde::Encode>(&mut self, key: &K) -> Result<(), Self::Error> {
        key.encode(&mut **self)
    }

    fn encode_value<V: minibserde::Encode>(&mut self, value: &V) -> Result<(), Self::Error> {
        value.encode(&mut **self)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<W> TupleEncoder for &mut XDREncoder<W>
where
    W: Write,
{
    type Error = Error;
    fn encode_element<T: minibserde::Encode>(&mut self, element: &T) -> Result<(), Self::Error> {
        element.encode(&mut **self)
    }
    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<W> StructEncoder for &mut XDREncoder<W>
where
    W: Write,
{
    type Error = Error;
    fn encode_field<T: minibserde::Encode>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.encode(&mut **self)
    }
    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<W> SeqEncoder for &mut XDREncoder<W>
where
    W: Write,
{
    type Error = Error;
    fn encode_element<T: minibserde::Encode>(&mut self, element: &T) -> Result<(), Self::Error> {
        element.encode(&mut **self)
    }
    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_u8_42() {
        let mut encoder = XDREncoder::new(Vec::new());
        encoder.encode_u8(42).unwrap();
        assert_eq!(encoder.into_inner(), vec![0, 0, 0, 42]);
    }

    #[test]
    fn test_encode_byte_array_3() {
        let mut encoder = XDREncoder::new(Vec::new());
        encoder.encode_byte_array(&[1, 2, 3]).unwrap();
        // 3 bytes + 1 byte padding to align to decoder4
        assert_eq!(encoder.into_inner(), vec![1, 2, 3, 0]);
    }

    #[test]
    fn test_encode_bool_true() {
        let mut encoder = XDREncoder::new(Vec::new());
        encoder.encode_bool(true).unwrap();
        assert_eq!(encoder.into_inner(), vec![0, 0, 0, 1]);
    }

    #[test]
    fn test_encode_bool_false() {
        let mut encoder = XDREncoder::new(Vec::new());
        encoder.encode_bool(false).unwrap();
        assert_eq!(encoder.into_inner(), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_encode_bytes_aligned() {
        let mut encoder = XDREncoder::new(Vec::new());
        encoder.encode_bytes(&[1, 2, 3, 4]).unwrap();
        // length prefix (4) + data — length is multiple of 4, no padding
        assert_eq!(encoder.into_inner(), vec![0, 0, 0, 4, 1, 2, 3, 4]);
    }

    #[test]
    fn test_encode_bytes_unaligned() {
        let mut encoder = XDREncoder::new(Vec::new());
        encoder.encode_bytes(&[1, 2, 3]).unwrap();
        // length prefix (3) + data [1,2,3] + 1 byte padding
        assert_eq!(encoder.into_inner(), vec![0, 0, 0, 3, 1, 2, 3, 0]);
    }

    #[test]
    fn test_encode_empty_bytes() {
        let mut encoder = XDREncoder::new(Vec::new());
        encoder.encode_bytes(&[]).unwrap();
        // length prefix (0), no padding for 0-length
        assert_eq!(encoder.into_inner(), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_encode_string() {
        let mut encoder = XDREncoder::new(Vec::new());
        encoder.encode_string("abc").unwrap();
        // length prefix (3) + "abc" + 1 byte padding
        assert_eq!(encoder.into_inner(), vec![0, 0, 0, 3, b'a', b'b', b'c', 0]);
    }

    #[test]
    fn test_encode_u32_big_endian() {
        let mut encoder = XDREncoder::new(Vec::new());
        encoder.encode_u32(0x12345678).unwrap();
        assert_eq!(encoder.into_inner(), vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_encode_enum() {
        #[derive(Encode)]
        #[allow(unused)]
        #[repr(u32)]
        enum MyEnum {
            Variant1 = 1,
            Variant2 = 16,
            #[minibserde(catch_all)]
            Variant3(u32) = 8,
        }
        let mut encoder = XDREncoder::new(Vec::new());
        let value = MyEnum::Variant1;
        let value2 = MyEnum::Variant3(7);
        value.encode(&mut encoder).unwrap();
        value2.encode(&mut encoder).unwrap();
        assert_eq!(encoder.into_inner(), vec![0, 0, 0, 1, 0, 0, 0, 7]);
    }
}
