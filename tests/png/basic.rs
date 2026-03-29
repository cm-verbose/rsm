use rsm::lib::img::png::image::png_image::PNGImage;

#[test]
fn test_read() {
  let image: PNGImage = PNGImage::read("./tests/png/png_suite/basi0g01.png").unwrap();
  assert!(image.meta.gamma.is_some_and(|gamma| gamma == 1.0));
  println!("{:#?}", image);
}
