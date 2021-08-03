use std::io::Read;
use crate::error;

pub trait FromReader<T> {
  fn from_reader(reader: &mut dyn Read) -> error::Result<T>;
}

pub fn read_be_u8_from_reader(reader: &mut dyn Read) -> Result<u8, error::SunshineError> {
  let mut buffer: [u8; 1] = [0; 1];
  reader.read_exact(&mut buffer)?;
  return Ok(u8::from_be_bytes(buffer));
}

pub fn read_be_u16_from_reader(reader: &mut dyn Read) -> Result<u16, error::SunshineError> {
  let mut buffer: [u8; 2] = [0; 2];
  reader.read_exact(&mut buffer)?;
  return Ok(u16::from_be_bytes(buffer));
}

pub fn read_be_u32_from_reader(reader: &mut dyn Read) -> Result<u32, error::SunshineError> {
  let mut buffer: [u8; 4] = [0; 4];
  reader.read_exact(&mut buffer)?;
  return Ok(u32::from_be_bytes(buffer));
}

pub fn read_vec_u8_from_reader(reader: &mut dyn Read, length: usize) -> Result<Vec<u8>, error::SunshineError> {
  let mut buffer: Vec<u8> = vec![0; length];
  reader.read_exact(&mut buffer)?;
  return Ok(buffer);
}

pub fn read_string_from_reader(reader: &mut dyn Read, length: usize) -> Result<String, error::SunshineError> {
  let bytes_vec = read_vec_u8_from_reader(reader, length)?;
  return Ok(String::from_utf8(bytes_vec)?);
}