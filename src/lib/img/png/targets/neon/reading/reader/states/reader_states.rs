mod private {
  pub(crate) trait Sealed {}
}

pub(crate) trait PNGState: private::Sealed {}

macro_rules! define_png_state {
  ($(#[$doc: meta])* $state: ident) => {
    $(#[$doc])*
    pub(crate) struct $state;
    impl private::Sealed for $state {}
    impl PNGState for $state{}
  };
}

define_png_state! {
  ReadPrelude
}

define_png_state! {
  ReadHeaderData
}

define_png_state! {
  ReadPostIHDR
}
