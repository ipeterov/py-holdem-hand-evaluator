use criterion::{criterion_group, criterion_main, Criterion};
use holdem_hand_evaluator::calculate_equity;

fn equity(c: &mut Criterion) {
    c.bench_function("equity", |b| {
        b.iter(|| calculate_equity(String::from("Ac Kc"), String::from("2h 3h"), 1, 50000))
    });
}

criterion_group!(benches, equity);
criterion_main!(benches);
