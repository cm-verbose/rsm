use rsm::rsm_lib::img::png::handler::png_handler::PNGHandler;
use std::path::Path;

#[test]
fn main() {
  let mut handler: PNGHandler = PNGHandler::new();
  let image = handler.read_file(Path::new("./image.png")).unwrap();

  println!("{:?}", image);
}
