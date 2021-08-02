use std::io::Read;
use crate::common;
use crate::error;

pub struct SuperClass {
  name_index: u16
}

impl common::FromReader<SuperClass> for SuperClass {
  fn from_reader(reader: &mut dyn Read) -> error::Result<SuperClass> {
    let name_index = common::read_be_u16_from_reader(reader)?;
    Ok(SuperClass{ name_index })
  }
}