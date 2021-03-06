use crate::common;
use crate::error;
use std::io::Read;

#[derive(Debug)]
pub struct Attributes {
  count: u16,
  attributes: Vec<Attribute>
}

impl common::FromReader<Attributes> for Attributes {
  fn from_reader(reader: &mut dyn Read) -> error::Result<Attributes> {
    let count = common::read_be_u16_from_reader(reader)?;
    let mut attributes = vec![];
    for _i in 0..count {
      let attribute = Attribute::from_reader(reader)?;
      attributes.push(attribute);
    }
    Ok(Attributes { count, attributes })
  }
}

#[derive(Debug)]
pub struct Attribute {
  name_index: u16,
  length: u32,
  bytes: Vec<u8>
}

impl common::FromReader<Attribute> for Attribute {
  fn from_reader(reader: &mut dyn Read) -> error::Result<Attribute> {
    let name_index = common::read_be_u16_from_reader(reader)?;
    let length = common::read_be_u32_from_reader(reader)?;
    let bytes = common::read_vec_u8_from_reader(reader, length as usize)?;
    Ok(Attribute { name_index, length, bytes })
  }
}