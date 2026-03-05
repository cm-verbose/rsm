/// Utility macro for testing images
#[macro_export]
macro_rules! test_image {
  ($fn_name: ident, $file_name: expr, $expected: expr) => {
    #[test]
    fn $fn_name() {
      use rsm::lib::{img::png::img::png_image::PNGImage, util::data::file_data::FileData};
      use std::path::Path;

      let mut image = PNGImage::new();
      let path = concat!("./tests/png/png-suite/", $file_name);
      
      assert_eq!(
        image.read(&FileData::new(Path::new(path)).expect("error")).is_ok(),
        $expected
      )
    }
  };
}
