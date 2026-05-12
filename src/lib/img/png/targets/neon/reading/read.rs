use crate::lib::{
  img::png::{
    img::img::PNGImage, targets::neon::reading::reader::reader::PNGReader,
  },
  util::{err::error::RSMError, files::file_data::FileData},
};

impl PNGImage {
  /// Load an image from a given [path](AsRef<Path>).
  pub fn load<T>(path: T) -> Result<(), RSMError>
  where
    T: TryInto<FileData>,
    T::Error: Into<RSMError>,
  {
    let bytes: FileData = path.try_into().map_err(Into::into)?;
    Self::load_bytes(bytes.get_bytes())
  }

  /// Load an image from a sequence of bytes.
  pub fn load_bytes(bytes: &[u8]) -> Result<(), RSMError> {
    let reader: PNGReader<'_, _> = PNGReader::new(bytes);
    reader.read()?;
    Ok(())
  }
}
