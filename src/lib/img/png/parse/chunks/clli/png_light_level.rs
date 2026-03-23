/// Content light level information read from an cLLI chunk
pub struct ContentLightLevel {
  /// Maximum Content Light Level measured in cd/m<sup>2</sup> (nits or
  /// candella per meter squared)
  pub max_cll: f32,

  /// Maximum Frame-Average Light Level measured in cd/m<sup>2</sup> (nits or
  /// candella per meter squared)
  pub max_fall: f32,
}
