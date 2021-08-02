use crate::common;
use crate::error;
use crate::attribute;
use std::io::Read;

pub struct Fields {
  count: u16,
  fields: Vec<Field>
}

pub struct Field {
  access: u16,
  name_index: u16,
  descriptor_index: u16,
  attributes: attribute::Attributes
}

impl common::FromReader<Fields> for Fields {
  fn from_reader(reader: &mut dyn Read) -> error::Result<Fields> {
    let count = common::read_be_u16_from_reader(reader)?;
    let mut fields = Vec::with_capacity(count as usize);
    for _i in 0..count {
      let field = Field::from_reader(reader)?;
      fields.push(field);
    }
    Ok(Fields { count, fields })
  }
}

impl common::FromReader<Field> for Field {
  fn from_reader(reader: &mut dyn Read) -> error::Result<Field> {
    let access = common::read_be_u16_from_reader(reader)?;
    let name_index = common::read_be_u16_from_reader(reader)?;
    let descriptor_index = common::read_be_u16_from_reader(reader)?;
    let attributes = attribute::Attributes::from_reader(reader)?;
    Ok(Field { access, name_index, descriptor_index, attributes })
  }
}