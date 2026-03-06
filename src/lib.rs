pub mod lib {
  /// Image formats
  pub mod img {
    /// `.png` Files
    pub mod png {
      pub mod chunk {
        pub mod png_chunk;
        pub mod png_chunk_type;
      }

      pub mod img {
        pub mod png_image;
      }

      /// Parsing functions
      pub mod parse {
        /// Parsing specific chunks
        pub mod chunks {
          // IHDR - Image header
          pub mod ihdr {
            pub mod handle_ihdr;
            pub mod png_bit_depth;
            pub mod png_color_type;
            pub mod png_compression_method;
            pub mod png_filter_method;
            pub mod png_header;
            pub mod png_interlace_method;
          }
          // gAMA - Image gamma
          pub mod handle_gama;
        }
        pub mod png_parser;
      }

      /// Reading functions
      pub mod read {
        pub mod reader {
          pub mod png_reader;
          pub mod read;
          pub mod validate;
        }
        pub mod png_read;
      }
    }
  }

  /// Utility modules
  pub mod util {
    pub mod data {
      pub mod file_data;
    }

    /// Error handling utilities
    pub mod err {
      pub mod rsm_error;
    }
  }
}
