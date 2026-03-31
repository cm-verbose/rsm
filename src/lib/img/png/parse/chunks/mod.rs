pub mod ihdr {
  pub mod handle_ihdr;
  pub mod png_bit_depth;
  pub mod png_color_type;
  pub mod png_compression_method;
  pub mod png_filter_method;
  pub mod png_header;
  pub mod png_interlace_method;
}

pub mod handle_plte;

pub mod actl {
  pub mod handle_actl;
  pub mod png_animation_control;
}

pub mod handle_bkgd;

pub mod cabx {
  pub mod handle_cabx;
  pub mod png_attribution_manifest;
}

pub mod chrm {
  pub mod handle_chrm;
  pub mod png_chromaticities;
}

pub mod cicp {
  pub mod handle_cicp;
  pub mod png_code_points;
}

pub mod clli {
  pub mod handle_clli;
  pub mod png_light_level;
}

pub mod fctl {
  pub mod handle_fctl;
  pub mod png_alpha_blend;
  pub mod png_fctl_frame;
  pub mod png_frame_area_disposal;
}

pub mod handle_gama;
pub mod handle_hist;

pub mod iccp {
  pub mod handle_iccp;
  pub mod png_icc_profile;
}

pub mod mdcv {
  pub mod handle_mdcv;
  pub mod png_color_volume;
}

pub mod phys {
  pub mod handle_phys;
  pub mod png_physical_dimensions;
}

pub mod handle_sbit;

pub mod srgb {
  pub mod handle_srgb;
  pub mod png_rendering_intent;
}

pub mod time {
  pub mod handle_time;
  pub mod png_time;
}

pub mod text {
  pub mod handle_text;
  pub mod png_text;
}

pub mod handle_trns;

pub mod handle_ztxt;

/// Utility modules for chunks
pub mod utils;
