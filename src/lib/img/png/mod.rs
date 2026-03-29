pub mod chunk {
  pub mod png_chunk;
  pub mod png_chunk_type;
}

pub mod image {
  pub mod png_image;
}

pub mod parse {
  pub mod chunks;

  pub mod states {
    pub mod data {
      pub mod png_metadata;
      pub mod set_data;
    }
    pub mod png_state;
    pub mod read_ihdr;
    pub mod read_post_ihdr;
    pub mod read_signature;
  }
  pub mod png_parser;
}

pub mod read {
  mod png_read;
}

pub mod reader {
  pub mod png_reader;
}
