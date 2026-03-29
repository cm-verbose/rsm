#![no_main]
use rsm::lib::img::png::img::png_image::PNGImage;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
  //
});
