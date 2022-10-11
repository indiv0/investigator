use std::hash::Hasher as TRAIT_Hasher;
use std::io;
use sha2::Digest as TRAIT_Digest;
use sha2::digest::ExtendableOutput as TRAIT_ExtendableOutput;



// =================
// === Constants ===
// =================

const KB: usize = 1024;
const BUF_SIZE: usize = 64 * KB;

const ADLER32_DIGEST_SIZE: usize = 8;
const ADLER32_ROLLING_DIGEST_SIZE: usize = 4;
const BELT_HASH_DIGEST_SIZE: usize = 32;
const BLAKE2B_DIGEST_SIZE: usize = 64;
const BLAKE2S_DIGEST_SIZE: usize = 32;
const BLAKE3_DIGEST_SIZE: usize = 32;
const CRC32_DIGEST_SIZE: usize = 8;
const FARMHASH_DIGEST_SIZE: usize = 8;
const FNV_DIGEST_SIZE: usize = 8;
const FSB_256_DIGEST_SIZE: usize = 32;
const FSB_512_DIGEST_SIZE: usize = 64;
const FXHASHER_DIGEST_SIZE: usize = 8;
const GROESTL_256_DIGEST_SIZE: usize = 32;
const GROESTL_512_DIGEST_SIZE: usize = 64;
const K12_256_DIGEST_SIZE: usize = 32;
const K12_512_DIGEST_SIZE: usize = 64;
const MD5_DIGEST_SIZE: usize = 16;
const METROHASH128_DIGEST_SIZE: usize = 8;
const METROHASH64_DIGEST_SIZE: usize = 8;
const RIPEMD_160_DIGEST_SIZE: usize = 20;
const SEAHASH_DIGEST_SIZE: usize = 8;
const SHA256_DIGEST_SIZE: usize = 32;
const SHA3_256_DIGEST_SIZE: usize = 32;
const SHA3_512_DIGEST_SIZE: usize = 64;
const SHA512_DIGEST_SIZE: usize = 64;
const SHABAL_512_DIGEST_SIZE: usize = 64;
const SIPHASH_DIGEST_SIZE: usize = 8;
const SM3_DIGEST_SIZE: usize = 32;
const T1HA_DIGEST_SIZE: usize = 8;
const TIGER2_DIGEST_SIZE: usize = 24;
const TIGER_DIGEST_SIZE: usize = 24;
const WHIRLPOOL_DIGEST_SIZE: usize = 64;
const XXH2_32_DIGEST_SIZE: usize = 4;
const XXH2_64_DIGEST_SIZE: usize = 8;
const XXH_3_DIGEST_SIZE: usize = 8;
const XXH_64_DIGEST_SIZE: usize = 8;



// ==============
// === Hasher ===
// ==============

pub trait Hasher<const DIGEST_SIZE: usize>: Default {
    fn update(&mut self, data: &[u8]);

    fn finish(self) -> [u8; DIGEST_SIZE];
}

