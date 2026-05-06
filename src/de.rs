use std::io::Read;

use binserde::{Decode, Decoder, EnumDecoder, MapDecoder, SeqDecoder, StructDecoder, TupleDecoder};

use crate::error::Error;

pub fn read_value<R: Read, T: Decode>(reader: &mut R) -> Result<T, Error> {
    let mut decoder = XDRDecoder::new(reader);
    T::decode(&mut decoder)
}

pub fn from_bytes<T: Decode>(bytes: &[u8]) -> Result<T, Error> {
    let mut reader = std::io::Cursor::new(bytes);
    let value = read_value(&mut reader)?;
    if reader.position() != bytes.len() as u64 {
        Err(Error::TrailingBytes)
    } else {
        Ok(value)
    }
}

pub fn decode_len<T: Decode>(bytes: &[u8]) -> Result<usize, Error> {
    let mut reader = std::io::Cursor::new(bytes);
    let _: T = read_value(&mut reader)?;
    Ok(reader.position() as usize)
}

fn read_pad(reader: &mut impl Read, len: usize) -> Result<(), Error> {
    let padding = len.wrapping_neg() & 3; // the same as (4 - (len % 4)) % 4
    if padding > 0 {
        let mut buf = [0u8; 3];
        reader.read_exact(&mut buf[..padding])?;
        if buf != [0; 3] {
            return Err(Error::NonZeroPadding);
        }
    }
    Ok(())
}

pub struct XDRDecoder<R>
where
    R: Read,
{
    reader: R,
}

impl<R: Read> XDRDecoder<R> {
    pub fn new(reader: R) -> Self {
        XDRDecoder { reader }
    }
}

impl<R: Read> Decoder for &mut XDRDecoder<R> {
    type Error = Error;
    type EnumDecoder = Self;
    type MapDecoder = Self;
    type SeqDecoder = Self;
    type StructDecoder = Self;
    type TupleDecoder = Self;

    fn decode_unit(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn decode_bool(self) -> Result<bool, Self::Error> {
        let v = self.decode_u32()?;
        match v {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::InvalidValue),
        }
    }

    fn decode_u8(self) -> Result<u8, Self::Error> {
        self.decode_u32()?.try_into().map_err(|_| Error::OutOfRange)
    }

    fn decode_u16(self) -> Result<u16, Self::Error> {
        self.decode_u32()?.try_into().map_err(|_| Error::OutOfRange)
    }

