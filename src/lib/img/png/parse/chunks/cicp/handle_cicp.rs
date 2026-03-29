use crate::lib::{
  img::png::parse::chunks::cicp::png_code_points::CodePoints, util::err::rsm_error::RSMError,
};

/// Handle cICP (Coding-independent code points for video signal type
/// identification) chunk
pub(in super::super::super) fn handle_cicp(data: [u8; 4]) -> Result<CodePoints, RSMError> {
  Ok(CodePoints {
    color_primaries: data[0],
    transfer_function: data[1],
    matrix_coefficient: data[2],
    full_video_range: data[3],
  })
}
