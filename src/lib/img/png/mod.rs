/// Image chunk
pub mod chunk {
  /// PNG image chunk
  pub mod chunk;

  /// PNG image chunk types
  pub mod chunk_types;
}

/// Image contents
pub mod img {
  /// PNG image
  pub mod image;
}

/// Parsing operations
pub mod parse;

/// Read operation on the image
pub mod read {
  /// Read data from file
  pub mod read;

  /// Chunk parsing operations
  pub mod reader {
    /// Read bytes
    pub mod byte_reader;

    /// Chunk parser
    pub mod reader;

    /// Parsing operation
    pub mod read;

    /// Different stages of parsing based state
    pub mod states {
      /// State for the parsing state machine
      pub mod png_state;

      /// State for reading the image's signature
      pub mod state_signature;

      /// State for reading content following the image's header
      pub mod state_read_post_ihdr;

      /// State for reading the image's header data
      pub mod state_read_ihdr;
    }
  }
}
