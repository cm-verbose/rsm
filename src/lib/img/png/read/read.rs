use crate::lib::{
  img::png::img::image::PNGImage,
  util::{err::error::RSMError, files::file_data::FileData},
};

impl PNGImage {
  /// Load a PNG image from a given [path](AsRef<Path>).
  pub fn load<T>(path: T) -> Result<(), RSMError>
  where
    T: TryInto<FileData>,
    T::Error: Into<RSMError>,
  {
    let file_data: FileData = path.try_into().map_err(Into::into)?;
    Self::load_bytes(file_data.get_bytes())
  }

  /// Load a PNG image from a given sequence of contiguous bytes.
  pub fn load_bytes(bytes: &[u8]) -> Result<(), RSMError> {
    #[cfg(target_feature = "neon")]
    {
      use crate::lib::img::png::read::reader::neon::reader::PNGReader;
      let reader: PNGReader<'_, _> = PNGReader::new();
      reader.read(bytes)
    }
  }
}
