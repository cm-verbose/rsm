use crate::lib::{
  img::png::{img::image::PNGImage, read::reader::reader::PNGReader},
  util::{err::error::RSMError, files::file_data::FileData},
};

impl PNGImage {
  /// Load a PNG image from a given path.
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
    let reader: PNGReader<_> = PNGReader::new();
    reader.read(bytes)?;
    Ok(())
  }
}
