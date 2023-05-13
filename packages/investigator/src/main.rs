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
        stdin: io::Stdin,
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
    ($( $ident:ident ),*,) => {
        paste::paste! {
            impl<'a> Input<'a> {
                fn hash(&mut self, algorithm: &str) -> io::Result<(Vec<u8>, &str)> {
                    match algorithm {
                    $(
                        #[cfg(feature = "hash-" $ident)]
                        stringify!([<$ident:snake:lower>]) => {
                            let mut hasher = investigator::$ident::default();
                            match self {
                                Self::Stdin { path, stdin } => {
                                    let mut stdin = stdin.lock();
                                    investigator::copy_wide(&mut stdin , &mut hasher)?;
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
}

impl_hash!(
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
    let path = env::args().nth(2);
    let path = path.as_deref();
    let mut input = Input::new(path).unwrap();
    let (hash, path) = input.hash(&algorithm).unwrap();
    let hash = hex::encode(hash);
    println!("{hash}  {path}");
}
