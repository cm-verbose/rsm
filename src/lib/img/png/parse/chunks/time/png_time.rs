/// Last-modification time from the `tIME` chunk
#[derive(Debug, PartialEq)]
pub struct ModificationTime {
  pub year: u16,
  pub month: u8,
  pub day: u8,
  pub hour: u8,
  pub minute: u8,
  pub second: u8,
}
