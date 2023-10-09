use rand::RngCore as _;
use std::fs;
use std::io::Write as _;
use std::time;



// =================
// === Constants ===
// =================

const KB: usize = 1000; // 1000 Bytes
const MB: usize = 1000 * KB; // 1000 * 1000 Bytes
const MEMORY_SIZES: [usize; 1] = [2 * MB];
const FILE_SIZES: [usize; 1] = [50 * KB];
const WARM_UP_TIME_MILLIS: u64 = 100; // Defaults to 3000
const MEASUREMENT_TIME_MILLIS: u64 = 1000; // Defaults to 10000?
const SAMPLE_SIZE: usize = 10; // Defaults to 1000?
const NUMBER_OF_SAMPLES: usize = 10; // Defaults to 100



// ==================
// === Benchmarks ===
// ==================

// === impl_bench_group_hash ===

macro_rules! impl_bench_group_hash {
    ($( $ty:ident ),*,) => {
        impl_bench_group_hash!(@ $( (stringify!($ty), $ty), )* );
    };
    (@ $( ($name:expr, $ty:ident) ),*,) => {
        paste::paste! {
            fn hash(c: &mut criterion::Criterion) {
                let bufs = MEMORY_SIZES.iter().map(|&size| {
                    let mut buf = vec![0; size];
                    rand::thread_rng().fill_bytes(&mut buf);
                    buf
                }).collect::<Vec<_>>();

                let mut group = c.benchmark_group("hash");
                for (idx, size) in MEMORY_SIZES.iter().enumerate() {
                    let buf = &bufs[idx][..];
                    group.throughput(criterion::Throughput::Bytes(*size as u64));
                    $(
                    #[cfg(feature = "hash-" $ty)]
                    group.bench_with_input(criterion::BenchmarkId::new($name, size), size, |b, &_size| {
                        b.iter(|| {
                            let mut buf = criterion::black_box(buf);
                            let mut reader = &mut buf;
                            let mut hasher = dupdir_hash::$ty::default();
                            dupdir_hash::Hasher::update(&mut hasher, criterion::black_box(&mut reader));
                            dupdir_hash::Hasher::finish(hasher)
                        })
                    });
                    )*
                }
                group.finish();
            }
        }
    };
}

impl_bench_group_hash!(
    Adler32,
    Adler32Rolling,
    BeltHash,
    Blake2b,
    Blake2bSimd,
    Blake2s,
    Blake2sSimd,
    Blake3,
    Crc32Fast,
    FarmHash,
    Fnv,
    Fsb256,
    Fsb512,
    FxHasher,
    FxHasher32,
    FxHasher64,
    FxHasherRustc,
    Groestl256,
    Groestl512,
    Md5,
    MetroHash64,
    MetroHash128,
    Ripemd160,
    Seahash,
    Sha256,
    Sha512,
    Sha3_256,
    Sha3_512,
    Shabal512,
    Siphash,
    Sm3,
    T1ha,
    T1ha2,
    Tiger,
    Tiger2,
    Whirlpool,
    Xxh3,
    Xxh64,
    Xxh64TwoHash,
    Xxh2_32,
    Xxh2_64,
);

// === impl_bench_group_hash_file ===

macro_rules! impl_bench_group_hash_file {
    ($( $ty:ident ),*,) => {
        impl_bench_group_hash_file!(@ $( (stringify!($ty), $ty), )* );
    };
    (@ $( ($name:expr, $ty:ident) ),*,) => {
        paste::paste! {
            fn hash_file(c: &mut criterion::Criterion) {
                let bufs = FILE_SIZES.iter().map(|&size| {
                    let mut buf = vec![0; size];
                    rand::thread_rng().fill_bytes(&mut buf);
                    buf
                }).collect::<Vec<_>>();

                let tmp_dir = tempdir::TempDir::new("hashes").unwrap();

                let files = bufs.iter().zip(FILE_SIZES.iter()).map(|(buf, &size)| {
                    let path = tmp_dir.path().join(format!("{size}"));
                    let mut file = std::fs::File::create(&path).unwrap();
                    file.write_all(buf).unwrap();
                    path
                }).collect::<Vec<_>>();

                let mut group = c.benchmark_group("hash_file");
                for (idx, size) in FILE_SIZES.iter().enumerate() {
                    let path = files[idx].as_path();
                    group.throughput(criterion::Throughput::Bytes(*size as u64));
                    $(
                    #[cfg(feature = "hash-" $ty)]
                    group.bench_with_input(criterion::BenchmarkId::new($name, size), size, |b, &_size| {
                        b.iter(|| {
                            let path = criterion::black_box(path);
                            let mut reader = fs::File::open(path).unwrap();
                            let mut hasher = dupdir_hash::$ty::default();
                            dupdir_hash::copy_wide(
                                criterion::black_box(&mut reader),
                                criterion::black_box(&mut hasher),
                            ).unwrap();
                            dupdir_hash::Hasher::finish(hasher)
                        })
                    });
                    )*
                }
                group.finish();
            }
        }
    };
}

impl_bench_group_hash_file!(
    Adler32,
    Adler32Rolling,
    BeltHash,
    Blake2b,
    Blake2bSimd,
    Blake2s,
    Blake2sSimd,
    Blake3,
    Crc32Fast,
    FarmHash,
    Fnv,
    Fsb256,
    Fsb512,
    FxHasher,
    FxHasher32,
    FxHasher64,
    FxHasherRustc,
    Groestl256,
    Groestl512,
    Md5,
    MetroHash64,
    MetroHash128,
    Ripemd160,
    Seahash,
    Sha256,
    Sha512,
    Sha3_256,
    Sha3_512,
    Shabal512,
    Siphash,
    Sm3,
    T1ha,
    T1ha2,
    Tiger,
    Tiger2,
    Whirlpool,
    Xxh3,
    Xxh64,
    Xxh64TwoHash,
    Xxh2_32,
    Xxh2_64,
);

criterion::criterion_group!{
    name = benches;
    config = criterion::Criterion::default()
        .plotting_backend(criterion::PlottingBackend::Gnuplot)
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(time::Duration::from_millis(WARM_UP_TIME_MILLIS))
        .measurement_time(time::Duration::from_millis(MEASUREMENT_TIME_MILLIS))
        .nresamples(NUMBER_OF_SAMPLES)
        .with_plots();
    targets = hash, hash_file
}
criterion::criterion_main!(benches);
