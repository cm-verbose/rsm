use rsm::lib::{img::png::img::png_image::PNGImage, util::data::file_data::FileData};
use std::path::Path;

#[test]
fn test_gpt() {
  let mut image: PNGImage = PNGImage::new();
  image
    .read(&FileData::new(Path::new("./tests/png/r_suite/gpt_1.png")).unwrap())
    .unwrap();
}
