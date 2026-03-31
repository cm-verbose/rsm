use crate::define_png_enum;

define_png_enum! {
  #[derive(Debug, PartialEq)]
  pub enum AlphaBlend {
    Source = 0,
    Over = 1,
  }
}
