use crate::rsm::img::png::{
  chunk::chunk::Chunk, image::png_image::PNGImage, reader::png_reader::PNGReader,
};

impl<'a> PNGReader<'a> {
  /// Handle the IDHR (Image header) chunk
  pub(in super::super) fn handle_ihdr(&mut self, chunk: &Chunk<'a>) -> Result<(), String> {
    let data: [u8; 13] = chunk.data.try_into().unwrap();
    let width: u32 = self.get_ihdr_size(&data[0..4])?;
    let height: u32 = self.get_ihdr_size(&data[4..8])?;

    macro_rules! set_image_data {
      ($($field:ident: $field_value:expr),+) => {
        self.image = PNGImage {
          $($field: Some($field_value),)+
          ..Default::default()
        };
      };
    }
    set_image_data!(
      width: width, height: height, bit_depth: data[8], color_type: data[9],
      compression_method: data[10], filter_method: data[11], interlace_method: data[12]
    );
    Ok(())
  }

  /// Get the size (either width or height) of the IHDR chunk
  fn get_ihdr_size(&self, slice: &[u8]) -> Result<u32, String> {
    if let Ok::<[u8; 4], _>(bytes) = slice.try_into() {
      let size: u32 = u32::from_be_bytes(bytes);
      if size == 0 {
        return Err(format!("Invalid dimension: {}", size));
      }
      Ok(size)
    } else {
      Err(format!("Failed parsing slice : {:?}", slice))
    }
  }
}
