mod magic;
mod error;
mod version;
mod common;
mod constant;
mod access;
mod this;
mod superior;
mod interface;
mod field;
mod attribute;
mod method;
mod class;

use std::fs::File;
use std::io::BufReader;

fn main() {
  let class = File::open("/home/whosethink/Downloads/Color.class").unwrap();
  let mut reader = BufReader::new(class);
}
