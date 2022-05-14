use criterion::{black_box, criterion_group, criterion_main, Criterion};
use letter_freq::logic::{
    count_letters_fx_hash_map_fold, count_letters_fx_hash_map_forloop, count_letters_hash_map_fold,
    count_letters_hash_map_forloop, count_letters_parallel, count_letters_with_rayon,
};

fn criterion_benchmark(c: &mut Criterion) {
    const TEXT: &str = include_str!("./input");

    c.bench_function("letter counting - HashMap, Fold", |b| {
        b.iter(|| count_letters_hash_map_fold(black_box(TEXT)))
    });

    c.bench_function("letter counting - HashMap, ForLoop", |b| {
        b.iter(|| count_letters_hash_map_forloop(black_box(TEXT)))
    });

    c.bench_function("letter counting - FxHashMap, Fold", |b| {
        b.iter(|| count_letters_fx_hash_map_fold(black_box(TEXT)))
    });

    c.bench_function("letter counting - FxHashMap, ForLoop", |b| {
        b.iter(|| count_letters_fx_hash_map_forloop(black_box(TEXT)))
    });

    c.bench_function("letter counting - FxHashMap, ForLoop, 4 threads", |b| {
        b.iter(|| count_letters_parallel(black_box(TEXT), black_box(4)))
    });

    c.bench_function("letter counting - FxHashMap, ForLoop, 8 threads", |b| {
        b.iter(|| count_letters_parallel(black_box(TEXT), black_box(8)))
    });

    c.bench_function("letter counting - FxHashMap, Rayon", |b| {
        b.iter(|| count_letters_with_rayon(black_box(TEXT)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
