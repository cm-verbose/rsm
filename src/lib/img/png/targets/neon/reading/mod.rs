/// Neon file read operations
pub mod read;

/// Neon image reader
pub mod reader {
  /// Neon reader crc32
  pub mod crc;

  /// Neon reader read operation
  pub mod read;

  /// Neon chunk reading functions
  pub mod read_chunk;

  /// Neon reader constructor
  pub mod reader;

  /// Byte taking functions
  pub mod take;

  /// Neon reader states
  pub mod states {
    /// Neon reader states definitions
    pub mod reader_states;

    /// Neon reader state: Read header data
    pub mod state_read_header_data;

    /// Neon reader state: Read prelude
    pub mod state_read_prelude;
  }
}
