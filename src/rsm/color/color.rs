use std::{any::Any, fmt::Debug};

/// Represents color, usually used on graphic elements (text, images etc.)
pub trait Color: Debug {
  fn as_any(&self) -> &dyn Any;
}

#[macro_export]
macro_rules! impl_color_conversions {
  ($base: ty, $($to: ty => $body: expr),* $(,)?) => {
    use std::any::Any;
    $(
      impl From<$base> for $to {
        fn from(base: $base) -> Self {
          $body(base as $base)
        }
      }

      impl Color for $base {
        fn as_any(&self) -> &dyn Any {
          self
        }
      }
    )*
  };
}