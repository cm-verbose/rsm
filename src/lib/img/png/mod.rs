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
  /// Parsing specific chunks
  pub mod chunks {
    // IHDR - Image header
    pub mod ihdr {
      pub mod handle_ihdr;
      pub mod png_bit_depth;
      pub mod png_color_type;
      pub mod png_compression_method;
      pub mod png_filter_method;
      pub mod png_header;
      pub mod png_interlace_method;
    }

    /// PLTE - Palette
    pub mod handle_plte;

    /// bkGD - Background color
    pub mod handle_bkgd;

    /// gAMA - Image gamma
    pub mod handle_gama;

    /// pHYs - Physical pixel dimensions
    pub mod phys {
      pub mod handle_phys;
      pub mod png_physical_dimensions;
    }

    /// sRGB - Standard RGB color space
    pub mod srgb {
      pub mod handle_srgb;
      pub mod png_rendering_intent;
    }

    /// sBIT - Significant bits
    pub mod handle_sbit;

    // tRNS - Transparency
    pub mod handle_trns;
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
