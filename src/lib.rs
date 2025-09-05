pub mod rsm_lib {
  pub mod img {
    pub mod png {
      pub mod chunk {
        pub mod png_chunk;
      }
      pub mod handler {
        pub mod chunking;
        pub mod parsing;
        pub mod png_handler;
        pub mod reading;
        pub mod validation;
      }
      pub mod image {
        pub mod png_image;
      }
    }
  }

  pub mod color {
    pub mod color;

    pub mod colors {
      pub mod rgb;

      pub mod rgba;
    }
  }

  pub mod util {
    pub mod reporter;
  }
}
