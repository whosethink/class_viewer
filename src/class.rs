use crate::common;
use crate::magic;
use crate::version;
use crate::constant;
use crate::access;
use crate::this;
use crate::superior;
use crate::interface;
use crate::field;
use crate::method;
use crate::attribute;
use std::io::Read;
use crate::access::ClassAccessFlag;
use std::borrow::Borrow;
use crate::magic::Magic;
use std::ops::Add;

#[derive(Debug)]
pub struct Class {
  magic: magic::Magic,
  version: version::Version,
  constants: constant::Constants,
  access: access::ClassAccessFlag,
  this: this::ThisClass,
  superior: superior::SuperClass,
  interfaces: interface::Interfaces,
  fields: field::Fields,
  methods: method::Methods,
  attributes: attribute::Attributes
}

impl common::FromReader<Class> for Class {
  fn from_reader(reader: &mut dyn Read) -> crate::error::Result<Class> {
    let magic = magic::Magic::from_reader(reader)?;
    let version = version::Version::from_reader(reader)?;
    let constants = constant::Constants::from_reader(reader)?;
    let access = access::ClassAccessFlag::from_reader(reader)?;
    let this = this::ThisClass::from_reader(reader)?;
    let superior = superior::SuperClass::from_reader(reader)?;
    let interfaces = interface::Interfaces::from_reader(reader)?;
    let fields = field::Fields::from_reader(reader)?;
    let methods = method::Methods::from_reader(reader)?;
    let attributes = attribute::Attributes::from_reader(reader)?;
    Ok(Class { magic, version, constants, access, this, superior, interfaces, fields, methods, attributes })
  }

}

impl ToString for Class {
  fn to_string(&self) -> String {
    let mut result = vec![];
    result.push(self.magic.to_string());
    result.push(self.version.to_string());
    result.push(self.constants.to_string());
    return result.join("\n");
  }
}

impl Class {
  fn get_magic(&self) -> &Magic {
    &self.magic
  }
}