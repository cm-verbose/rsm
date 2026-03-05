/// Macro used to generate all chunk types available in a PNG
macro_rules! define_chunk_types {
  ($($variant:ident),+ $(,)?) => {
    const fn str_to_u32(text: &str) -> u32 {
      let b = text.as_bytes();
      u32::from_be_bytes([b[0], b[1], b[2], b[3]])
    }

    /// A PNG chunk type
    #[allow(non_camel_case_types)]
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum ChunkType {
      $($variant,)+
      Private(u32)
    }

    impl From<u32> for ChunkType {
      fn from(value: u32) -> Self {
        match value {
          $(value if value == str_to_u32(stringify!($variant)) => ChunkType::$variant,)+
          _ => ChunkType::Private(value)
        }
      }
    }

    impl ChunkType {
      pub fn as_u32(&self) -> u32 {
        match self {
          $(ChunkType::$variant => str_to_u32(stringify!($variant)),)+
          ChunkType::Private(value) => *value
        }
      }

      pub fn as_bytes(&self) -> [u8; 4] {
        self.as_u32().to_be_bytes()
      }
    }
  }
}

define_chunk_types! {
  IHDR, PLTE, IDAT, IEND, acTL, cHRM, cICP, gAMA, iCCP, mDCV, cLLI, sBIT, sRGB, sPLT, bKGD,
  hIST, tRNS, eXIf, fcTL, pHYs, fdAT, tIME, iTXt, tEXt, zTXt, caBX
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use proptest::{prop_assert_eq, proptest};

  /// Test well-defined chunk types
  #[test]
  fn validate_defined_chunks() {
    let defined_chunks: [&str; 25] = [
      "IHDR", "PLTE", "IDAT", "IEND", "acTL", "cHRM", "cICP", "gAMA", "iCCP", "mDCV", "cLLI",
      "sBIT", "sRGB", "sPLT", "bKGD", "hIST", "tRNS", "eXIf", "fcTL", "pHYs", "fdAT", "tIME",
      "tEXt", "zTXt", "caBX",
    ];

    for chunk in defined_chunks {
      let bytes: &[u8; 4] = chunk.as_bytes().try_into().unwrap();
      let r#type: ChunkType = ChunkType::from(u32::from_be_bytes(*bytes));

      assert_eq!(*bytes, r#type.as_bytes());
      assert!(!matches!(r#type, ChunkType::Private(_)))
    }
  }

  proptest! {
    /// Test chunk type conversions (&str to u32, u32 to &str)
    #[test]
    fn test_u32_private_chunk_conversions(random_chunk_type in "[a-zA-Z]{4}") {
      let chunk_u32: u32 = str_to_u32(&random_chunk_type);
      let expected: &[u8; 4] = &random_chunk_type.as_bytes().try_into().unwrap();

      prop_assert_eq!(&chunk_u32.to_be_bytes(), expected);
      let chunk_type = ChunkType::from(chunk_u32);

      prop_assert_eq!(chunk_type.as_bytes(), *expected)
    }
  }
}
