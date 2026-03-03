use rsm::lib::img::png::img::png_image::PNGImage;
use std::path::Path;

#[test]
fn test_basi() {
  let image: PNGImage = PNGImage::new();

  assert!(
    image
      .read(Path::new("./tests/png/png-suite/basi0g01.png"))
      .is_ok()
  )
}
