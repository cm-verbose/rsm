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

      /// State for handling the PNG prelude (Signature and IHDR)
      pub mod state_png_prelude;
    }
  }
}
