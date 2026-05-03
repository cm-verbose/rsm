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

  /// Chunk reading operations
  pub mod reader {
    /// Neon-specific reader
    #[cfg(target_feature = "neon")]
    pub mod neon;
  }
}
