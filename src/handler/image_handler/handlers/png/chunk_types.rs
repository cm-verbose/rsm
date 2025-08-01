#![allow(non_upper_case_globals)]

/// Defines the different possible chunk types available to use in a
/// PNG image.
#[macro_export]
macro_rules! define_chunk_types {
  ($($variant:ident),+) => {
    // Converts passed identifiers into a number, calculated from bytes
    const fn to_repr(var: &'static str) -> u32 {
      const fn get_repr(chunk_type: &'static str) -> Result<[u8; 4], ()> {
        if chunk_type.len() != 4 {
          return Err(());
        }
        let bytes = chunk_type.as_bytes();
        Ok([bytes[0], bytes[1], bytes[2], bytes[3]])
      }
      match get_repr(var) {
        Ok(bytes) => u32::from_be_bytes(bytes),
        Err(()) => panic!("Invalid chunk length"),
      }
    }

    /// Represents all PNG chunk types along with a variant for custom
    /// defined chunk types
    #[allow(non_camel_case_types, unused)]
    #[derive(Clone, Debug, PartialEq)]
    #[repr(u32)]
    pub enum ChunkType {
      $($variant = to_repr(stringify!($variant)),)+
      custom(String),
    }

    $(const $variant: u32 = to_repr(stringify!($variant));)+
    impl From<u32> for ChunkType {
      fn from(value: u32) -> Self {
        match value {
          $($variant => ChunkType::$variant,)+
          _ => ChunkType::custom(String::from_utf8(value.to_be_bytes().to_vec()).unwrap()),
        }
      }
    }
  };
}

define_chunk_types!(
  IHDR, PLTE, IDAT, IEND, acTL, cHRM, cICP, gAMA, iCCP, mDCV, cLLI, sBIT, sRGB, bKGD, hIST, tRNS,
  eXIf, fcTL, pHYs, fdAT, tIME, iTXt, tEXt, zTXt
);
