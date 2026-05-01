mod private {
  pub trait Sealed {}
}

/// A possible state for the PNG reader state.
pub trait PNGState: private::Sealed {}

/// Defines a given state for the PNG reader state machine.
macro_rules! define_png_state {
  ($(#[$doc: meta])* $state: ident) => {
    $(#[$doc])*
    pub(crate) struct $state;
    impl private::Sealed for $state {}
    impl PNGState for $state{}
  };
}

define_png_state! {
  /// State where the image signature is read.
  ReadSignature
}

define_png_state! {
/// State where image header data is read.
  ReadIHDR
}

define_png_state! {
  /// State to read data following the image header
  ReadPostIHDR
}
