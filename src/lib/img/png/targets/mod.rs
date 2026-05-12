use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(target_feature="sve2")] {
    /// SVE2-optimized operations
    pub mod sve2;
  } else if #[cfg(target_feature="neon")] {
    /// Neon-optimized operations
    pub mod neon;
  } else {
    /// Generic implementation
    pub mod generic;
  }
}
