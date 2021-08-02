use std::io::Read;
use crate::common;
use crate::error;

#[derive(Debug)]
pub struct Constants {
  count: u16,
  constants: Vec<ConstantEnum>
}

impl common::FromReader<Constants> for Constants {
  fn from_reader(reader: &mut dyn Read) -> error::Result<Constants> {
    let count = common::read_be_u16_from_reader(reader)?;
    let mut constants = Vec::with_capacity(
      (count - 1) as usize
    );
    for _i in 1..count {
      constants.push(Constants::read_constant_item(reader).unwrap());
    }
    Ok(
      Constants {
        count,
        constants
      }
    )
  }

}

impl Constants {
  fn read_constant_item(reader: &mut dyn Read) -> error::Result<ConstantEnum> {
    let tag = common::read_be_u8_from_reader(reader)?;
    match ConstantType::from(tag) {
      ConstantType::Class => Constants::read_constant_class(reader),
      ConstantType::FieldRef => Constants::read_constant_field_ref(reader),
      ConstantType::MethodRef => Constants::read_constant_method_ref(reader),
      ConstantType::InterfaceMethodRef => Constants::read_constant_interface_method_ref(reader),
      ConstantType::String => Constants::read_constant_string(reader),
      ConstantType::Integer => Constants::read_constant_integer(reader),
      ConstantType::Float => Constants::read_constant_float(reader),
      ConstantType::Long => Constants::read_constant_long(reader),
      ConstantType::Double => Constants::read_constant_double(reader),
      ConstantType::NameAndType => Constants::read_constant_name_and_type(reader),
      ConstantType::UTF8 => Constants::read_constant_utf8(reader),
      ConstantType::MethodHandle => Constants::read_constant_method_handle(reader),
      ConstantType::MethodType => Constants::read_constant_method_type(reader),
      ConstantType::InvokeDynamic => Constants::read_constant_invoke_dynamic(reader),
      _ => Err(error::SunshineError::MessageError(format!("unknown tag {}", tag)))
    }
  }

  fn read_constant_class(reader: &mut dyn Read) -> error::Result<ConstantEnum> {
    let index = common::read_be_u16_from_reader(reader)?;
    Ok(ConstantEnum::Class(index))
  }

  fn read_constant_field_ref(reader: &mut dyn Read) -> error::Result<ConstantEnum> {
    let class_index = common::read_be_u16_from_reader(reader)?;
    let name_and_type_index = common::read_be_u16_from_reader(reader)?;
    Ok(ConstantEnum::FieldRef(class_index, name_and_type_index))
  }

  fn read_constant_interface_method_ref(reader: &mut dyn Read) -> error::Result<ConstantEnum> {
    let class_index = common::read_be_u16_from_reader(reader)?;
    let name_and_type_index = common::read_be_u16_from_reader(reader)?;
    Ok(ConstantEnum::InterfaceMethodRef(class_index, name_and_type_index))
  }

  fn read_constant_method_ref(reader: &mut dyn Read) -> error::Result<ConstantEnum> {
    let class_index = common::read_be_u16_from_reader(reader)?;
    let name_and_type_index = common::read_be_u16_from_reader(reader)?;
    Ok(ConstantEnum::MethodRef(class_index, name_and_type_index))
  }

  fn read_constant_string(reader: &mut dyn Read) -> error::Result<ConstantEnum> {
    let string_index = common::read_be_u16_from_reader(reader)?;
    Ok(ConstantEnum::String(string_index))
  }

  fn read_constant_integer(reader: &mut dyn Read) -> error::Result<ConstantEnum> {
    let bytes = common::read_be_u32_from_reader(reader)?;
    Ok(ConstantEnum::Integer(bytes))
  }

  fn read_constant_float(reader: &mut dyn Read) -> error::Result<ConstantEnum> {
    let bytes = common::read_be_u32_from_reader(reader)?;
    Ok(ConstantEnum::Float(bytes))
  }

  fn read_constant_long(reader: &mut dyn Read) -> error::Result<ConstantEnum> {
    let high = common::read_be_u32_from_reader(reader)?;
    let low = common::read_be_u32_from_reader(reader)?;
    Ok(ConstantEnum::Long(high, low))
  }

  fn read_constant_double(reader: &mut dyn Read) -> error::Result<ConstantEnum> {
    let high = common::read_be_u32_from_reader(reader)?;
    let low = common::read_be_u32_from_reader(reader)?;
    Ok(ConstantEnum::Double(high, low))
  }

  fn read_constant_name_and_type(reader: &mut dyn Read) -> error::Result<ConstantEnum> {
    let name = common::read_be_u16_from_reader(reader)?;
    let descriptor = common::read_be_u16_from_reader(reader)?;
    Ok(ConstantEnum::NameAndType(name, descriptor))
  }

  fn read_constant_utf8(reader: &mut dyn Read) -> error::Result<ConstantEnum> {
    let length = common::read_be_u16_from_reader(reader)?;
    let bytes = common::read_vec_u8_from_reader(reader, length as usize)?;
    Ok(ConstantEnum::UTF8(length, bytes))
  }

  fn read_constant_method_handle(reader: &mut dyn Read) -> error::Result<ConstantEnum> {
    let reference_king = common::read_be_u8_from_reader(reader)?;
    let reference_index = common::read_be_u16_from_reader(reader)?;
    Ok(ConstantEnum::MethodHandle(reference_king, reference_index))
  }

  fn read_constant_method_type(reader: &mut dyn Read) -> error::Result<ConstantEnum> {
    let descriptor_index = common::read_be_u16_from_reader(reader)?;
    Ok(ConstantEnum::MethodType(descriptor_index))
  }

  fn read_constant_invoke_dynamic(reader: &mut dyn Read) -> error::Result<ConstantEnum> {
    let bootstrap_method_attr_index = common::read_be_u16_from_reader(reader)?;
    let name_and_type_index = common::read_be_u16_from_reader(reader)?;
    Ok(ConstantEnum::InvokeDynamic(bootstrap_method_attr_index, name_and_type_index))
  }

}

#[derive(Debug)]
pub enum ConstantEnum {
  Class(u16),
  FieldRef(u16, u16),
  MethodRef(u16, u16),
  InterfaceMethodRef(u16, u16),
  String(u16),
  Integer(u32),
  Float(u32),
  Long(u32, u32),
  Double(u32, u32),
  NameAndType(u16, u16),
  UTF8(u16, Vec<u8>),
  MethodHandle(u8, u16),
  MethodType(u16),
  InvokeDynamic(u16, u16),
}

pub enum ConstantType {
  Class,
  FieldRef,
  MethodRef,
  InterfaceMethodRef,
  String,
  Integer,
  Float,
  Long,
  Double,
  NameAndType,
  UTF8,
  MethodHandle,
  MethodType,
  InvokeDynamic,
  None,
}

impl From<u8> for ConstantType {
  fn from(tag: u8) -> Self {
    match tag {
      1 => ConstantType::UTF8,
      3 => ConstantType::Integer,
      4 => ConstantType::Float,
      5 => ConstantType::Long,
      6 => ConstantType::Double,
      7 => ConstantType::Class,
      8 => ConstantType::String,
      9 => ConstantType::FieldRef,
      10 => ConstantType::MethodRef,
      11 => ConstantType::InterfaceMethodRef,
      12 => ConstantType::NameAndType,
      15 => ConstantType::MethodHandle,
      16 => ConstantType::MethodType,
      19 => ConstantType::InvokeDynamic,
      _ => ConstantType::None
    }
  }
}