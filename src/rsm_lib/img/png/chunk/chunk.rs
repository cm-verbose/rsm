#![allow(non_upper_case_globals)]
macro_rules! define_chunk_types {
  ($($variant: ident),+) => {
    const fn get_val(chunk_type: &str) -> Result<[u8; 4], ()> {
      if chunk_type.len() != 4 {
        Err(())
      } else {
        let bytes = chunk_type.as_bytes();
        Ok([bytes[0], bytes[1], bytes[2], bytes[3]])
      }
    }

    const fn to_val(variant: &str) -> u32 {
      match get_val(variant) {
        Ok(v) => u32::from_be_bytes(v),
        Err(_) => panic!("Invalid chunk type defined")
      }
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, PartialEq)]
    #[repr(u32)]
    pub enum ChunkType {
      $($variant = to_val(stringify!($variant)),)+
      custom(String)
    }


    $(const $variant: u32 = to_val(stringify!($variant));)+
    impl From<u32> for ChunkType {
      fn from(value: u32) -> Self {
        match value {
          $($variant => ChunkType::$variant,)+
          _ => {
            let chunk_text = String::from_utf8(value.to_be_bytes().to_vec()).unwrap();
            ChunkType::custom(chunk_text)
          }
        }
      }
    }

    impl ChunkType {
      pub fn value(&self) -> u32 {
        match self {
          $(Self::$variant => $variant,)+
          ChunkType::custom(text) => to_val(text)
        }
      }
    }
  };
}

define_chunk_types!(
  IHDR, PLTE, IDAT, IEND, acTL, cHRM, cICP, gAMA, iCCP, mDCV, cLLI, sBIT, sRGB, bKGD, hIST, tRNS,
  eXIf, fcTL, pHYs, fdAT, tIME, iTXt, tEXt, zTXt
);

/// Represents a PNG chunk

#[derive(Debug)]
pub struct Chunk<'a> {
  pub length: u32,
  pub r#type: ChunkType,
  pub data: &'a [u8],
  pub crc: [u8; 4],
}
