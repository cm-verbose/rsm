#![cfg(feature = "png")]
use criterion::{self, Criterion, criterion_group, criterion_main};
use rsm::lib::img::png::img::img::PNGImage;
use std::{hint::black_box, path::Path};

fn bench_png(c: &mut Criterion) {
  let mut group = c.benchmark_group("PNG");
  let base = env!("CARGO_MANIFEST_DIR");
  let suite_path = Path::new(base).join("tests/png/png_suite");

  group.sample_size(100);

  let test_cases: Vec<(String, Vec<u8>)> = std::fs::read_dir(suite_path)
    .unwrap()
    .filter_map(|e| e.ok())
    .map(|e| {
      let path = e.path();
      let name = path.file_name().unwrap().to_string_lossy().into_owned();
      let data = std::fs::read(path).expect("Failed to read test file");
      (name, data)
    })
    .collect();

  for (name, data) in &test_cases {
    if name.starts_with("x") || !name.ends_with(".png") {
      continue;
    }
    group.bench_with_input(name, data, |b, input| {
      b.iter(|| {
        let img = PNGImage::load_bytes(black_box(input)).unwrap();
        black_box(img)
      })
    });
  }

  group.finish();
}

criterion_group!(benches, bench_png);
criterion_main!(benches);
