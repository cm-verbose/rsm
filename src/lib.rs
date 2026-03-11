pub mod lib {
  /// Image formats
  pub mod img {
    /// `.png` Files
    pub mod png;
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
