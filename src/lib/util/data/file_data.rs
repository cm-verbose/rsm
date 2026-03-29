use crate::lib::util::err::rsm_error::RSMError;
use memmap2::Mmap;
use std::{
  borrow::Cow,
  ffi::{OsStr, OsString},
  fs::File,
  io::Read,
  path::{Path, PathBuf},
};

/// Utility enum used to separate file I/O logic from pasring logic to ensure
/// consistency across reads.
pub enum FileData<'a> {
  /// File data stored in the heap
  Heap(Cow<'a, [u8]>),

  /// Memory-mapped file data
  Map(Mmap),
}

impl<'a> FileData<'a> {
  /// Create new file data for values that can be interpreted as a [Path].
  pub fn from_path(path: impl AsRef<Path>) -> Result<Self, RSMError> {
    let mut file: File = File::open(path)?;

    let size: usize = file.metadata()?.len() as usize;
    let page_size: usize = page_size::get();

    let result: FileData<'_> = if size < page_size {
      let mut buffer: Vec<u8> = Vec::with_capacity(size);
      file.read_to_end(&mut buffer)?;

      Self::Heap(Cow::Owned(buffer))
    } else {
      let map: Mmap = unsafe { Mmap::map(&file) }?;
      Self::Map(map)
    };
    Ok(result)
  }

  /// Obtain the bytes read from a given file
  pub fn as_bytes(&self) -> &[u8] {
    match self {
      Self::Heap(data) => data,
      Self::Map(map) => map,
    }
  }
}

/// Macro used to define conversions for [FileData] for any [AsRef<Path>].
macro_rules! define_conversions {
  ($($to: ty),+) => {
    $(
      // Compile time check for the type passed to $ty.
      const _: () = {
        fn is_impl_path<T: std::convert::AsRef<std::path::Path> + ?Sized>() {}
        let _ = is_impl_path::<$to>;
      };

      impl<'a> TryFrom<&'a $to> for FileData<'a> {
        type Error = RSMError;

        fn try_from(path: &'a $to) -> Result <Self, Self::Error> {
          FileData::from_path(path)
        }
      }
    )+
  };
}

define_conversions!(OsStr, OsString, Path, PathBuf, str, String);
