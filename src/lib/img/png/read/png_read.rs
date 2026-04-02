use crate::lib::{
  img::png::{image::png_image::PNGImage, parse::png_parser::PNGParser},
  util::{data::file_data::FileData, err::rsm_error::RSMError},
};

impl PNGImage {
  /// Read a file as a PNG image from a value that can be interpreted as a
  /// [FileData] using [TryInto].
  #[inline]
  pub fn read<T>(data: T) -> Result<Self, RSMError>
  where
    T: TryInto<FileData<'static>>,
    T::Error: Into<RSMError>,
  {
    let file_data: FileData<'_> = data.try_into().map_err(Into::into)?;
    Self::read_bytes(file_data.as_bytes())
  }

  /// Read a sequence of bytes as the data of a PNG image.
  pub fn read_bytes(data: &'_ [u8]) -> Result<Self, RSMError> {
    Self::parse(data)
  }

  /// Drive the parser's finite state machine to the end to read the data
  fn parse(data: &'_ [u8]) -> Result<Self, RSMError> {
    let parser = PNGParser::new(data);
    let parser = parser.read_signature()?;
    let (parser, header) = parser.read_ihdr()?;
    let (parser, mut post_ihdr, first_idat) = parser.read_post_ihdr(&header)?;
    let (parser, data) = parser.read_idat(&first_idat, &header, &mut post_ihdr)?;
    let _ = parser.read_post_idat(&mut post_ihdr, &header);

    Ok(Self {
      header,
      meta: post_ihdr,
      data,
    })
  }
}
