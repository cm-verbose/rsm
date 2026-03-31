use crate::define_png_enum;

define_png_enum! {
  #[derive(Debug, PartialEq)]
  pub enum FrameAreaDisposal {
    None = 0,
    Background = 1,
    Previous = 2,
  }
}
