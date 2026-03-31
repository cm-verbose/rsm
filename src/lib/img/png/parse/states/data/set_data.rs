use crate::lib::{
  img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    parse::{
      chunks::{
        actl::handle_actl::handle_actl, cabx::handle_cabx::handle_cabx,
        chrm::handle_chrm::handle_chrm, cicp::handle_cicp::handle_cicp,
        clli::handle_clli::handle_clli, fctl::handle_fctl::handle_fctl, handle_bkgd::handle_bkgd,
        handle_gama::handle_gama, handle_hist::handle_hist, handle_plte::handle_plte,
        handle_sbit::handle_sbit, handle_trns::handle_trns, handle_ztxt::handle_ztxt,
        iccp::handle_iccp::handle_iccp, ihdr::png_header::PNGHeader,
        mdcv::handle_mdcv::handle_mdcv, phys::handle_phys::handle_phys,
        srgb::handle_srgb::handle_srgb, text::handle_text::handle_text,
        time::handle_time::handle_time,
      },
      states::data::png_metadata::PNGMetadata,
    },
  },
  util::err::rsm_error::RSMError,
};

impl PNGMetadata {
  pub(crate) fn set_data(&mut self, chunk: Chunk<'_>, header: &PNGHeader) -> Result<(), RSMError> {
    match chunk.r#type {
      ChunkType::acTL => {
        if let Ok(control) = chunk.parse_data_sized::<8, _, _>(|&data| handle_actl(data)) {
          self.animation_control = control;
        }
      }

      ChunkType::bKGD => {
        if let Ok(background) = chunk.parse_data(|data| handle_bkgd(data, header.color_type)) {
          self.background_bytes = Some(background.to_vec());
        }
      }

      ChunkType::caBX => {
        if let Ok(manifest) = chunk.parse_data(handle_cabx) {
          self
            .attribution_manifests
            .get_or_insert(Vec::new())
            .extend(manifest.unwrap())
        }
      }

      ChunkType::cHRM => {
        if let Ok(chromacities) = chunk.parse_data_sized::<32, _, _>(|&data| handle_chrm(data)) {
          self.chromaticities = chromacities;
        }
      }

      ChunkType::cICP => {
        if let Ok(code_points) = chunk.parse_data_sized::<4, _, _>(|&data| handle_cicp(data)) {
          self.code_points = Some(code_points);
        }
      }

      ChunkType::cLLI => {
        if let Ok(light_level) = chunk.parse_data_sized::<8, _, _>(|&data| handle_clli(data)) {
          self.light_level = light_level;
        }
      }

      ChunkType::fcTL => {
        if let Ok(frame) = chunk.parse_data_sized::<26, _, _>(|&data| handle_fctl(data, header)) {
          self.frames.get_or_insert(Vec::new()).push(frame.unwrap());
        }
      }

      ChunkType::gAMA => {
        if let Ok(gamma) = chunk.parse_data_sized::<4, _, _>(|&data| handle_gama(data)) {
          self.gamma = Some(gamma);
        }
      }

      ChunkType::hIST => {
        if let Ok(histogram) = chunk.parse_data(handle_hist) {
          self.histogram = histogram;
        }
      }

      ChunkType::iCCP => {
        if let Ok(profile) = chunk.parse_data(handle_iccp) {
          self.icc_profile = Some(profile);
        }
      }

      ChunkType::mDCV => {
        if let Ok(color_volume) = chunk.parse_data_sized::<24, _, _>(|&data| handle_mdcv(data)) {
          self.color_volume = Some(color_volume);
        }
      }

      ChunkType::pHYs => {
        if let Ok(physical_dimensions) =
          chunk.parse_data_sized::<9, _, _>(|&data| handle_phys(data))
        {
          self.physical_dimensions = physical_dimensions
        }
      }

      ChunkType::PLTE => {
        let palette = chunk.parse_data(handle_plte)?;
        self.palette = Some(palette);
      }

      ChunkType::sBIT => {
        if let Ok(bits) = chunk.parse_data(|data| handle_sbit(data, header.color_type)) {
          self.significant_bits = Some(bits.to_vec());
        }
      }

      ChunkType::sRGB => {
        if let Ok(intent) = chunk.parse_data_sized::<1, _, _>(|&data| handle_srgb(data)) {
          self.rendering_intent = Some(intent);
        }
      }

      ChunkType::tEXt => {
        if let Ok(text) = chunk.parse_data(handle_text) {
          self.text_entries.get_or_insert(Vec::new()).push(text);
        }
      }

      ChunkType::tIME => {
        if let Ok(modification_time) = chunk.parse_data_sized::<7, _, _>(|&data| handle_time(data))
        {
          self.modification_time = modification_time;
        }
      }

      ChunkType::tRNS => {
        if let Ok(transparency) = chunk.parse_data(|data| handle_trns(data, header.color_type)) {
          self.transparency_bytes = Some(transparency.to_vec());
        }
      }

      ChunkType::zTXt => {
        if let Ok(text) = chunk.parse_data(handle_ztxt) {
          self.text_entries.get_or_insert(Vec::new()).push(text);
        }
      }

      _ => {}
    }
    Ok(())
  }
}
