use std::fs;
use std::io::Write as _;
use std::path;



// =================
// === Constants ===
// =================

const HELLO_WORLD_PATH: &str = "benches/hello_world";
const HELLO_WORLD_DATA: &[u8] = b"Hello, world!";



// =========================================
// === create_file_with_hello_world_data ===
// =========================================

fn create_file_with_hello_world_data() -> path::PathBuf {
    let path = path::Path::new(HELLO_WORLD_PATH);

    if !path.exists() {
        let mut file = fs::File::create(path).unwrap();
        file.write_all(HELLO_WORLD_DATA).unwrap();
    }

    path.to_path_buf()
}



// =================
// === impl_test ===
// =================

macro_rules! impl_test {
    ($ty:ident $(<'_>)?, $expected:expr) => {
        paste::paste! {
            #[cfg(feature = "hash-" $ty)]
            mod [<$ty:snake:lower>] {
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
        }
    };
}

impl_test!(Adler32, "00000000205e048a"); // FIXME [NP]: too long
impl_test!(Adler32Rolling, "205e048a");
impl_test!(BeltHash, "249dc153df2f7bf8f9b0e5400c1c8deff429b2ff013247a98f8a2cbcec995ade");
impl_test!(Blake2b, "a2764d133a16816b5847a737a786f2ece4c148095c5faa73e24b4cc5d666c3e45ec271504e14dc6127ddfce4e144fb23b91a6f7b04b53d695502290722953b0f");
impl_test!(Blake2bSimd, "a2764d133a16816b5847a737a786f2ece4c148095c5faa73e24b4cc5d666c3e45ec271504e14dc6127ddfce4e144fb23b91a6f7b04b53d695502290722953b0f");
impl_test!(Blake2s, "30d8777f0e178582ec8cd2fcdc18af57c828ee2f89e978df52c8e7af078bd5cf");
impl_test!(Blake2sSimd, "30d8777f0e178582ec8cd2fcdc18af57c828ee2f89e978df52c8e7af078bd5cf");
impl_test!(Blake3, "ede5c0b10f2ec4979c69b52f61e42ff5b413519ce09be0f14d098dcfe5f6f98d");
impl_test!(Crc32Fast, "00000000ebe6c6e6"); // FIXME [NP]: too long
impl_test!(FarmHash, "307c26b3e0789a47");
impl_test!(Fnv, "38d1334144987bf4");
impl_test!(Fsb256, "b75c250c35ccebeb67d6e9a5173e638a0ebc2545674c2da17fc0275a62b3f69c");
impl_test!(Fsb512, "75186f19cd5b7c57d4be1247d7f39bdc681ec796cebb5668ea2eb4eb233294071ca915e56887549464dc7d3e077f08492e6ed0d382943efbeab20e191a5f09d0");
impl_test!(FxHasher, "562dc0284e81dff2");
impl_test!(FxHasher32, "00000000c5b0ab5f"); // FIXME [NP]: too long
impl_test!(FxHasher64, "562dc0284e81dff2");
impl_test!(FxHasherRustc, "562dc0284e81dff2");
impl_test!(Groestl256, "63e4ab2044e38c1fb1725313f2229e038926af839c86eaf96553027d2c851e18");
impl_test!(Groestl512, "b60658e723a8eb1743823a8002175486bc24223ba3dc6d8cb435a948f6d2b9744ac9e307e1d38021ea18c4d536d28fc23491d7771a5a5b0d02ffad9a073dcc28");
impl_test!(KangarooTwelve256<'_>, "2a7eccaa09ff7e30cb1413bda28dad7f90759f22fc63535369bf17595b1166af");
impl_test!(KangarooTwelve512<'_>, "2a7eccaa09ff7e30cb1413bda28dad7f90759f22fc63535369bf17595b1166af5d6edd1b483c5eee16d5291ac37c454ff1f26d8ce176a7c73a79232e5b2e402f");
impl_test!(Md5, "6cd3556deb0da54bca060b4c39479839");
impl_test!(MetroHash128, "5930f69e4971f2c0");
impl_test!(MetroHash64, "fc8b20d0f74c7aa7");
impl_test!(Ripemd160, "58262d1fbdbe4530d8865d3518c6d6e41002610f");
impl_test!(Seahash, "0682402aaca36178");
impl_test!(Sha256, "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3");
impl_test!(Sha3_256, "f345a219da005ebe9c1a1eaad97bbf38a10c8473e41d0af7fb617caa0c6aa722");
impl_test!(Sha3_512, "8e47f1185ffd014d238fabd02a1a32defe698cbf38c037a90e3c0a0a32370fb52cbd641250508502295fcabcbf676c09470b27443868c8e5f70e26dc337288af");
impl_test!(Sha512, "c1527cd893c124773d811911970c8fe6e857d6df5dc9226bd8a160614c0cd963a4ddea2b94bb7d36021ef9d865d5cea294a82dd49a0bb269f51f6e7a57f79421");
impl_test!(Shabal512, "7048f0a589339d2d26890701ed3b2d1ed7c8dd1ac37fec517c7a8c39d5d51548e96ea8dfaceb5b99f9d1db3b18a7652e0412348ebfd61d32d755d6098bff8cb3");
impl_test!(Siphash, "ae5020d7cf49d14f");
impl_test!(Sm3, "e3bca101b496880c3653dad85861d0e784b00a8c18f7574472d156060e9096bf");
impl_test!(T1ha, "936f6a215ae53484");
impl_test!(T1ha2, "345623cc534aa878");
impl_test!(Tiger, "b5e5dd73a5894236937084131bb845189cdc5477579b9f36");
impl_test!(Tiger2, "5d76a0e497c8cb50616ce102d7c0d9d4c5e6260b1e8bac4e");
impl_test!(Whirlpool, "a1a8703be5312b139b42eb331aa800ccaca0c34d58c6988e44f45489cfb16beb4b6bf0ce20be1db22a10b0e4bb680480a3d2429e6c483085453c098b65852495");
impl_test!(Xxh3, "f3c34bf11915e869");
impl_test!(Xxh64, "f58336a78b6f9476");
impl_test!(Xxh64TwoHash, "f58336a78b6f9476");
impl_test!(Xxh2_32, "31b7405d");
impl_test!(Xxh2_64, "f58336a78b6f9476");

