use base64_rs::decoders::decode_faster;
use base64_rs::encoders::{encode, encode_faster};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  let string_encode = String::from("Man is distinguished, not only by his reason, but by this singular passion from other animals, 
    which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable 
    generation of knowledge, exceeds the short vehemence of any carnal pleasure.");
  let string_decode = String::from("TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0
  aGlzIHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCAKd2hpY2ggaXMgYSBsdXN0IG9mIHRoZSBtaW5kLCB0aGF0IGJ5I
  GEgcGVyc2V2ZXJhbmNlIG9mIGRlbGlnaHQgaW4gdGhlIGNvbnRpbnVlZCBhbmQgaW5kZWZhdGlnYWJsZSAKZ2VuZXJhdGlvbiBvZiBrbm
  93bGVkZ2UsIGV4Y2VlZHMgdGhlIHNob3J0IHZlaGVtZW5jZSBvZiBhbnkgY2FybmFsIHBsZWFzdXJlLg==");

  let mut contents_encode = string_encode.into_bytes();
  let mut contents_decode = string_decode.into_bytes();
  c.bench_function("encode", |b| {
    b.iter(|| encode(black_box(&mut contents_encode)))
  });
  c.bench_function("encode_faster", |b| {
    b.iter(|| encode_faster(black_box(&mut contents_encode)))
  });
  c.bench_function("decode_faster", |b| {
    b.iter(|| decode_faster(black_box(&mut contents_decode)))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
