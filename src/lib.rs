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
