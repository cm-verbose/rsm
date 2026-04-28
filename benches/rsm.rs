#![cfg(feature = "png")]
use criterion::{self, Criterion, criterion_group, criterion_main};
use rsm::lib::img::png::img::image::PNGImage;
use std::{
  hint::black_box,
  path::{Path, PathBuf},
};

fn bench_png(c: &mut Criterion) -> &mut Criterion {
  let base: &str = env!("CARGO_MANIFEST_DIR");
  let buf: PathBuf = Path::new(base).join("tests/png/png_suite/basi0g01.png");
  let data: Vec<u8> = std::fs::read(buf).unwrap();

  c.bench_function("Read PNG", |b| {
    b.iter(|| PNGImage::load_bytes(black_box(&data)).unwrap());
  })
}

criterion_group!(benches, bench_png);
criterion_main!(benches);
