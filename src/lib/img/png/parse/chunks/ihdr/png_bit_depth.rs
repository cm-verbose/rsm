use crate::define_png_enum;

define_png_enum! {
  #[derive(Debug, PartialEq, Clone, Copy)]
  pub enum BitDepth {
    D1 = 1,
    D2 = 2,
    D4 = 4,
    D8 = 8,
    D16 = 16
  }
}
