use crate::common;
use crate::error;
use crate::attribute;
use std::io::Read;

#[derive(Debug)]
pub struct Methods {
  count: u16,
  methods: Vec<Method>
}

impl common::FromReader<Methods> for Methods {
  fn from_reader(reader: &mut dyn Read) -> error::Result<Methods> {
    let count = common::read_be_u16_from_reader(reader)?;
    let mut methods = Vec::with_capacity(count as usize);
    for _i in 0..count {
      let method = Method::from_reader(reader)?;
      methods.push(method);
    }
    Ok(Methods { count, methods })
  }
}

#[derive(Debug)]
pub struct Method {
  access: u16,
  name_index: u16,
  descriptor_index: u16,
  attributes: attribute::Attributes
}

impl common::FromReader<Method> for Method {
  fn from_reader(reader: &mut dyn Read) -> error::Result<Method> {
    let access = common::read_be_u16_from_reader(reader)?;
    let name_index = common::read_be_u16_from_reader(reader)?;
    let descriptor_index = common::read_be_u16_from_reader(reader)?;
    let attributes = attribute::Attributes::from_reader(reader)?;
    Ok(Method { access, name_index, descriptor_index, attributes })
  }
}