    fn decode_u32(self) -> Result<u32, Self::Error> {
        let mut buf = [0; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    fn decode_u64(self) -> Result<u64, Self::Error> {
        let mut buf = [0; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    fn decode_u128(self) -> Result<u128, Self::Error> {
        let mut buf = [0; 16];
        self.reader.read_exact(&mut buf)?;
        Ok(u128::from_be_bytes(buf))
    }

    fn decode_i8(self) -> Result<i8, Self::Error> {
        let v = self.decode_u32()?;
        if v > 0xFF {
            return Err(Error::OutOfRange);
        }
        Ok(v as i8)
    }

    fn decode_i16(self) -> Result<i16, Self::Error> {
        let v = self.decode_u32()?;
        if v > 0xFF_FF {
            return Err(Error::OutOfRange);
        }
        Ok(v as i16)
    }

    fn decode_i32(self) -> Result<i32, Self::Error> {
        let mut buf = [0; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }

    fn decode_i64(self) -> Result<i64, Self::Error> {
        let mut buf = [0; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }

    fn decode_i128(self) -> Result<i128, Self::Error> {
        let mut buf = [0; 16];
        self.reader.read_exact(&mut buf)?;
        Ok(i128::from_be_bytes(buf))
    }

    fn decode_f32(self) -> Result<f32, Self::Error> {
        let mut buf = [0; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(f32::from_be_bytes(buf))
    }

    fn decode_f64(self) -> Result<f64, Self::Error> {
        let mut buf = [0; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(f64::from_be_bytes(buf))
    }

    fn decode_string(self) -> Result<String, Self::Error> {
        let len = self.decode_u32()?;
        let mut buf = vec![0; len as usize];
        self.reader.read_exact(&mut buf)?;
        read_pad(&mut self.reader, len as usize)?;
        String::from_utf8(buf).map_err(|_| Error::InvalidValue)
    }

    fn decode_bytes(self) -> Result<Vec<u8>, Self::Error> {
        let len = self.decode_u32()?;
        let mut buf = vec![0; len as usize];
        self.reader.read_exact(&mut buf)?;
        read_pad(&mut self.reader, len as usize)?;
        Ok(buf)
    }

    fn decode_byte_array<const N: usize>(self) -> Result<[u8; N], Self::Error> {
        let mut buf = [0; N];
        self.reader.read_exact(&mut buf)?;
        read_pad(&mut self.reader, N)?;
        Ok(buf)
    }

    fn decode_option<T: binserde::Decode>(self) -> Result<Option<T>, Self::Error> {
        let v = self.decode_bool()?;
        if v {
            Ok(Some(T::decode(self)?))
        } else {
            Ok(None)
        }
    }

    fn decode_map(self) -> Result<Self::MapDecoder, Self::Error> {
        Ok(self)
    }

    fn decode_seq(self) -> Result<Self::SeqDecoder, Self::Error> {
        Ok(self)
    }

    fn decode_struct(self, _len: usize) -> Result<Self::StructDecoder, Self::Error> {
        Ok(self)
    }

    fn decode_tuple(self, _len: usize) -> Result<Self::TupleDecoder, Self::Error> {
        Ok(self)
    }

    fn decode_variant(self) -> Result<Self::EnumDecoder, Self::Error> {
        Ok(self)
    }
}

impl<R: Read> EnumDecoder for &mut XDRDecoder<R> {
    type Error = Error;

    fn decode_discriminant_u8(&mut self) -> Result<u8, Self::Error> {
        self.decode_u8()
    }

    fn decode_discriminant_u16(&mut self) -> Result<u16, Self::Error> {
        self.decode_u16()
    }

    fn decode_discriminant_u32(&mut self) -> Result<u32, Self::Error> {
        self.decode_u32()
    }

    fn decode_discriminant_u64(&mut self) -> Result<u64, Self::Error> {
        self.decode_u64()
    }

    fn decode_discriminant_u128(&mut self) -> Result<u128, Self::Error> {
        self.decode_u128()
    }

    fn decode_discriminant_usize(&mut self) -> Result<usize, Self::Error> {
        self.decode_u32().map(|v| v as usize)
    }

    fn decode_discriminant_i8(&mut self) -> Result<i8, Self::Error> {
        self.decode_i8()
    }

    fn decode_discriminant_i16(&mut self) -> Result<i16, Self::Error> {
        self.decode_i16()
    }

    fn decode_discriminant_i32(&mut self) -> Result<i32, Self::Error> {
        self.decode_i32()
    }

    fn decode_discriminant_i64(&mut self) -> Result<i64, Self::Error> {
        self.decode_i64()
    }

    fn decode_discriminant_i128(&mut self) -> Result<i128, Self::Error> {
        self.decode_i128()
    }

    fn decode_discriminant_isize(&mut self) -> Result<isize, Self::Error> {
        self.decode_i32().map(|v| v as isize)
    }

    fn decode_field<T: Decode>(&mut self) -> Result<T, Self::Error> {
        T::decode(&mut **self)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_unknown_discriminant(&mut self, discriminant: impl std::fmt::Display) -> Self::Error {
        Error::Custom(format!("unknown variant discriminant: {}", discriminant))
    }
}

impl<R: Read> StructDecoder for &mut XDRDecoder<R> {
    type Error = Error;

    fn decode_field<T: Decode>(&mut self) -> Result<T, Self::Error> {
        T::decode(&mut **self)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<R: Read> SeqDecoder for &mut XDRDecoder<R> {
    type Error = Error;

    fn decode_len(&mut self) -> Result<usize, Self::Error> {
        let len = self.decode_u32()?;
        usize::try_from(len).map_err(|_| Error::OutOfRange)
    }

    fn decode_element<T: Decode>(&mut self) -> Result<T, Self::Error> {
        T::decode(&mut **self)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<R: Read> MapDecoder for &mut XDRDecoder<R> {
    type Error = Error;

    fn decode_length(&mut self) -> Result<usize, Self::Error> {
        let len = self.decode_u32()?;
        let len = usize::try_from(len).map_err(|_| Error::OutOfRange)?;
        Ok(len)
    }

    fn decode_key<T: Decode>(&mut self) -> Result<T, Self::Error> {
        T::decode(&mut **self)
    }

    fn decode_value<T: Decode>(&mut self) -> Result<T, Self::Error> {
        T::decode(&mut **self)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<R: Read> TupleDecoder for &mut XDRDecoder<R> {
    type Error = Error;

    fn decode_element<T: Decode>(&mut self) -> Result<T, Self::Error> {
        T::decode(&mut **self)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_u8() {
        let v: Vec<u8> = vec![0, 0, 0, 42];
        let value: u8 = read_value(&mut v.as_slice()).unwrap();
        assert_eq!(value, 42);
    }
}
