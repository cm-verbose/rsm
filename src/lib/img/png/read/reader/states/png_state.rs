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
  /// Read image prelude (Signature and fixed IHDR start)
  ReadPrelude
}

define_png_state! {
  ReadPostPrelude
}