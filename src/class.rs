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
    todo!()
  }
}