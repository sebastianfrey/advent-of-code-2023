use std::str::FromStr;

pub fn to_i32(str: &str) -> i32 {
  return  i32::from_str(str).unwrap();
}