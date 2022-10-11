use investigator::Hasher as TRAIT_Hasher;
use std::env;
use std::fs;
use std::io;



// =============
// === Input ===
// =============

enum Input<'a> {
    Stdin {
        path: &'a str,
        stdin: io::StdinLock<'a>,
    },
    File {
        path: &'a str,
        file: fs::File,
    },
}

// === Main `impl` ===

impl<'a> Input<'a> {
    fn new(path: Option<&'a str>) -> io::Result<Self> {
        let path = path.unwrap_or("-");
        let input = match path {
            "-" => {
                let stdin = io::stdin();
                let stdin = stdin.lock();
                Self::Stdin { path, stdin }
            },
            path => {
                let file = fs::File::open(path)?;
                Self::File { path, file }
            }
        };
        Ok(input)
    }
}

// === impl_hash ===

macro_rules! impl_hash {
    ($( ($name:expr, $ident:ident) ),*,) => {
        impl<'a> Input<'a> {
            fn hash(&mut self, algorithm: &str) -> io::Result<(Vec<u8>, &str)> {
                match algorithm {
                $(
                    $name => {
                        let mut hasher = investigator::$ident::default();
                        match self {
                            Self::Stdin { path, stdin } => {
                                investigator::copy_wide(stdin , &mut hasher)?;
                                let hash = hasher.finish().to_vec();
                                Ok((hash, path))
                            },
                            Self::File { path, file } => {
                                investigator::copy_wide(file, &mut hasher)?;
                                let hash = hasher.finish().to_vec();
                                Ok((hash, path))
                            },
                        }
                    },
                )*
                    _ => panic!("Unknown algorithm: {algorithm}"),
                }
            }
        }
    }
}

impl_hash!(
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
    //("xxh2_64", Xxh2_64)
);



// ============
// === main ===
// ============

fn main() {
    let algorithm = env::args().nth(1).unwrap();
    let path = env::args().nth(2);
    let path = path.as_deref();
    let mut input = Input::new(path).unwrap();
    let (hash, path) = input.hash(&algorithm).unwrap();
    let hash = hex::encode(hash);
    println!("{hash}  {path}");
}
