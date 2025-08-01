mod handler;
use std::path::Path;
use crate::handler::Handler;

fn main() {
  let handler = Handler::new();
  handler.read_from_path(Path::new("./image.png")).unwrap();
}
