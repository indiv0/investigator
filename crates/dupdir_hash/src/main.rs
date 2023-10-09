use core::str;
use core::str::FromStr as _;
use dupdir_hash::Hasher as _;
use std::env;
use std::fs;
use std::io;

// =============
// === Input ===
// =============

enum Input<'a> {
    Stdin { path: &'a str, stdin: io::Stdin },
    File { path: &'a str, file: fs::File },
}

// === Main `impl` ===

impl<'a> Input<'a> {
    fn new(path: Option<&'a str>) -> io::Result<Self> {
        let path = path.unwrap_or("-");
        let input = match path {
            "-" => {
                let stdin = io::stdin();
                Self::Stdin { path, stdin }
            }
            path => {
                let file = fs::File::open(path)?;
                Self::File { path, file }
            }
        };
        Ok(input)
    }

    fn hash(&mut self, algorithm: Algorithm) -> io::Result<(Vec<u8>, &str)> {
        match self {
            Self::Stdin { path, stdin } => {
                let mut stdin = stdin.lock();
                let hash = algorithm.hash(&mut stdin)?;
                Ok((hash, path))
            }
            Self::File { path, file } => {
                let hash = algorithm.hash(file)?;
                Ok((hash, path))
            }
        }
    }
}

// =======================
// === impl_algorithms ===
// =======================

macro_rules! impl_algorithms {
    ($( $ident:ident ),*,) => {
        paste::paste! {
            // =================
            // === Algorithm ===
            // =================

            #[derive(Clone, Copy, Debug)]
            #[must_use]
            enum Algorithm {
            $(
                #[cfg(feature = "hash-" $ident)]
                $ident,
            )*
            }


            // === Internal `impl`s ===

            impl Algorithm {
                fn hash(&self, reader: &mut impl io::Read) -> io::Result<Vec<u8>> {
                    match self {
                    $(
                        #[cfg(feature = "hash-" $ident)]
                        Self::$ident => {
                            let mut hasher = dupdir_hash::$ident::default();
                            dupdir_hash::copy_wide(reader, &mut hasher)?;
                            let hash = hasher.finish();
                            let hash = hash.to_vec();
                            Ok(hash)
                        },
                    )*
                    }
                }
            }


            // === Trait `impl`s ===

            impl str::FromStr for Algorithm {
                type Err = String;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                    $(
                        #[cfg(feature = "hash-" $ident)]
                        stringify!([<$ident:snake:lower>]) => Ok(Self::$ident),
                    )*
                        _ => Err(format!("Unknown algorithm: \"{s}\".")),
                    }
                }
            }
        }
    }
}

impl_algorithms!(
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

// ============
// === main ===
// ============

fn main() {
    let algorithm = env::args().nth(1).unwrap();
    let algorithm = Algorithm::from_str(&algorithm).unwrap();
    let path = env::args().nth(2);
    let path = path.as_deref();
    let mut input = Input::new(path).unwrap();
    let (hash, path) = input.hash(algorithm).unwrap();
    let hash = hex::encode(hash);
    println!("{hash}  {path}");
}
