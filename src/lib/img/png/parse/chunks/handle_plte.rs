use crate::lib::util::err::rsm_error::RSMError;

/// Handle `PLTE` (Palette) chunk
pub(crate) fn handle_plte(data: &[u8]) -> Result<Vec<[u8; 3]>, RSMError> {
  if !data.len().is_multiple_of(3) || data.len() > 768 || data.is_empty() {
    return Err(RSMError::InvalidLength);
  }
  let palette: Vec<[u8; 3]> = data
    .chunks_exact(3)
    .map(|triple| [triple[0], triple[1], triple[2]])
    .collect();
  Ok(palette)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lib::util::err::rsm_error::RSMError;
  use proptest::{
    collection::vec,
    prelude::{Strategy, any},
    proptest,
  };

  fn filter_above_768() -> impl Strategy<Value = Vec<u8>> {
    (257..10_000usize)
      .prop_map(|k| k * 3)
      .prop_flat_map(|size| vec(any::<u8>(), size))
  }

  #[test]
  fn test_plte_len_zero() {
    let res = handle_plte(&[]);
    assert!(res.is_err());
  }

  proptest! {
    #[test]
    fn test_plte_invalid_lengths(data in filter_above_768()) {
      let res: Result<Vec<[u8; 3]>, RSMError> = handle_plte(&data);
      assert!(res.is_err());
    }
  }
}
