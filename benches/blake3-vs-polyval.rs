use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

#[cfg(feature = "cpb-bench")]
use criterion_cycles_per_byte::CyclesPerByte as Measurement;

#[cfg(not(feature = "cpb-bench"))]
use criterion::measurement::WallTime as Measurement;

use polyval::universal_hash::UniversalHash;

const KB: usize = 1024;

fn bench_blake3(c: &mut Criterion<Measurement>) {
    let mut group = c.benchmark_group("blake3");

    let mut hasher = blake3::Hasher::new();
    let buf = vec![0; 1024 * 1024];
    for s in &[1, 4, 16, 256, 1024] {
        group.throughput(Throughput::Bytes((*s * KB) as u64));
        group.bench_function(
            BenchmarkId::new("hash", s * KB),
            |b| b.iter(
                || { hasher.update(&buf[..s * KB]); }
            )
        );
    }

    group.finish();
}

fn bench_polyval(c: &mut Criterion<Measurement>) {
    let mut group = c.benchmark_group("polyval");

    let mut hasher = polyval::Polyval::new(&Default::default());
    let buf = vec![0; 1024 * 1024];
    for s in &[1, 4, 16, 256, 1024] {
        group.throughput(Throughput::Bytes((*s * KB) as u64));
        group.bench_function(
            BenchmarkId::new("hash", s * KB),
            |b| b.iter(
                || hasher.update_padded(&buf[..s * KB])
            )
        );
    }

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(Measurement);
    targets = bench_blake3, bench_polyval
);
criterion_main!(benches);
