pub mod rsm_lib {
  mod color {
    pub mod colors {
      pub mod rgb;
      pub mod rgba;
    }
    pub mod color;
  }

  pub mod img {
    pub mod png {
      pub mod chunk {
        pub mod chunk;
      }
      pub mod handler {
        pub mod png_handler;
      }
      pub mod image {
        pub mod png_image;
      }
      pub mod reader {
        pub mod chunks {
          pub mod idat {
            pub mod filter_reverse;
            pub mod handle_idat;
          }
          pub mod handle_iend;
          pub mod handle_ihdr;
        }

        pub mod parse;
        pub mod png_reader;
        pub mod read;
        pub mod read_chunks;
        pub mod validation;
      }
    }
  }

  pub mod util {
    pub mod reports {
      pub mod reporter;
    }
  }
}
