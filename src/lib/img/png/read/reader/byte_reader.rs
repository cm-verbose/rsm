use crate::lib::{
  img::png::{
    chunk::chunk::Chunk,
    read::reader::{reader::PNGReader, states::png_state::PNGState},
  },
  util::err::error::RSMError,
};

impl<'d, S: PNGState> PNGReader<'d, S> {
  /// Take n bytes from the reader's current position.
  #[inline]
  pub(crate) fn take(&mut self, next: usize) -> Result<&'d [u8], RSMError> {
    let end = self
      .ptr
      .checked_add(next)
      .ok_or(RSMError::Other(String::from("")))?;

    if end <= self.data.len() {
      let bytes: &[u8] = unsafe { self.data.get_unchecked(self.ptr..end) };
      self.ptr = end;
      Ok(bytes)
    } else {
      Err(RSMError::Other(String::from("Out of bounds")))
    }
  }

  /// Take a constant N bytes from the reader's current position.
  #[inline]
  pub(crate) fn take_sized<const N: usize>(
    &mut self,
  ) -> Result<&'d [u8; N], RSMError> {
    let sized: &'d [u8; N] = self
      .take(N)?
      .try_into()
      .map_err(|_| RSMError::Other(String::from("Out of bounds")))?;
    Ok(sized)
  }

  /// Read a [chunk](Chunk).
  pub(crate) fn read_chunk(&mut self) -> Result<Chunk<'d>, RSMError> {
    let bytes: &[u8; 8] = self.take_sized::<8>()?;
    let res: u64 = u64::from_be_bytes(*bytes);

    // We get 8 bytes like so:
    // 0x[AAAA][BBBB][CCCC][DDDD][EEEE][FFFF][GGGG][HHHH]
    //
    // Shifting by 32 bits gives us the first 4 bytes (0x[AAAA][BBBB][CCCC][DDDD]).
    // The cast as u32 discards the high bits of the u64 which leaves us with the last four bytes.
    let length: u32 = (res >> 32) as u32;
    let r#type: u32 = res as u32;

    let content_total: usize = (length + 4) as usize;
    let content: &[u8] = self.take(content_total)?;
    let (data, crc) = unsafe { content.split_at_unchecked(length as usize) };

    Ok(Chunk {
      r#type,
      data,
      _crc: crc.try_into().unwrap(),
    })
  }

  /// Delagates chunk handling to a callback for parsing or other operations.
  #[inline(always)]
  pub(crate) fn handle_chunk<F, R>(
    &mut self,
    r#type: u32,
    data: &'d [u8],
    mut callback: F,
  ) -> Result<R, RSMError>
  where
    F: FnMut(u32, &'d [u8]) -> Result<R, RSMError>,
  {
    callback(r#type, data)
  }
}
