use crate::common;
use crate::error;
use std::io::Read;

#[derive(Debug)]
pub struct Interfaces {
  count: u16,
  interfaces: Vec<Interface>
}

impl common::FromReader<Interfaces> for Interfaces {
  fn from_reader(reader: &mut dyn Read) -> error::Result<Interfaces> {
    let count = common::read_be_u16_from_reader(reader)?;
    let mut interfaces = vec![];
    for _i in 1..count {
      let name_index = common::read_be_u16_from_reader(reader)?;
      interfaces.push(Interface { name_index });
    }
    Ok(Interfaces { count, interfaces })
  }
}

#[derive(Debug)]
pub struct Interface {
  name_index: u16
}