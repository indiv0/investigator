use investigator::Hasher as TRAIT_Hasher;
use std::env;
use std::fs;
use std::io;



// =======================
// === select_algoritm ===
// =======================

macro_rules! select_algorithm {
    ($algorithm:expr, $( ($name:expr, $ident:ident) ),*) => {
        match $algorithm {
        $(
            $name => |reader| investigator::$ident::from_reader(reader).map(|hash| hash.to_vec()),
        )*
            _ => panic!("Unknown algorithm: {}", $algorithm),
        }
    }
}



// ============
// === main ===
// ============

fn main() {
    let algorithm = env::args().nth(1).unwrap();
    let path = env::args().nth(2).unwrap();
    let algorithm = select_algorithm!(
        algorithm.as_str(),
        ("adler32", Adler32),
        ("adler32rolling", Adler32Rolling),
        ("belthash", BeltHash),
        ("blake2b", Blake2b),
        ("blake2b_simd", Blake2bSimd),
        ("blake2s", Blake2s),
        ("blake2s_simd", Blake2sSimd),
        ("blake3", Blake3),
        ("crc32fast", Crc32Fast),
        ("farm_hash", FarmHash),
        ("fnv", Fnv),
        ("fsb256", Fsb256),
        ("fsb512", Fsb512),
        ("fxhasher", FxHasher),
        ("fxhasher32", FxHasher32),
        ("fxhasher64", FxHasher64),
        ("fxhasher_rustc", FxHasherRustc),
        ("groestl256", Groestl256),
        ("groestl512", Groestl512),
        ("md5", Md5),
        ("metrohash64", MetroHash64),
        ("metrohash128", MetroHash128),
        ("ripemd160", Ripemd160),
        ("seahash", Seahash),
        ("sha256", Sha256),
        ("sha512", Sha512),
        ("sha3_256", Sha3_256),
        ("sha3_512", Sha3_512),
        ("shabal512", Shabal512),
        ("siphash", Siphash),
        ("sm3", Sm3),
        ("t1ha", T1ha),
        ("t1ha2", T1ha2),
        ("tiger", Tiger),
        ("tiger2", Tiger2),
        ("whirlpool", Whirlpool),
        ("xxh3", Xxh3),
        ("xxh64", Xxh64),
        ("xxh64_twohash", Xxh64TwoHash),
        ("xxh2_32", Xxh2_32),
        ("xxh2_64", Xxh2_64)
    );
    //let algorithm = match algorithm.as_str() {
    //    "fx_hasher" => |reader| investigator::FxHasher::from_reader(reader).map(|hash| hash.to_vec()),
    //    "sha256" => |reader| investigator::Sha256::from_reader(reader).map(|hash| hash.to_vec()),
    //    "blake3" => |reader| investigator::Blake3::from_reader(reader).map(|hash| hash.to_vec()),
    //    _ => panic!("Unknown algorithm: {}", algorithm),
    //};
    let file = fs::File::open(&path).unwrap();
    let mut reader = io::BufReader::new(file);
    let hash = algorithm(&mut reader).unwrap();
    let hash = hex::encode(hash);
    println!("{hash}  {path}");
}
