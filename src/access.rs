pub struct ClassAccessFlag {
  flag: u16,
}

pub enum ClassAccessFlagEnum {
  Public,
  Private,
  Super,
  Interface,
  Abstract,
  Synthetic,
  Annotation,
  Enum,
}

impl From<ClassAccessFlagEnum> for u16 {
  fn from(flag: ClassAccessFlagEnum) -> Self {
    match flag {
      ClassAccessFlagEnum::Public => 0x0001,
      ClassAccessFlagEnum::Private => 0x0010,
      ClassAccessFlagEnum::Super => 0x0020,
      ClassAccessFlagEnum::Interface => 0x0200,
      ClassAccessFlagEnum::Abstract => 0x0400,
      ClassAccessFlagEnum::Synthetic => 0x1000,
      ClassAccessFlagEnum::Annotation => 0x2000,
      ClassAccessFlagEnum::Enum => 0x4000
    }
  }
}

impl From<ClassAccessFlagEnum> for String {
  fn from(flag: ClassAccessFlagEnum) -> Self {
    match flag {
      ClassAccessFlagEnum::Public => "PUBLIC".to_string(),
      ClassAccessFlagEnum::Private => "PRIVATE".to_string(),
      ClassAccessFlagEnum::Super => "SUPER".to_string(),
      ClassAccessFlagEnum::Interface => "INTERFACE".to_string(),
      ClassAccessFlagEnum::Abstract => "ABSTRACT".to_string(),
      ClassAccessFlagEnum::Synthetic => "SYNTHETIC".to_string(),
      ClassAccessFlagEnum::Annotation => "ANNOTATION".to_string(),
      ClassAccessFlagEnum::Enum => "ENUM".to_string()
    }
  }
}

pub struct FieldAccessFlag {
  flag: u16,
}

pub enum FieldAccessFlagEnum {
  Public,
  Private,
  Protected,
  Static,
  Final,
  Volatile,
  Transient,
  Synthetic,
  Enum,
}

impl From<FieldAccessFlagEnum> for u16 {
  fn from(flag: FieldAccessFlagEnum) -> Self {
    match flag {
      FieldAccessFlagEnum::Public => 0x0001,
      FieldAccessFlagEnum::Private => 0x0002,
      FieldAccessFlagEnum::Protected => 0x0004,
      FieldAccessFlagEnum::Static => 0x0008,
      FieldAccessFlagEnum::Final => 0x0010,
      FieldAccessFlagEnum::Volatile => 0x0040,
      FieldAccessFlagEnum::Transient => 0x0080,
      FieldAccessFlagEnum::Synthetic => 0x1000,
      FieldAccessFlagEnum::Enum => 0x4000
    }
  }
}

impl From<FieldAccessFlagEnum> for String {
  fn from(flag: FieldAccessFlagEnum) -> Self {
    match flag {
      FieldAccessFlagEnum::Public => "PUBLIC".to_string(),
      FieldAccessFlagEnum::Private => "PRIVATE".to_string(),
      FieldAccessFlagEnum::Protected => "PROTECTED".to_string(),
      FieldAccessFlagEnum::Static => "STATIC".to_string(),
      FieldAccessFlagEnum::Final => "FINAL".to_string(),
      FieldAccessFlagEnum::Volatile => "VOLATILE".to_string(),
      FieldAccessFlagEnum::Transient => "TRANSIENT".to_string(),
      FieldAccessFlagEnum::Synthetic => "SYNTHETIC".to_string(),
      FieldAccessFlagEnum::Enum => "ENUM".to_string()
    }
  }
}

pub struct MethodAccessFlag {
  flag: u16,
}

pub enum MethodAccessEnum {
  Public,
  Private,
  Protected,
  Static,
  Final,
  Synchronized,
  Bridge,
  Varargs,
  Native,
  Abstract,
  Strict,
  Synthetic,
}

impl From<MethodAccessEnum> for u16 {
  fn from(flag: MethodAccessEnum) -> Self {
    match flag {
      MethodAccessEnum::Public => 0x0001,
      MethodAccessEnum::Private => 0x0002,
      MethodAccessEnum::Protected => 0x0004,
      MethodAccessEnum::Static => 0x0008,
      MethodAccessEnum::Final => 0x0010,
      MethodAccessEnum::Synchronized => 0x0020,
      MethodAccessEnum::Bridge => 0x0040,
      MethodAccessEnum::Varargs => 0x0080,
      MethodAccessEnum::Native => 0x0100,
      MethodAccessEnum::Abstract => 0x0400,
      MethodAccessEnum::Strict => 0x0800,
      MethodAccessEnum::Synthetic => 0x1000
    }
  }
}

impl From<MethodAccessEnum> for String {
  fn from(flag: MethodAccessEnum) -> Self {
    match flag {
      MethodAccessEnum::Public => "PUBLIC".to_string(),
      MethodAccessEnum::Private => "PRIVATE".to_string(),
      MethodAccessEnum::Protected => "PROTECTED".to_string(),
      MethodAccessEnum::Static => "STATIC".to_string(),
      MethodAccessEnum::Final => "FINAL".to_string(),
      MethodAccessEnum::Synchronized => "SYNCHRONIZED".to_string(),
      MethodAccessEnum::Bridge => "BRIDGE".to_string(),
      MethodAccessEnum::Varargs => "VARARGS".to_string(),
      MethodAccessEnum::Native => "NATIVE".to_string(),
      MethodAccessEnum::Abstract => "ABSTRACT".to_string(),
      MethodAccessEnum::Strict => "STRICT".to_string(),
      MethodAccessEnum::Synthetic => "SYNTHETIC".to_string()
    }
  }
}