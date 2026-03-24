/// A PNG chunk
pub mod chunk {
  pub mod png_chunk;
  pub mod png_chunk_type;
}

/// Representation of a PNG image
pub mod img {
  pub mod png_data;
  pub mod png_image;
}

/// Parsing functions
pub mod parse {
  pub mod png_parse;

  /// Parsing specific chunks
  pub mod chunks {
    /// `IHDR` - Image header
    pub mod ihdr {
      pub mod handle_ihdr;
      pub mod png_bit_depth;
      pub mod png_color_type;
      pub mod png_compression_method;
      pub mod png_filter_method;
      pub mod png_header;
      pub mod png_interlace_method;
    }

    /// `PLTE` - Palette
    pub mod handle_plte;

    /// `acTL` - Animation control chunk
    pub mod actl {
      pub mod handle_actl;
      pub mod png_animation_control;
    }

    /// `bkGD` - Background color
    pub mod handle_bkgd;

    /// `cHRM` - Primary chromaticities and white point
    pub mod chrm {
      pub mod handle_chrm;
      pub mod png_chromacities;
    }

    /// `cICP` - Coding-independent code points for video signal type identification
    pub mod cicp {
      pub mod handle_cicp;
      pub mod png_code_points;
    }

    /// `cLLI` - Content light level information
    pub mod clli {
      pub mod handle_clli;
      pub mod png_light_level;
    }

    /// `gAMA` - Image gamma
    pub mod handle_gama;

    /// `iCCP` - Embedded ICC profile
    pub mod iccp {
      pub mod handle_iccp;
      pub mod png_iccp_profile;
    }

    /// `mDCV` - Mastering display color volume
    pub mod mdcv {
      pub mod handle_mdcv;
      pub mod png_color_volume;
    }

    /// `pHYs` - Physical pixel dimensions
    pub mod phys {
      pub mod handle_phys;
      pub mod png_physical_dimensions;
    }

    /// `sBIT` - Significant bits
    pub mod handle_sbit;

    /// `sRGB` - Standard RGB color space
    pub mod srgb {
      pub mod handle_srgb;
      pub mod png_rendering_intent;
    }

    /// `tEXt` - Textual data
    pub mod text {
      pub mod handle_text;
      pub mod png_text;
    }

    /// `tIME` - Image last-modification time
    pub mod time {
      pub mod handle_time;
      pub mod png_time;
    }

    /// `tRNS` - Transparency
    pub mod handle_trns;

    /// `zTXt` - Compressed textual data
    pub mod handle_ztxt;
  }
  pub mod png_parser;
}

/// Reading functions
pub mod read {
  pub mod reader {
    pub mod png_reader;
    pub mod read;
    pub mod validate;
  }
  pub mod png_read;
}
