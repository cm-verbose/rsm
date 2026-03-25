pub mod lib {
  /// Image formats
  pub mod img {
    /// `.png` files
    pub mod png;
  }

  /// Utility modules
  pub mod util {
    /// Data handlign utilities
    pub mod data {
      /// File data utility
      pub mod file_data;
    }

    /// Error handling utilities
    pub mod err {
      /// RSM error
      pub mod rsm_error;
    }
  }
}
