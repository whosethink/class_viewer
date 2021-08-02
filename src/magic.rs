use std::io::Read;
use crate::error;
use crate::common;

#[derive(Debug)]
pub struct Magic {
  number: u32
}

impl common::FromReader<Magic> for Magic {
  fn from_reader(reader: &mut dyn Read) -> error::Result<Magic> {
    let number = common::read_be_u32_from_reader(reader)?;
    Ok(Magic {
      number
    })
  }
}