use std::ops::Range;

use crate::lib::util::err::rsm_error::RSMError;

pub(crate) fn get_bytes(range: Range<usize>, data: &[u8]) -> Result<&[u8], RSMError> {
  let Some(bytes) = data.get(range) else {
    return Err(RSMError::NotEnoughContent);
  };
  Ok(bytes)
}

/// Read text from bytes (Latin-1)
pub(crate) fn read_text(bytes: &[u8]) -> Result<String, RSMError> {
  Ok(bytes.iter().map(|&b| b as char).collect())
}

#[macro_export]
macro_rules! define_png_enum {
  (
    $(#[$meta:meta])*
    $vis:vis enum $name: ident {
      $($variant: ident = $value: expr),+ $(,)?
    }
  ) => {
    use crate::lib::util::err::rsm_error::RSMError;

    $(#[$meta])*
    $vis enum $name {
      $($variant = $value),+
    }

    impl TryFrom<u8> for $name {
      type Error = RSMError;

      fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
          $($value => Ok(Self::$variant),)+
          _ => Err(RSMError::InvalidContent)
        }
      }
    }

    #[cfg(test)]
    mod tests {
      use super::*;
      use proptest::prelude::*;

      #[test]
      fn test_mapping() {
        $(assert_eq!($name::$variant, $name::try_from($value).unwrap());)+
      }

      proptest! {
        #[test]
        fn test_invalid_values(value in 1..=u8::MAX) {
          let valid = [$($value),+];

          if !valid.contains(&value) {
            prop_assert!($name::try_from(value).is_err())
          }
        }
      }
    }
  }
}
