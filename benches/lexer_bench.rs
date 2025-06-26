use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mascal::lexer::tokenize;
mod common;

fn make_inputs() -> Vec<(&'static str, String)> {
    let small  = "()+-*/".repeat(10);
    let medium = "([{}])".repeat(1_000);
    let large  = "([{}])".repeat(10_000);
    vec![("small", small), ("medium", medium), ("large", large)]
}

fn bench_lexer(c: &mut Criterion) {
    let mut group = c.benchmark_group("lexer");
    group.sample_size(50).measurement_time(Duration::from_secs(15));
    for (label, input) in make_inputs() {
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(label),
            &input,
            |b, data| {
                b.iter(|| {
                    // 5) Invoke your lexer
                    let tokens = tokenize(data).unwrap();
                    std::hint::black_box(tokens);
                })
            },
        );
    }
    group.finish();
}

criterion_group!(lexer_benches, bench_lexer);
criterion_main!(lexer_benches);
