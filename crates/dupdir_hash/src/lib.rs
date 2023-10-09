use std::hash::Hasher as _;
use std::io;

// =================
// === Constants ===
// =================

const KB: usize = 1024;
const BUF_SIZE: usize = 64 * KB;

// ==============
// === Export ===
// ==============

#[cfg(test)]
mod tests;

// ==============
// === Hasher ===
// ==============

pub trait Hasher<const DIGEST_SIZE: usize> {
    fn update(&mut self, data: &[u8]);

    fn finish(self) -> [u8; DIGEST_SIZE];
}

pub fn copy_wide<const DIGEST_SIZE: usize>(
    reader: &mut impl io::Read,
    hasher: &mut impl Hasher<DIGEST_SIZE>,
) -> io::Result<u64> {
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

// =====================
// === define_hasher ===
// =====================

macro_rules! define_hasher {
    (
        $size:ident,
        rust_crypto_hash,
    ) => {
        #[inline]
        fn update(&mut self, data: &[u8]) {
            let Self(inner) = self;
            inner.update(data);
        }

        #[inline]
        fn finish(self) -> [u8; $size] {
            let Self(inner) = self;
            let mut result = [0; $size];
            let hash = inner.finalize();
            let hash = hash.as_ref();
            result.copy_from_slice(hash);
            result
        }
    };
    (
        $size:ident,
        rust_crypto_hash_with_digest,
        $digest:ty,
    ) => {
        paste::paste! {
            #[inline]
            fn update(&mut self, data: &[u8]) {
                let Self(ref mut inner) = self;
                $digest::update(inner, data);
            }

            #[inline]
            fn finish(self) -> [u8; $size] {
                let Self(inner) = self;
                let mut result = [0; $size];
                let hash = $digest::finalize(inner);
                let hash = hash.as_ref();
                result.copy_from_slice(hash);
                result
            }
        }
    };
    (
        $size:ident,
        rust_crypto_hash_extendable,
        $digest:ty,
        $extendable_output:ty,
    ) => {
        paste::paste! {
            #[inline]
            fn update(&mut self, data: &[u8]) {
                let Self(ref mut inner) = self;
                $digest::update(inner, data);
            }

            #[inline]
            fn finish(self) -> [u8; $size] {
                let Self(inner) = self;
                let mut result = [0; $size];
                $extendable_output::finalize_xof_into(inner, &mut result);
                result
            }
        }
    };
    (
        $size:ident,
        std_hasher_per_byte,
    ) => {
        #[inline]
        fn update(&mut self, data: &[u8]) {
            let Self(inner) = self;
            for byte in data {
                inner.write_u8(*byte);
            }
        }

        #[inline]
        fn finish(self) -> [u8; $size] {
            let Self(inner) = self;
            let digest = inner.finish();
            digest.to_be_bytes()
        }
    };
    (
        $size:ident,
        std_hasher,
    ) => {
        #[inline]
        fn update(&mut self, data: &[u8]) {
            let Self(inner) = self;
            inner.update(data);
        }

        #[inline]
        fn finish(self) -> [u8; $size] {
            let Self(mut inner) = self;
            let digest = inner.finish();
            digest.to_be_bytes()
        }
    };
    (
        $size:ident,
        rolling_adler32,
    ) => {
        #[inline]
        fn update(&mut self, data: &[u8]) {
            let Self(inner) = self;
            inner.update_buffer(data);
        }

        #[inline]
        fn finish(self) -> [u8; $size] {
            let Self(inner) = self;
            let digest = inner.hash();
            digest.to_be_bytes()
        }
    };
}

// =================
// === impl_hash ===
// =================

macro_rules! impl_hash {
    (
        @inner,
        $size:ident,
        $ident:ident $(< $( $gen:tt ),+ >)?,
        $digest_size:expr,
        $inner:ty,
        $( $tail:tt )*
    ) => {
        paste::paste! {
            #[cfg(feature = "hash-" $ident)]
            pub const $size: usize = $digest_size;

            #[cfg(feature = "hash-" $ident)]
            #[derive(Default)]
            pub struct $ident$( <$( $gen, )*> )?($inner);

            #[cfg(feature = "hash-" $ident)]
            impl $( <$( $gen ),*> )? Hasher<$size> for $ident $( <$( $gen ),*> )? {
                define_hasher!(
                    $size,
                    $( $tail )*
                );
            }
        }
    };
    (
        $ident:ident
        $( $tail:tt )*
    ) => {
        paste::paste! {
            impl_hash!(
                @inner,
                [<$ident:snake:upper _DIGEST_SIZE>],
                $ident
                $( $tail )*
            );
        }
    };
}

impl_hash!(Blake2bSimd, 64, blake2b_simd::State, rust_crypto_hash,);
impl_hash!(Blake2sSimd, 32, blake2s_simd::State, rust_crypto_hash,);
impl_hash!(
    BeltHash,
    32,
    belt_hash::BeltHash,
    rust_crypto_hash_with_digest,
    belt_hash::Digest,
);
impl_hash!(
    Blake2b,
    64,
    blake2::Blake2b512,
    rust_crypto_hash_with_digest,
    blake2::Digest,
);
impl_hash!(
    Blake2s,
    32,
    blake2::Blake2s256,
    rust_crypto_hash_with_digest,
    blake2::Digest,
);
impl_hash!(
    Blake3,
    32,
    blake3::Hasher,
    rust_crypto_hash_with_digest,
    digest::Digest,
);
impl_hash!(
    Fsb256,
    32,
    fsb::Fsb256,
    rust_crypto_hash_with_digest,
    fsb::Digest,
);
impl_hash!(
    Fsb512,
    64,
    fsb::Fsb512,
    rust_crypto_hash_with_digest,
    fsb::Digest,
);
impl_hash!(
    Groestl256,
    32,
    groestl::Groestl256,
    rust_crypto_hash_with_digest,
    groestl::Digest,
);
impl_hash!(
    Groestl512,
    64,
    groestl::Groestl512,
    rust_crypto_hash_with_digest,
    groestl::Digest,
);
impl_hash!(Md5, 16, md5::Md5, rust_crypto_hash_with_digest, md5::Digest,);
impl_hash!(
    Ripemd160,
    20,
    ripemd::Ripemd160,
    rust_crypto_hash_with_digest,
    ripemd::Digest,
);
impl_hash!(
    Sha256,
    32,
    sha2::Sha256,
    rust_crypto_hash_with_digest,
    sha2::Digest,
);
impl_hash!(
    Sha3_256,
    32,
    sha3::Sha3_256,
    rust_crypto_hash_with_digest,
    sha3::Digest,
);
impl_hash!(
    Sha3_512,
    64,
    sha3::Sha3_512,
    rust_crypto_hash_with_digest,
    sha3::Digest,
);
impl_hash!(
    Sha512,
    64,
    sha2::Sha512,
    rust_crypto_hash_with_digest,
    sha2::Digest,
);
impl_hash!(
    Shabal512,
    64,
    shabal::Shabal512,
    rust_crypto_hash_with_digest,
    shabal::Digest,
);
impl_hash!(Sm3, 32, sm3::Sm3, rust_crypto_hash_with_digest, sm3::Digest,);
impl_hash!(
    Tiger,
    24,
    tiger::Tiger,
    rust_crypto_hash_with_digest,
    tiger::Digest,
);
impl_hash!(
    Tiger2,
    24,
    tiger::Tiger2,
    rust_crypto_hash_with_digest,
    tiger::Digest,
);
impl_hash!(
    Whirlpool,
    64,
    whirlpool::Whirlpool,
    rust_crypto_hash_with_digest,
    whirlpool::Digest,
);
impl_hash!(
    KangarooTwelve256<'a>,
    32,
    k12::KangarooTwelve<'a>,
    rust_crypto_hash_extendable,
    digest::Update,
    digest::ExtendableOutput,
);
impl_hash!(
    KangarooTwelve512<'a>,
    64,
    k12::KangarooTwelve<'a>,
    rust_crypto_hash_extendable,
    digest::Update,
    digest::ExtendableOutput,
);
impl_hash!(Adler32, 8, adler::Adler32, std_hasher_per_byte,);
impl_hash!(Crc32Fast, 8, crc32fast::Hasher, std_hasher_per_byte,);
impl_hash!(FarmHash, 8, farmhash::FarmHasher, std_hasher_per_byte,);
impl_hash!(Fnv, 8, fnv::FnvHasher, std_hasher_per_byte,);
impl_hash!(FxHasher, 8, fxhash::FxHasher, std_hasher_per_byte,);
impl_hash!(FxHasher32, 8, fxhash::FxHasher32, std_hasher_per_byte,);
impl_hash!(FxHasher64, 8, fxhash::FxHasher64, std_hasher_per_byte,);
impl_hash!(FxHasherRustc, 8, rustc_hash::FxHasher, std_hasher_per_byte,);
impl_hash!(
    MetroHash128,
    8,
    metrohash::MetroHash128,
    std_hasher_per_byte,
);
impl_hash!(MetroHash64, 8, metrohash::MetroHash64, std_hasher_per_byte,);
impl_hash!(Seahash, 8, seahash::SeaHasher, std_hasher_per_byte,);
impl_hash!(Siphash, 8, siphasher::sip::SipHasher, std_hasher_per_byte,);
impl_hash!(T1ha, 8, t1ha::T1haHasher, std_hasher_per_byte,);
impl_hash!(Xxh2_32, 4, xxhash2::State32, std_hasher_per_byte,);
impl_hash!(Xxh2_64, 8, xxhash2::State64, std_hasher_per_byte,);
impl_hash!(Xxh3, 8, xxhash_rust::xxh3::Xxh3, std_hasher_per_byte,);
impl_hash!(Xxh64, 8, xxhash_rust::xxh64::Xxh64, std_hasher_per_byte,);
impl_hash!(Xxh64TwoHash, 8, twox_hash::XxHash64, std_hasher_per_byte,);
impl_hash!(T1ha2, 8, t1ha::T1ha2Hasher, std_hasher,);
impl_hash!(Adler32Rolling, 4, adler32::RollingAdler32, rolling_adler32,);
