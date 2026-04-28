#![cfg(feature = "png")]
use rsm::lib::img::png::img::image::PNGImage;
use std::path::Path;

#[test]
fn test_basic() {
  let base: &str = env!("CARGO_MANIFEST_DIR");
  let path = Path::new(base).join("tests/png/png_suite/basi0g01.png");
  let _image = PNGImage::load(path).unwrap();
}
