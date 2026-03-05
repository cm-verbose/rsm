use rsm::lib::{img::png::img::png_image::PNGImage, util::data::file_data::FileData};
use std::path::Path;

#[test]
fn test_basi() {
  let mut image: PNGImage = PNGImage::new();

  assert!(
    image
      .read(&FileData::new(Path::new("./tests/png/png-suite/basi0g01.png")).expect("fail"))
      .is_ok()
  )
}
