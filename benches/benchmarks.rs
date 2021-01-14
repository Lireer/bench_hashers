use ahash::AHasher;
use criterion::{
    criterion_group, criterion_main, AxisScale, BatchSize, BenchmarkId, Criterion,
    PlotConfiguration, Throughput,
};
use fnv::FnvBuildHasher;
use fxhash::FxBuildHasher;
use highway::HighwayBuildHasher;
use metrohash::MetroBuildHasher;
use rand::RngCore;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use std::hash::{BuildHasher, BuildHasherDefault, Hasher};
use twox_hash::RandomXxHashBuilder64;

#[macro_use]
macro_rules! bench_hasher {
    ( $bench_group:expr, $bench_name:literal, $hasher:ty, $n_bytes:expr ) => {
        $bench_group.bench_function(BenchmarkId::new($bench_name, $n_bytes), |bencher| {
            let setup = || {
                let mut bytes = vec![0; $n_bytes];
                let mut rng = Pcg64::seed_from_u64(10);
                rng.fill_bytes(&mut bytes);

                (<$hasher>::default().build_hasher(), bytes)
            };

            bencher.iter_batched(
                setup,
                |(mut hasher, bytes)| {
                    hasher.write(&bytes);
                    hasher.finish();
                },
                BatchSize::SmallInput,
            )
        });
    };
}

fn hashing(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hashing");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for i in 0..=24 {
        let n_bytes = 2usize.pow(i);
        group.throughput(Throughput::Bytes(n_bytes as u64));
        bench_hasher!(group, "aHash", BuildHasherDefault<AHasher>, n_bytes);
        bench_hasher!(group, "FNV", FnvBuildHasher, n_bytes);
        bench_hasher!(group, "FxHash", FxBuildHasher, n_bytes);
        bench_hasher!(group, "HighwayHash", HighwayBuildHasher, n_bytes);
        bench_hasher!(group, "MetroHash", MetroBuildHasher, n_bytes);
        bench_hasher!(group, "xxHash", RandomXxHashBuilder64, n_bytes);
    }
    group.finish()
}

criterion_group!(benches, hashing);
criterion_main!(benches);
