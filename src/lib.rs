pub mod lib {
  /// Image formats
  pub mod img {
    /// `.png` Files
    pub mod png {
      pub mod chunk {
        pub mod png_chunk;
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
    pub mod rsm_error;
  }
}
