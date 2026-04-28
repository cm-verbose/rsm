use crate::lib::util::err::error::RSMError;
#[cfg(unix)]
use memmap2::Advice;
use memmap2::Mmap;
use std::{
  borrow::Cow,
  fs::{self, File},
  path::{Path, PathBuf},
};

/// A utility to represent file data.
pub enum FileData {
  /// File read using native reads.
  Native(Cow<'static, [u8]>),

  /// File read using a memory map based on a heuristic.
  Mmap(Mmap),
}

impl FileData {
  /// Reads file data from a given path.
  pub fn new(path: impl AsRef<Path>) -> Result<Self, RSMError> {
    let file: File = File::open(&path)?;

    let size: usize = file.metadata()?.len() as usize;
    let page_size: usize = page_size::get();

    let result: FileData = if size < page_size * 4 {
      let buffer: Vec<u8> = fs::read(path)?;
      Self::Native(Cow::Owned(buffer))
    } else {
      let map: Mmap = unsafe { Mmap::map(&file) }?;

      #[cfg(unix)]
      map.advise(Advice::Sequential)?;
      Self::Mmap(map)
    };
    Ok(result)
  }

  /// Get the bytes within the file data.
  pub fn get_bytes(&self) -> &[u8] {
    match self {
      Self::Native(bytes) => bytes,
      Self::Mmap(map) => map,
    }
  }
}

impl TryFrom<PathBuf> for FileData {
  type Error = RSMError;

  fn try_from(buffer: PathBuf) -> Result<Self, Self::Error> {
    Self::new(buffer)
  }
}

impl TryFrom<&Path> for FileData {
  type Error = RSMError;

  fn try_from(path: &Path) -> Result<Self, Self::Error> {
    Self::new(path)
  }
}
