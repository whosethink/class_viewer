use std::io::Read;
use crate::common;
use crate::error;

#[derive(Debug)]
pub struct Version {
  minor: u16,
  major: u16
}

impl common::FromReader<Version> for Version {
  fn from_reader(reader: &mut dyn Read) -> error::Result<Version> {
    let major = common::read_be_u16_from_reader(reader)?;
    let minor = common::read_be_u16_from_reader(reader)?;
    Ok(Version { minor, major })
  }
}