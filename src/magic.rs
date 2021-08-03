use std::io::Read;
use crate::error;
use crate::common;
use std::ops::Add;

#[derive(Debug)]
pub struct Magic {
  number: u32
}

impl Magic {
  pub fn get_number(&self) -> u32 {
    self.number
  }
}

impl ToString for Magic {
  fn to_string(&self) -> String {
    format!("Magic: {}", self.number)
  }
}

impl common::FromReader<Magic> for Magic {
  fn from_reader(reader: &mut dyn Read) -> error::Result<Magic> {
    let number = common::read_be_u32_from_reader(reader)?;
    Ok(Magic {
      number
    })
  }
}