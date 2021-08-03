use std::fs::File;
use std::io::BufReader;

mod access;
mod args;
mod attribute;
mod class;
mod common;
mod constant;
mod error;
mod field;
mod interface;
mod magic;
mod method;
mod superior;
mod this;
mod version;

use common::FromReader;

fn main() {
  let file = File::open("/home/whosethink/Downloads/Color.class").unwrap();
  let mut reader = BufReader::new(file);
  let c = class::Class::from_reader(&mut reader).unwrap();
  println!("{}", c.to_string());
}
