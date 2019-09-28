use std::error::Error;

pub fn sample_code(a: u8, b: u8) -> Result<u8, Box<Error>> {
  Ok(a + b)
}
