use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate rust_playground;
use rust_playground::{assemble_tiles, count_monsters, parse_input};

fn criterion_benchmark(c: &mut Criterion) {
  let tiles = parse_input("input.txt");
  let image = assemble_tiles(&tiles);
  assert_eq!(count_monsters(&image, false), 43);

  c.bench_function("assemble image", |b| {
    b.iter(|| assemble_tiles(black_box(&tiles)))
  });

  c.bench_function("count monsters", |b| {
    b.iter(|| count_monsters(black_box(&image), false))
  });

  c.bench_function("assemble and count monsters", |b| {
    b.iter(|| count_monsters(&assemble_tiles(black_box(&tiles)), false))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
