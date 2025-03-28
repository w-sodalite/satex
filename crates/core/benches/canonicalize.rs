use criterion::{black_box, criterion_group, criterion_main, Criterion};
use satex_core::util::canonicalize;

fn _bench_canonicalize(c: &mut Criterion) {
    c.bench_function("canonicalize", |b| {
        b.iter(|| {
            let path = canonicalize("/api/v1/resource");
            black_box(path)
        })
    });

    c.bench_function("canonicalize_dot1", |b| {
        b.iter(|| {
            let path = canonicalize("/api/v1/./resource");
            black_box(path)
        })
    });

    c.bench_function("canonicalize_dot2", |b| {
        b.iter(|| {
            let path = canonicalize("/api/v1/../resource");
            black_box(path)
        })
    });
}

criterion_group!(bench_canonicalize, _bench_canonicalize);

criterion_main!(bench_canonicalize);
