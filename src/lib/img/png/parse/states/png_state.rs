mod private {
  pub trait Sealed {}
}

/// State for the parser which state is sealed using [`private::Sealed`].
pub trait PNGState: private::Sealed {}

macro_rules! define_png_state {
  ($(#[$comment:meta])* $state: ident) => {
    $(#[$comment])*
    pub struct $state;
    impl private::Sealed for $state {}
    impl PNGState for $state {}
  };
}

define_png_state! {
  /// Initial state in which the parser is reading the image's signature.
  ReadSignature
}

define_png_state! {
  /// State in which the IDHR (Image header) chunk is being read.
  ReadIHDR
}

define_png_state! {
  /// State in which chunks that follow the IHDR (Image header) chunk but
  /// precede the IDAT (Image data) chunk are read.
  ReadPostIHDR
}

define_png_state! {
  /// State in which the IDAT (Image data) chunks are being read.
  ReadIDAT
}

define_png_state! {
  /// State in which chunks that follow the IDAT (Image data) chunks are read.
  /// These include text annotations or timestamp information.
  ReadPostIDAT
}

define_png_state! {
  /// State in which the final chunk, IEND (Image trailer) is read.
  ReadIEND
}

define_png_state! {
  /// State in which trailing bytes (which may exist) are read, but only for
  /// metadata purposes.
  ReadPostIEND
}
