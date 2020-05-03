use base64_rs::encoders::{encode, encode_faster};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  let string = String::from("Man is distinguished, not only by his reason, but by this singular passion from other animals, 
    which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable 
    generation of knowledge, exceeds the short vehemence of any carnal pleasure.");
  let mut contents = string.into_bytes();
  c.bench_function("encode", |b| b.iter(|| encode(black_box(&mut contents))));
  c.bench_function("encode_faster", |b| {
    b.iter(|| encode_faster(black_box(&mut contents)))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
