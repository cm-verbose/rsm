use crate::lib::img::png::targets::neon::reading::reader::states::reader_states::{
  PNGState, ReadPrelude,
};
use std::marker::PhantomData;

/// Neon PNG reader.
pub(crate) struct PNGReader<'d, S: PNGState> {
  pub(crate) _state: PhantomData<S>,
  pub(crate) data: &'d [u8],
  pub(crate) ptr: *const u8,
  pub(crate) end: *const u8,
}

impl<'d> PNGReader<'d, ReadPrelude> {
  pub(crate) fn new(data: &'d [u8]) -> Self {
    let ptr: *const u8 = data.as_ptr();
    let end: *const u8 = unsafe { ptr.add(data.len()) };

    Self {
      _state: PhantomData,
      data,
      ptr,
      end,
    }
  }
}
