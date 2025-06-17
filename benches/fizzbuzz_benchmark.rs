use criterion::{criterion_group, criterion_main, Criterion};
use fizzbuzz::generator::Generator;
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let fizzbuzz_matcher = vec![("Fizz".to_string(), 3), ("Buzz".to_string(), 5)];

    let fizzbuzzbazz_matcher = vec![
        ("Fizz".to_string(), 3),
        ("Buzz".to_string(), 5),
        ("Bazz".to_string(), 7),
    ];

    c.bench_function("standard fizzbuzz", |b| {
        b.iter(|| {
            let generator =
                Generator::new(None, black_box(100), black_box(fizzbuzz_matcher.clone()));
            generator.collect::<String>()
        })
    });

    c.bench_function("10K digits fizzbuzz", |b| {
        b.iter(|| {
            let generator =
                Generator::new(None, black_box(10_000), black_box(fizzbuzz_matcher.clone()));
            generator.collect::<String>()
        })
    });

    c.bench_function("fizzbuzzbazz", |b| {
        b.iter(|| {
            let generator = Generator::new(
                None,
                black_box(100),
                black_box(fizzbuzzbazz_matcher.clone()),
            );
            generator.collect::<String>()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
