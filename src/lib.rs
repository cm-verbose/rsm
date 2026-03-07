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
        pub mod png_data;
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

          /// bkGD - Background color
          pub mod handle_bkgd;

          /// gAMA - Image gamma
          pub mod handle_gama;

          /// pHYs - Physical pixel dimensions
          pub mod phys {
            pub mod handle_phys;
            pub mod png_physical_dimensions;
          }

          /// PLTE - Palette
          pub mod handle_plte;

          /// sRGB - Standard RGB color space
          pub mod srgb {
            pub mod handle_srgb;
            pub mod png_rendering_intent;
          }
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
