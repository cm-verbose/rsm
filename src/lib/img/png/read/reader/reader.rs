use crate::lib::img::png::read::reader::states::png_state::{PNGState, ReadPrelude};
use std::marker::PhantomData;

/// PNG reader.
#[derive(Clone)]
pub(crate) struct PNGReader<'d, S: PNGState> {
  pub _state: PhantomData<S>,
  pub data: &'d [u8],
  pub ptr: usize,
}

impl<'d> PNGReader<'d, ReadPrelude> {
  /// Creates a new reader.
  pub(crate) fn new() -> Self {
    Self {
      _state: PhantomData,
      data: &[],
      ptr: 0,
    }
  }
}
