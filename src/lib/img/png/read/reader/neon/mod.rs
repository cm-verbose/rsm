/// Reader chunk reading operations
pub mod chunk;

/// Neon reader read operation
pub mod read;

/// Neon reader
pub mod reader;

/// Neon state handlers
pub mod state_handlers {
  /// Read prelude state
  pub mod state_read_prelude;

  /// Read header metadata
  pub mod state_read_header_data;
}

/// Neon reader states
pub mod states {
  /// Read states
  pub mod states;
}

/// Reader byte taking operations
pub mod take;
