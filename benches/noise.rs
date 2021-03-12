use lib::*;
use criterion::*;

pub fn noise(c: &mut Criterion){
    let mut arr = vec![0u8;2560*1440*4];
    let mut factor = Factor::new(0xDEADBEAF_ABBADEAD_12345678_AABBCCDD);
    c.bench_function("noise fill 2560x1440", |b| b.iter(
        || black_box(noise_fill(&mut arr, &mut factor))
    ));
}

pub fn rand(c: &mut Criterion){
    let mut factor = Factor::new(0xDEADBEAF);
    c.bench_function("rand", |b| b.iter(
        || {
                black_box(factor.next());
        }
    ));
}
pub fn rand128(c: &mut Criterion){
    let mut factor = Factor::new(0xDEADBEAF_ABBADEAD_12345678_AABBCCDD);
    c.bench_function("rand", |b| b.iter(
        || {
                black_box(factor.next128());
        }
    ));
}

criterion_group!(fill, noise);
criterion_group!(noise_test, rand, rand128);
criterion_main!(noise_test, fill);  