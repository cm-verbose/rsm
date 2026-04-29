mod private {
  pub trait Sealed {}
}

/// A possible state for the PNG state machine
pub trait PNGState: private::Sealed {}

/// State where the image signature is read
#[derive(Clone, Copy)]
pub(crate) struct ReadSignature;
impl private::Sealed for ReadSignature {}
impl PNGState for ReadSignature {}

/// State where Image header data is read
#[derive(Clone, Copy)]
pub(crate) struct ReadIHDR;
impl private::Sealed for ReadIHDR {}
impl PNGState for ReadIHDR {}
