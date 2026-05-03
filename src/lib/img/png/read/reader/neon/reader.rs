use crate::lib::img::png::read::reader::neon::states::states::{
  PNGState, ReadPrelude,
};
use std::{marker::PhantomData, ptr};

/// Neon implementation of the reader.
pub(crate) struct PNGReader<'d, S: PNGState> {
  pub _state: PhantomData<S>,
  pub data: &'d [u8],
  pub ptr: *const u8,
}

impl<'d> PNGReader<'d, ReadPrelude> {
  pub(crate) fn new() -> Self {
    Self {
      _state: PhantomData,
      data: &[],
      ptr: ptr::null(),
    }
  }
}
