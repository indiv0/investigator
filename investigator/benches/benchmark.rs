use rand::RngCore as TRAIT_RngCore;
use std::fs;
use std::io::Write as TRAIT_Write;



// =================
// === Constants ===
// =================

const KB: usize = 1000; // 1000 Bytes
const MB: usize = 1000 * KB; // 1000 * 1000 Bytes
const MEMORY_SIZES: [usize; 2] = [MB, 2 * MB];
const FILE_SIZES: [usize; 4] = [KB, 5 * KB, 10 * KB, 50 * KB];



// ==================
// === Benchmarks ===
// ==================

// === impl_bench_group_hash ===

macro_rules! impl_bench_group_hash {
    ($( ($name:expr, $ty:ident) ),*,) => {
        paste::paste! {
            fn hash(c: &mut criterion::Criterion) {
                panic!("foo");
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
                            let mut hasher = investigator::$ty::default();
                            investigator::Hasher::update(&mut hasher, criterion::black_box(&mut reader));
                            investigator::Hasher::finish(hasher)
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
    //("adler32", Adler32),
    ("adler32rolling", Adler32Rolling),
    //("belthash", BeltHash),
    //("blake2b", Blake2b),
    //("blake2b_simd", Blake2bSimd),
    //("blake2s", Blake2s),
    //("blake2s_simd", Blake2sSimd),
    ("blake3", Blake3),
    //("crc32fast", Crc32Fast),
    //("farm_hash", FarmHash),
    //("fnv", Fnv),
    //("fsb256", Fsb256),
    //("fsb512", Fsb512),
    //("fxhasher", FxHasher),
    //("fxhasher32", FxHasher32),
    //("fxhasher64", FxHasher64),
    //("fxhasher_rustc", FxHasherRustc),
    //("groestl256", Groestl256),
    //("groestl512", Groestl512),
    //("md5", Md5),
    //("metrohash64", MetroHash64),
    //("metrohash128", MetroHash128),
    //("ripemd160", Ripemd160),
    //("seahash", Seahash),
    ("sha256", Sha256),
    //("sha512", Sha512),
    //("sha3_256", Sha3_256),
    //("sha3_512", Sha3_512),
    //("shabal512", Shabal512),
    //("siphash", Siphash),
    //("sm3", Sm3),
    //("t1ha", T1ha),
    ("t1ha2", T1ha2),
    //("tiger", Tiger),
    //("tiger2", Tiger2),
    //("whirlpool", Whirlpool),
    //("xxh3", Xxh3),
    //("xxh64", Xxh64),
    //("xxh64_twohash", Xxh64TwoHash),
    //("xxh2_32", Xxh2_32),
    //("xxh2_64", Xxh2_64),
);

// === impl_bench_group_hash_file ===

macro_rules! impl_bench_group_hash_file {
    ($( ($name:expr, $ty:ident) ),*,) => {
        paste::paste! {
            fn hash_file(c: &mut criterion::Criterion) {
                let bufs = FILE_SIZES.iter().map(|&size| {
                    let mut buf = vec![0; size];
                    rand::thread_rng().fill_bytes(&mut buf);
                    buf
                }).collect::<Vec<_>>();

                let tmp_dir = tempdir::TempDir::new("investigator").unwrap();

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
                            let mut hasher = investigator::$ty::default();
                            investigator::copy_wide(
                                criterion::black_box(&mut reader),
                                criterion::black_box(&mut hasher),
                            ).unwrap();
                            investigator::Hasher::finish(hasher)
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
    ("t1ha2", T1ha2),
);

criterion::criterion_group!{
    name = benches;
    config = criterion::Criterion::default()
        .plotting_backend(criterion::PlottingBackend::Gnuplot)
        //.sample_size(10)
        //.warm_up_time(time::Duration::from_millis(1))
        //.measurement_time(time::Duration::from_millis(1))
        //.nresamples(1000)
        .with_plots();
    targets = hash, hash_file
}
criterion::criterion_main!(benches);
