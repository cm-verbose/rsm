#![no_main]
use rsm::lib::img::png::img::png_image::PNGImage;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
  let mut image = PNGImage::new();
  let _ = image.read_bytes(data);
});
