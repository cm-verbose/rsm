/// Define the possible PNG datastream chunk types
macro_rules! define_chunk_types {
  ($($variant:ident),+ $(,)?) => {
    const fn str_to_u32(text: &str) -> u32 {
      let b: &[u8] = text.as_bytes();
      let mut num: u32 = 0;
      let mut i = 4;
      while i != 0 {
        num |= (b[4 - i] as u32) << (i - 1) * 8;
        i -= 1;
      }
      num
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, PartialEq)]
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
      pub fn value(&self) -> u32 {
        match self {
          $(ChunkType::$variant => str_to_u32(stringify!($variant)),)+
          ChunkType::Private(value) => *value
        }
      }
    }
  }
}

define_chunk_types! {
  IHDR, PLTE, IDAT, IEND, acTL, cHRM, cICP, gAMA, iCCP, mDCV, cLLI, sBIT, sRGB, bKGD, hIST, tRNS,
  eXIf, fcTL, pHYs, fdAT, tIME, iTXt, tEXt, zTXt
}

#[allow(unused)]
#[derive(Debug)]
pub(in crate::rsm::img::png) struct Chunk<'c> {
  pub length: u32,
  pub r#type: ChunkType,
  pub data: &'c [u8],
  pub crc: [u8; 4],
}
