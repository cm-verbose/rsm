/// Logic for PNG chunks
pub mod chunks {
  /// Image header contents
  pub mod ihdr {
    /// Image header
    pub mod header;
  }

  /// Chunk information trait
  pub mod chunk_data;
}
