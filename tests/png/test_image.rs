/// Utility macro for testing images
#[macro_export]
macro_rules! test_png_suite_img {
  ($target: ident) => {
    paste::paste! {
      #[test]
      fn [<test_ $target>]() {
        use rsm::lib::{
          img::png::img::png_image::PNGImage,
          util::data::file_data::FileData
        };
        use std::path::Path;

        let name: &'static str = stringify!($target);
        let path_str = format!("{}{}.png", "./tests/png/png_suite/", name);

        let mut image = PNGImage::new();
        let result = image.read(&FileData::new(Path::new(&path_str)).unwrap());

        if (name.starts_with("x")) {
          assert!(result.is_err())
        } else {
          assert!(result.is_ok())
        }
      }
    }
  };
}
