use crate::lib::util::err::rsm_error::RSMError;
use memmap2::Mmap;
use std::{fs::File, io::Read, path::Path};

/// Utility enum used to separate reading logic from parser logic
pub enum FileData {
  Native(Vec<u8>),
  MemoryMapped(Mmap),
}

impl FileData {
  /// Interprets a file's path as file data
  pub fn new(path: &Path) -> Result<Self, RSMError> {
    let mut file: File = File::open(path)?;
    let file_sz: usize = file.metadata()?.len() as usize;
    let pg_sz: usize = page_size::get();

    if file_sz < pg_sz * 4 {
      let mut buffer: Vec<u8> = Vec::with_capacity(file_sz);
      file.read_to_end(&mut buffer)?;
      Ok(FileData::Native(buffer))
    } else {
      let map: Mmap = unsafe { Mmap::map(&file) }?;
      Ok(FileData::MemoryMapped(map))
    }
  }

  pub fn as_bytes(&self) -> &[u8] {
    match self {
      FileData::Native(items) => items,
      FileData::MemoryMapped(mmap) => mmap,
    }
  }
}
