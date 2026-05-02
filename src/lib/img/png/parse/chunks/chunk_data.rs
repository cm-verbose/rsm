use crate::lib::util::err::error::RSMError;

pub(crate) trait _ChunkData<'d>: Sized {
  fn from_bytes(data: &'d [u8]) -> Result<Self, RSMError>;
}
