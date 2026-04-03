/// `IHDR` - Image header chunk
pub mod ihdr {
  pub mod handle_ihdr;
  pub mod png_bit_depth;
  pub mod png_color_type;
  pub mod png_compression_method;
  pub mod png_filter_method;
  pub mod png_header;
  pub mod png_interlace_method;
}

/// `PLTE` - Palette chunk
pub mod handle_plte;

/// `acTL` - Animation control chunk
pub mod actl {
  pub mod handle_actl;
  pub mod png_animation_control;
}

/// `bkGD` - Background color chunk
pub mod handle_bkgd;

/// `caBX` - Content Credentials
pub mod cabx {
  pub mod handle_cabx;
  pub mod png_attribution_manifest;
}

/// `cHRM` - Primary chromaticities and white point
pub mod chrm {
  pub mod handle_chrm;
  pub mod png_chromaticities;
}

/// `cICP` Coding-independent code points for video signal type identification chunk
pub mod cicp {
  pub mod handle_cicp;
  pub mod png_code_points;
}

/// `cLLI` Content light level information chunk
pub mod clli {
  pub mod handle_clli;
  pub mod png_light_level;
}

/// `eXIf` - Exchangeable image file profile chunk
pub mod exif {
  pub mod handle_exif;
  pub mod png_exif;
}

/// `fcTL` - Frame Control chunk
pub mod fctl {
  pub mod handle_fctl;
  pub mod png_alpha_blend;
  pub mod png_fctl_frame;
  pub mod png_frame_area_disposal;
}

/// `gAMA` - Image gamma chunk
pub mod handle_gama;
pub mod handle_hist;

/// `iCCP` - Embedded ICC profile chunk
pub mod iccp {
  pub mod handle_iccp;
  pub mod png_icc_profile;
}

/// `IDAT` - Image data chunk
pub mod idat {
  pub mod handle_idat;
  pub mod png_filters;
  pub mod png_pixel_data;
  pub mod png_subimage;
  pub mod png_unfilter;
}

/// `mDCV` - Mastering display color volume chunk
pub mod mdcv {
  pub mod handle_mdcv;
  pub mod png_color_volume;
}

/// `pHYs` - Physical pixel dimensions chunk
pub mod phys {
  pub mod handle_phys;
  pub mod png_physical_dimensions;
}

/// `sBIT` - Significant bits chunk
pub mod handle_sbit;

/// `sRGB` - Standard RGB color space chunk
pub mod srgb {
  pub mod handle_srgb;
  pub mod png_rendering_intent;
}

/// `tIME` Image last-modification time chunk
pub mod time {
  pub mod handle_time;
  pub mod png_time;
}

/// `tEXt` - Textual data chunk
pub mod text {
  pub mod handle_text;
  pub mod png_text;
}

/// `tRNS` - Transparency chunk
pub mod handle_trns;

/// `zTXt` - Compressed textual data chunk
pub mod handle_ztxt;

/// Utility modules for chunks
pub mod utils;
