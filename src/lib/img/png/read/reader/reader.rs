use crate::lib::img::png::read::reader::states::png_state::{PNGState, ReadSignature};
use std::marker::PhantomData;

/// PNG parser
#[derive(Clone)]
pub(crate) struct PNGReader<'d, S: PNGState> {
  pub _state: PhantomData<S>,
  pub data: &'d [u8],
  pub ptr: usize,
}

impl<'d> PNGReader<'d, ReadSignature> {
  /// Creates a new parser
  pub(crate) fn new() -> Self {
    Self {
      _state: PhantomData,
      data: &[],
      ptr: 0,
    }
  }
}
