use investigator::Hasher as TRAIT_Hasher;
use rand::RngCore as TRAIT_RngCore;
use std::fs;
use std::io::Write as TRAIT_Write;
use std::path;



// =================
// === Constants ===
// =================

const KB: usize = 1000; // 1000 Bytes
const MB: usize = 1000 * KB; // 1000 * 1000 Bytes
const SIZES: [usize; 5] = [KB, 10 * KB, 100 * KB, MB, 10 * MB];



// ==================
// === Benchmarks ===
// ==================

// === impl_bench_group ===

macro_rules! impl_bench_group {
    ($( ($name:expr, $ty:ident) ),*,) => {
        fn hashes(c: &mut criterion::Criterion) {
            let bufs = SIZES.iter().map(|&size| {
                let mut buf = vec![0; size];
                rand::thread_rng().fill_bytes(&mut buf);
                buf
            }).collect::<Vec<_>>();
            $(
                let mut group = c.benchmark_group($name);
                for (idx, size) in SIZES.iter().enumerate() {
                    let buf = &bufs[idx];
                    group.throughput(criterion::Throughput::Bytes(*size as u64));
                    group.bench_with_input(criterion::BenchmarkId::from_parameter(size), size, |b, &size| {
                        b.iter(|| {
                            assert_eq!(buf.len(), size);
                            let reader = &mut &buf[..];
                            let input = criterion::black_box(reader);
                            investigator::$ty::from_reader(input).unwrap();
                        })
                    });
                }
                group.finish();
            )*
        }
    };
}

impl_bench_group!(
    //("adler32", Adler32),
    ("adler32rolling", Adler32Rolling),
    //("belthash", BeltHash),
    //("blake2b", Blake2b),
    ("blake2b_simd", Blake2bSimd),
    //("blake2s", Blake2s),
    //("blake2s_simd", Blake2sSimd),
    ("blake3", Blake3),
    //("crc32fast", Crc32Fast),
    ("farm_hash", FarmHash),
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
    ("metrohash64", MetroHash64),
    ("metrohash128", MetroHash128),
    //("ripemd160", Ripemd160),
    //("seahash", Seahash),
    ("sha256", Sha256),
    ("sha512", Sha512),
    //("sha3_256", Sha3_256),
    //("sha3_512", Sha3_512),
    //("shabal512", Shabal512),
    ("siphash", Siphash),
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
    //("xxh2_64", Xxh2_64)
);

criterion::criterion_group!(benches, hashes);
criterion::criterion_main!(benches);
