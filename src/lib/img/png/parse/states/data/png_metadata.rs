use crate::lib::img::png::parse::chunks::{
  actl::png_animation_control::AnimationControl,
  cabx::png_attribution_manifest::AttributionManifest, chrm::png_chromaticities::Chromaticities,
  cicp::png_code_points::CodePoints, clli::png_light_level::ContentLightLevel,
  fctl::png_fctl_frame::FCTLFrame, iccp::png_icc_profile::ICCProfile,
  mdcv::png_color_volume::ColorVolume, phys::png_physical_dimensions::PhysicalDimensions,
  srgb::png_rendering_intent::RenderingIntent, text::png_text::Text,
  time::png_time::ModificationTime,
};

#[derive(Default, Debug)]
pub struct PNGMetadata {
  pub animation_control: Option<AnimationControl>,
  pub attribution_manifest: Option<Vec<AttributionManifest>>,
  pub background_bytes: Option<Vec<u8>>,
  pub code_points: Option<CodePoints>,
  pub frames: Option<Vec<FCTLFrame>>,
  pub gamma: Option<f32>,
  pub chromaticities: Option<Chromaticities>,
  pub color_volume: Option<ColorVolume>,
  pub histogram: Option<Vec<u16>>,
  pub light_level: Option<ContentLightLevel>,
  pub modification_time: Option<ModificationTime>,
  pub icc_profile: Option<ICCProfile>,
  pub palette: Option<Vec<[u8; 3]>>,
  pub physical_dimensions: Option<PhysicalDimensions>,
  pub rendering_intent: Option<RenderingIntent>,
  pub significant_bits: Option<Vec<u8>>,
  pub text_entries: Option<Vec<Text>>,
  pub transparency_bytes: Option<Vec<u8>>,
}
