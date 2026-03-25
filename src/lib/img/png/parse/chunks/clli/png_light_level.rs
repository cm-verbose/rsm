/// Content light level information read from an cLLI chunk
#[derive(Debug, PartialEq)]
pub struct ContentLightLevel {
  /// Maximum Content Light Level measured in cd/m<sup>2</sup>
  /// ([nits](https://en.wikipedia.org/wiki/Candela_per_square_metre) or
  /// candella per meter squared)
  pub max_cll: f32,

  /// Maximum Frame-Average Light Level measured in cd/m<sup>2</sup> (
  /// ([nits](https://en.wikipedia.org/wiki/Candela_per_square_metre) or
  /// candella per meter squared)
  pub max_fall: f32,
}
