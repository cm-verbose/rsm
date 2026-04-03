use rsm::lib::img::png::image::png_image::PNGImage;
use std::{
  env,
  path::{Path, PathBuf},
};

#[test]
fn test_read() {
  let manifest_dir = env!("CARGO_MANIFEST_DIR");
  let path = Path::new(manifest_dir);
  let target: PathBuf = path.join("tests/png/png_suite/basi0g01.png");

  let image: PNGImage = PNGImage::read(&target).unwrap();
  assert!(image.meta.gamma.is_some_and(|gamma| gamma == 1.0));
  println!("{:#?}", image);
  image.data.display_terminal();
}
