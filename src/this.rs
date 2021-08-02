use std::io::Read;
use crate::common;
use crate::error;

pub struct ThisClass {
  name_index: u16
}

impl common::FromReader<ThisClass> for ThisClass {
  fn from_reader(reader: &mut dyn Read) -> error::Result<ThisClass> {
    let name_index = common::read_be_u16_from_reader(reader)?;
    Ok(ThisClass { name_index })
  }
}