pub fn copy_wide<const DIGEST_SIZE: usize>(reader: &mut impl io::Read, hasher: &mut impl Hasher<DIGEST_SIZE>) -> io::Result<u64> {
    let mut buffer = [0u8; BUF_SIZE];
    let mut total = 0;
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => return Ok(total),
            Ok(n) => {
                hasher.update(&buffer[..n]);
                total += n as u64;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
}



// =============================
// === impl_rust_crypto_hash ===
// =============================

macro_rules! impl_rust_crypto_hash {
    ($( ($ident:ident, $inner:ty, $digest_size:expr) ),*) => {
        $(
            #[derive(Default)]
            pub struct $ident($inner);
            
            impl Hasher<$digest_size> for $ident {
                #[inline]
                fn update(&mut self, data: &[u8]) {
                    self.0.update(data);
                }

                #[inline]
                fn finish(self) -> [u8; $digest_size] {
                    let mut result = [0; $digest_size];
                    result.copy_from_slice(self.0.finalize().as_ref());
                    result
                }
            }
        )*
    };
}

impl_rust_crypto_hash!(
    (BeltHash, belt_hash::BeltHash, BELT_HASH_DIGEST_SIZE),
    (Blake2b, blake2::Blake2b512, BLAKE2B_DIGEST_SIZE),
    (Blake2bSimd, blake2b_simd::State, BLAKE2B_DIGEST_SIZE),
    (Blake2s, blake2::Blake2s256, BLAKE2S_DIGEST_SIZE),
    (Blake2sSimd, blake2s_simd::State, BLAKE2S_DIGEST_SIZE),
    (Blake3, blake3::Hasher, BLAKE3_DIGEST_SIZE),
    (Fsb256, fsb::Fsb256, FSB_256_DIGEST_SIZE),
    (Fsb512, fsb::Fsb512, FSB_512_DIGEST_SIZE),
    (Groestl256, groestl::Groestl256, GROESTL_256_DIGEST_SIZE),
    (Groestl512, groestl::Groestl512, GROESTL_512_DIGEST_SIZE),
    (Md5, md5::Md5, MD5_DIGEST_SIZE),
    (Ripemd160, ripemd::Ripemd160, RIPEMD_160_DIGEST_SIZE),
    (Sha256, sha2::Sha256, SHA256_DIGEST_SIZE),
    (Sha3_256, sha3::Sha3_256, SHA3_256_DIGEST_SIZE),
    (Sha3_512, sha3::Sha3_512, SHA3_512_DIGEST_SIZE),
    (Sha512, sha2::Sha512, SHA512_DIGEST_SIZE),
    (Shabal512, shabal::Shabal512, SHABAL_512_DIGEST_SIZE),
    (Sm3, sm3::Sm3, SM3_DIGEST_SIZE),
    (Tiger, tiger::Tiger, TIGER_DIGEST_SIZE),
    (Tiger2, tiger::Tiger2, TIGER2_DIGEST_SIZE),
    (Whirlpool, whirlpool::Whirlpool, WHIRLPOOL_DIGEST_SIZE)
);

// ========================================
// === impl_rust_crypto_hash_extendable ===
// ========================================

macro_rules! impl_rust_crypto_hash_extendable {
    ($( ($ident:ident, $inner:ty, $digest_size:expr) ),*) => {
        $(
            #[derive(Default)]
            pub struct $ident($inner);
            
            impl Hasher<$digest_size> for $ident {
                #[inline]
                fn update(&mut self, data: &[u8]) {
                    digest::Update::update(&mut self.0, data);
                }

                #[inline]
                fn finish(self) -> [u8; $digest_size] {
                    let mut result = [0; $digest_size];
                    self.0.finalize_xof_into(&mut result);
                    result
                }
            }
        )*
    };
}

impl_rust_crypto_hash_extendable!(
    (KangarooTwelve256, k12::KangarooTwelve, K12_256_DIGEST_SIZE),
    (KangarooTwelve512, k12::KangarooTwelve, K12_512_DIGEST_SIZE)
);



// ================================
// === impl_std_hasher_per_byte ===
// ================================

macro_rules! impl_std_hasher_per_byte {
    ($( ($ident:ident, $inner:ty, $digest_size:expr) ),*) => {
        $(
            #[derive(Default)]
            pub struct $ident($inner);
            
            impl Hasher<$digest_size> for $ident {
                #[inline]
                fn update(&mut self, data: &[u8]) {
                    for byte in data {
                        self.0.write_u8(*byte);
                    }
                }

                #[inline]
                fn finish(self) -> [u8; $digest_size] {
                    let digest = self.0.finish();
                    digest.to_be_bytes()
                }
            }
        )*
    };
}

impl_std_hasher_per_byte!(
    (Adler32, adler::Adler32, ADLER32_DIGEST_SIZE),
    (Crc32Fast, crc32fast::Hasher, CRC32_DIGEST_SIZE),
    (FarmHash, farmhash::FarmHasher, FARMHASH_DIGEST_SIZE),
    (Fnv, fnv::FnvHasher, FNV_DIGEST_SIZE),
    (FxHasher, fxhash::FxHasher, FXHASHER_DIGEST_SIZE),
    (FxHasher32, fxhash::FxHasher32, FXHASHER_DIGEST_SIZE),
    (FxHasher64, fxhash::FxHasher64, FXHASHER_DIGEST_SIZE),
    (FxHasherRustc, rustc_hash::FxHasher, FXHASHER_DIGEST_SIZE),
    (MetroHash128, metrohash::MetroHash128, METROHASH128_DIGEST_SIZE),
    (MetroHash64, metrohash::MetroHash64, METROHASH64_DIGEST_SIZE),
    (Seahash, seahash::SeaHasher, SEAHASH_DIGEST_SIZE),
    (Siphash, siphasher::sip::SipHasher, SIPHASH_DIGEST_SIZE),
    (T1ha, t1ha::T1haHasher, T1HA_DIGEST_SIZE),
    (Xxh2_32, xxhash2::State32, XXH2_32_DIGEST_SIZE),
    (Xxh2_64, xxhash2::State64, XXH2_64_DIGEST_SIZE),
    (Xxh3, xxhash_rust::xxh3::Xxh3, XXH_3_DIGEST_SIZE),
    (Xxh64, xxhash_rust::xxh64::Xxh64, XXH_64_DIGEST_SIZE),
    (Xxh64TwoHash, twox_hash::XxHash64, XXH_64_DIGEST_SIZE)
);



// =======================
// === impl_std_hasher ===
// =======================

macro_rules! impl_std_hasher {
    ($( ($ident:ident, $inner:ty, $digest_size:expr) ),*) => {
        $(
            #[derive(Default)]
            pub struct $ident($inner);
            
            impl Hasher<$digest_size> for $ident {
                #[inline]
                fn update(&mut self, data: &[u8]) {
                    self.0.update(data);
                }

                #[inline]
                fn finish(mut self) -> [u8; $digest_size] {
                    let digest = self.0.finish();
                    digest.to_be_bytes()
                }
            }
        )*
    };
}

impl_std_hasher!(
    (T1ha2, t1ha::T1ha2Hasher, T1HA_DIGEST_SIZE)
);


// ===================================
// === impl_rolling_adler32_hasher ===
// ===================================

macro_rules! impl_rolling_adler32_hasher {
    ($( ($ident:ident, $inner:ty, $digest_size:expr) ),*) => {
        $(
            #[derive(Default)]
            pub struct $ident($inner);
            
            impl Hasher<$digest_size> for $ident {
                fn update(&mut self, data: &[u8]) {
                    self.0.update_buffer(data);
                }

                fn finish(self) -> [u8; $digest_size] {
                    let digest = self.0.hash();
                    digest.to_be_bytes()
                }
            }
        )*
    };
}

impl_rolling_adler32_hasher!(
    (Adler32Rolling, adler32::RollingAdler32, ADLER32_ROLLING_DIGEST_SIZE)
);



// ============
// === Test ===
// ============

#[cfg(test)]
mod test {
    use std::fs;
    use std::io::Write as TRAIT_Write;
    use std::path;



    // =================
    // === Constants ===
    // =================

    const HELLO_WORLD_DATA: &str = "benches/hello_world";



    // =================
    // === impl_test ===
    // =================

    macro_rules! impl_test {
        ($ident:ident, $ty:ident, $expected:expr) => {
            mod $ident {
                use crate::Hasher as TRAIT_Hasher;
                use std::fs;

                #[test]
                fn hash_bytes() {
                    let mut bytes = &b"Hello, world!"[..];
                    let mut hasher = crate::$ty::default();
                    crate::copy_wide(&mut bytes, &mut hasher).unwrap();
                    let hash = hasher.finish();
                    assert_eq!(hex::encode(hash), $expected);
                }

                #[test]
                fn hash_file() {
                    let path = super::create_file_with_hello_world_data();
                    let mut reader = fs::File::open(&path).unwrap();
                    let mut hasher = crate::$ty::default();
                    crate::copy_wide(&mut reader, &mut hasher).unwrap();
                    let hash = hasher.finish();
                    assert_eq!(hex::encode(hash), $expected);
                }
            }
        };
    }

    impl_test!(adler32, Adler32, "00000000205e048a"); // FIXME [NP]: too long
    impl_test!(adler32_rolling, Adler32Rolling, "205e048a");
    impl_test!(belt_hash, BeltHash, "249dc153df2f7bf8f9b0e5400c1c8deff429b2ff013247a98f8a2cbcec995ade");
    impl_test!(blake2b, Blake2b, "a2764d133a16816b5847a737a786f2ece4c148095c5faa73e24b4cc5d666c3e45ec271504e14dc6127ddfce4e144fb23b91a6f7b04b53d695502290722953b0f");
    impl_test!(blake2b_simd, Blake2bSimd, "a2764d133a16816b5847a737a786f2ece4c148095c5faa73e24b4cc5d666c3e45ec271504e14dc6127ddfce4e144fb23b91a6f7b04b53d695502290722953b0f");
    impl_test!(blake2s, Blake2s, "30d8777f0e178582ec8cd2fcdc18af57c828ee2f89e978df52c8e7af078bd5cf");
    impl_test!(blake2s_simd, Blake2sSimd, "30d8777f0e178582ec8cd2fcdc18af57c828ee2f89e978df52c8e7af078bd5cf");
    impl_test!(blake3, Blake3, "ede5c0b10f2ec4979c69b52f61e42ff5b413519ce09be0f14d098dcfe5f6f98d");
    impl_test!(crc32fast, Crc32Fast, "00000000ebe6c6e6"); // FIXME [NP]: too long
    impl_test!(farm_hash, FarmHash, "307c26b3e0789a47");
    impl_test!(fnv, Fnv, "38d1334144987bf4");
    impl_test!(fsb256, Fsb256, "b75c250c35ccebeb67d6e9a5173e638a0ebc2545674c2da17fc0275a62b3f69c");
    impl_test!(fsb512, Fsb512, "75186f19cd5b7c57d4be1247d7f39bdc681ec796cebb5668ea2eb4eb233294071ca915e56887549464dc7d3e077f08492e6ed0d382943efbeab20e191a5f09d0");
    impl_test!(fx_hasher, FxHasher, "562dc0284e81dff2");
    impl_test!(fx_hasher_32, FxHasher32, "00000000c5b0ab5f"); // FIXME [NP]: too long
    impl_test!(fx_hasher_64, FxHasher64, "562dc0284e81dff2");
    impl_test!(fx_hasher_rustc, FxHasherRustc, "562dc0284e81dff2");
    impl_test!(groestl256, Groestl256, "63e4ab2044e38c1fb1725313f2229e038926af839c86eaf96553027d2c851e18");
    impl_test!(groestl512, Groestl512, "b60658e723a8eb1743823a8002175486bc24223ba3dc6d8cb435a948f6d2b9744ac9e307e1d38021ea18c4d536d28fc23491d7771a5a5b0d02ffad9a073dcc28");
    impl_test!(kangaroo_twelve256, KangarooTwelve256, "2a7eccaa09ff7e30cb1413bda28dad7f90759f22fc63535369bf17595b1166af");
    impl_test!(kangaroo_twelve512, KangarooTwelve512, "2a7eccaa09ff7e30cb1413bda28dad7f90759f22fc63535369bf17595b1166af5d6edd1b483c5eee16d5291ac37c454ff1f26d8ce176a7c73a79232e5b2e402f");
    impl_test!(md5, Md5, "6cd3556deb0da54bca060b4c39479839");
    impl_test!(metro_hash_128, MetroHash128, "5930f69e4971f2c0");
    impl_test!(metro_hash_64, MetroHash64, "fc8b20d0f74c7aa7");
    impl_test!(ripemd160, Ripemd160, "58262d1fbdbe4530d8865d3518c6d6e41002610f");
    impl_test!(seahash, Seahash, "0682402aaca36178");
    impl_test!(sha256, Sha256, "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3");
    impl_test!(sha3_256, Sha3_256, "f345a219da005ebe9c1a1eaad97bbf38a10c8473e41d0af7fb617caa0c6aa722");
    impl_test!(sha3_512, Sha3_512, "8e47f1185ffd014d238fabd02a1a32defe698cbf38c037a90e3c0a0a32370fb52cbd641250508502295fcabcbf676c09470b27443868c8e5f70e26dc337288af");
    impl_test!(sha512, Sha512, "c1527cd893c124773d811911970c8fe6e857d6df5dc9226bd8a160614c0cd963a4ddea2b94bb7d36021ef9d865d5cea294a82dd49a0bb269f51f6e7a57f79421");
    impl_test!(shabal512, Shabal512, "7048f0a589339d2d26890701ed3b2d1ed7c8dd1ac37fec517c7a8c39d5d51548e96ea8dfaceb5b99f9d1db3b18a7652e0412348ebfd61d32d755d6098bff8cb3");
    impl_test!(siphash, Siphash, "ae5020d7cf49d14f");
    impl_test!(sm3, Sm3, "e3bca101b496880c3653dad85861d0e784b00a8c18f7574472d156060e9096bf");
    impl_test!(t1ha, T1ha, "936f6a215ae53484");
    impl_test!(t1ha2, T1ha2, "345623cc534aa878");
    impl_test!(tiger, Tiger, "b5e5dd73a5894236937084131bb845189cdc5477579b9f36");
    impl_test!(tiger2, Tiger2, "5d76a0e497c8cb50616ce102d7c0d9d4c5e6260b1e8bac4e");
    impl_test!(whirlpool, Whirlpool, "a1a8703be5312b139b42eb331aa800ccaca0c34d58c6988e44f45489cfb16beb4b6bf0ce20be1db22a10b0e4bb680480a3d2429e6c483085453c098b65852495");
    impl_test!(xxh3, Xxh3, "f3c34bf11915e869");
    impl_test!(xxh64, Xxh64, "f58336a78b6f9476");
    impl_test!(xxh64_twohash, Xxh64TwoHash, "f58336a78b6f9476");
    impl_test!(xxhash2_32, Xxh2_32, "31b7405d");
    impl_test!(xxhash2_64, Xxh2_64, "f58336a78b6f9476");
    
    // === create_file_with_hello_world_data ===

    fn create_file_with_hello_world_data() -> path::PathBuf {
        let path = path::Path::new(HELLO_WORLD_DATA);
    
        if !path.exists() {
            let mut file = fs::File::create(path).unwrap();
            file.write_all(&b"Hello, world!"[..]).unwrap();
        }
    
        path.to_path_buf()
    }
}
