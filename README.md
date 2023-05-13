# investigator

## Quickstart

```sh
# Install dependencies necessary to run development environment.
make pre-dependencies
# Update developer environment to latest version.
make update-dev-env
# Enable developer environment.
make dev-env
cd ..
# Install cargo-watch to automatically rebuild during development.
make post-dependencies
# Build, test, and run benchmarks
make run
```

## DupDir Usage

Usage:
```sh
mkdir -p target/data
clear && cargo check && RUST_BACKTRACE=1 time cargo run --release old_dup_dirs
cat target/data/dupdirs_by_path.txt | awk '{ print length, $0 }' | sort -n -s -r | cut -d" " -f2- > tmp.txt
scp tmp.txt 172.30.194.6:
ssh 172.30.194.6
sudo mv tmp.txt /storage/tmp.txt
sudo su
cd /storage
cat tmp.txt | grep -v "'" | grep -v ' \./lap-ca-nik-01\| \./lab-ca-kvm-02' | cut -d' ' -f2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27 | xargs -I{} du -d 0 "{}" | sort -n
```

Other usage:
```sh
mkdir -p target/data
clear && cargo check && RUST_BACKTRACE=1 time cargo run --release old_dup_dirs
cat target/data/dupdirs_by_path.txt | cut -d' ' -f2- | xargs -d '\n' du -d0 | sort -n
```

New usage:
```sh
clear && cargo run --package utils
find /Users/indiv0/Desktop/files -type f -name '*'$'\r''*'
find /Users/indiv0/Desktop/files -type f -name '*'$'\r''*' -delete
find /Users/indiv0/Desktop/files -not -perm -u=r -not -perm -u=w -not -perm -u=x -ls
find /Users/indiv0/Desktop/files -not -perm -u=r -not -perm -u=w -not -perm -u=x -delete
mkdir -p target/data
sudo su
time ./target/release/dupdir find /Users/indiv0/Desktop/files > target/data/files.txt && chown indiv0 target/data/files.txt
time ./target/release/dupdir hash target/data/files.txt > target/data/hashes.txt && chown indiv0 target/data/hashes.txt
time ./target/release/dupdir dir_files target/data/files.txt > target/data/dir_files.txt && chown indiv0 target/data/dir_files.txt
time ./target/release/dupdir dir_hashes target/data/dir_files.txt target/data/hashes.txt > target/data/dir_hashes.txt && chown indiv0 target/data/dir_hashes.txt
time ./target/release/dupdir dup_dirs target/data/dir_hashes.txt > target/data/dup_dirs.txt && chown indiv0 target/data/dup_dirs.txt
exit
cat target/data/dup_dirs.txt | cut -d';' -f2 | xargs -d '\n' du -d0 | sort -n
```

# Find Files Usages

```shell
cargo install cargo-watch
cargo install dioxus-cli
~/.cargo/bin/dioxus serve
~/.cargo/bin/dioxus build --release
rm -f find-files.db
~/.cargo/bin/cargo-watch -s "cargo test --package find-files && cargo run --release --package find-files ~/Desktop/files"
> find_by_ext tif,tiff,bmp,jpg,jpeg,gif,png
```

## Links

- [Dioxus - Custom Assets](https://github.com/DioxusLabs/dioxus/blob/c113d96bbe0a952f51652f019f5c313ac5c0257b/examples/custom_assets.rs)
- [Reddit - Published a Dioxus+TailwindCSS Example](https://old.reddit.com/r/rust/comments/1224elh/published_a_dioxustailwindcss_example_with_up_to/)
- [Dioxus - Managing State](https://github.com/DioxusLabs/dioxus/blob/35cb6616af3dd85d2370583d2a2e8d575df23d73/docs/guide/src/en/__unused/index.md)
- [Dioxus - Tests For Hooks](https://github.com/DioxusLabs/dioxus/issues/955#issuecomment-1531639013)
- [Dioxus - File Explorer Example](https://github.com/DioxusLabs/example-projects/blob/9a59be6f6506a15f868e64b95512dbbd479c9c0c/file-explorer/src/main.rs)

## Investigator Performance

### Benchmark by File Size (M2; Slowest Highlighted)

```shell
$ cd investigator
$ head -c 10000000 /dev/urandom > benches/random_data
$ rm bench.txt && ./bench.sh benches/random_data 2>&1 | tee -a bench.txt
$ head -c 1000000000 /dev/urandom > /tmp/random_data
$ rm bench.txt && ./bench.sh /tmp/random_data 2>&1 | tee -a bench.txt
$ head -c 10000000000 /dev/urandom > /tmp/random_data
$ rm bench.txt && ./bench.sh /tmp/random_data 2>&1 | tee -a bench.txt
```

| Algorithm | Real Time (10 MB) | Real Time (1000 MB) | Real Time (10000 MB) |
| - | - | - | - |
| openssl/sha256   | 0m0.007s     | 0m0.577s     | 0m6.673s  |
| b3sum/blake3     | 0m0.003s     | 0m0.142s     | 0m6.321s  |
| adler32          | **0m0.382s** |              |           |
| adler32rolling   | 0m0.006s     | 0m0.588s     | 0m5.228s  |
| belthash         | **0m0.102s** |              |           |
| blake2b          | 0m0.013s     | 0m1.031s     | 0m11.251s |
| blake2b_simd     | 0m0.012s     | 0m0.979s     | 0m10.696s |
| blake2s          | 0m0.018s     | **0m1.617s** |           |
| blake2s_simd     | 0m0.018s     | **0m1.559s** |           |
| blake3           | 0m0.009s     | 0m0.717s     | 0m8.068s  |
| crc32fast        | 0m0.048s     | **0m4.568s** |           |
| farm_hash        | 0m0.009s     | 0m0.898s     | 0m10.398s |
| fnv              | 0m0.015s     | **0m1.298s** |           |
| fsb256           | **0m0.692s** |              |           |
| fsb512           | **0m1.430s** |              |           |
| fxhasher         | 0m0.017s     | **0m1.584s** |           |
| fxhasher32       | 0m0.017s     | **0m1.547s** |           |
| fxhasher64       | 0m0.017s     | **0m1.538s** |           |
| fxhasher_rustc   | 0m0.017s     | **0m1.542s** |           |
| groestl256       | 0m0.044s     | **0m4.236s** |           |
| groestl512       | 0m0.059s     | **0m5.749s** |           |
| md5              | 0m0.019s     | **0m1.711s** |           |
| metrohash128     | 0m0.008s     | 0m0.647s     | 0m6.900s  |
| metrohash64      | 0m0.008s     | 0m0.649s     | 0m6.450s  |
| ripemd160        | 0m0.036s     | **0m3.375s** |           |
| seahash          | 0m0.037s     | **0m3.555s** |           |
| sha256           | 0m0.007s     | 0m0.543s     | 0m5.439s  |
| sha512           | 0m0.007s     | 0m0.541s     | 0m5.427s  |
| sha3_256         | 0m0.018s     | **0m1.655s** |           |
| sha3_512         | 0m0.032s     | **0m3.006s** |           |
| shabal512        | 0m0.019s     | **0m1.759s** |           |
| siphash          | 0m0.010s     | 0m0.921s     | 0m8.969s  |
| sm3              | 0m0.030s     | **0m2.923s** |           |
| t1ha             | 0m0.032s     | **0m3.078s** |           |
| t1ha2            | 0m0.003s     | 0m0.150s     | 0m1.455s  |
| tiger            | 0m0.016s     | **0m1.492s** |           |
| tiger2           | 0m0.016s     | **0m1.491s** |           |
| whirlpool        | 0m0.030s     | **0m2.881s** |           |
| xxh3             | 0m0.022s     | **0m2.123s** |           |
| xxh64            | 0m0.032s     | **0m3.040s** |           |
| xxh64_twohash    | 0m0.038s     | **0m3.656s** |           |
| xxh2_32          | 0m0.039s     | **0m3.806s** |           |
| xxh2_64          | 0m0.038s     | **0m3.647s** |           |

### Benchmark by File Size (Xeon; Slowest Highlighted)

```shell
$ cd investigator
$ head -c 10000000 /dev/urandom > benches/random_data
$ rm bench.txt && ./bench.sh /tmp/random_data 2>&1 | tee -a bench.txt
$ head -c 1000000000 /dev/urandom > /tmp/random_data
$ rm bench.txt && ./bench.sh /tmp/random_data 2>&1 | tee -a bench.txt
$ head -c 10000000000 /dev/urandom > /tmp/random_data
$ rm bench.txt && ./bench.sh /tmp/random_data 2>&1 | tee -a bench.txt
```

| Algorithm | Real Time (10 MB) | Real Time (1000 MB) | Real Time (10000 MB) |
| - | - | - | - |
| openssl/sha256   | 0m0.007s     | 0m0.507s     | 0m4.969s  |
| b3sum/blake3     | 0m0.003s     | 0m0.043s     | 0m0.393s  |
| adler32          | 0m0.086s     | **0m8.661s** |           |
| adler32rolling   | 0m0.004s     | 0m0.399s     | 0m3.968s  |
| belthash         | 0m0.081s     | **0m7.752s** |           |
| blake2b          | 0m0.008s     | 0m0.737s     | 0m7.542s  |
| blake2b_simd     | 0m0.008s     | 0m0.724s     | 0m7.471s  |
| blake2s          | 0m0.023s     | **0m2.249s** |           |
| blake2s_simd     | 0m0.013s     | **0m1.278s** |           |
| blake3           | 0m0.003s     | 0m0.272s     | 0m2.590s  |
| crc32fast        | 0m0.035s     | **0m3.586s** |           |
| farm_hash        | 0m0.013s     | **0m1.288s** |           |
| fnv              | 0m0.010s     | 0m0.927s     | 0m9.414s  |
| fsb256           | **0m1.240s** |              |           |
| fsb512           | **0m2.470s** |              |           |
| fxhasher         | 0m0.012s     | **0m1.167s** |           |
| fxhasher32       | 0m0.011s     | **0m1.164s** |           |
| fxhasher64       | 0m0.011s     | **0m1.164s** |           |
| fxhasher_rustc   | 0m0.011s     | **0m1.158s** |           |
| groestl256       | 0m0.046s     | **0m4.702s** |           |
| groestl512       | 0m0.067s     | **0m6.858s** |           |
| md5              | 0m0.013s     | **0m1.299s** |           |
| metrohash128     | 0m0.015s     | **0m1.568s** |           |
| metrohash64      | 0m0.015s     | **0m1.561s** |           |
| ripemd160        | 0m0.026s     | **0m2.597s** |           |
| seahash          | 0m0.078s     | **0m7.550s** |           |
| sha256           | 0m0.005s     | 0m0.514s     | 0m5.064s  |
| sha512           | 0m0.005s     | 0m0.529s     | 0m4.834s  |
| sha3_256         | 0m0.021s     | **0m2.123s** |           |
| sha3_512         | 0m0.039s     | **0m3.963s** |           |
| shabal512        | 0m0.011s     | **0m1.054s** |           |
| siphash          | 0m0.008s     | 0m0.847s     | 0m8.084s  |
| sm3              | 0m0.023s     | **0m2.376s** |           |
| t1ha             | 0m0.091s     | **0m9.443s** |           |
| t1ha2            | 0m0.001s     | 0m0.146s     | 0m1.297s  |
| tiger            | 0m0.012s     | **0m1.270s** |           |
| tiger2           | 0m0.012s     | **0m1.271s** |           |
| whirlpool        | 0m0.048s     | **0m4.951s** |           |
| xxh3             | 0m0.056s     | **0m5.721s** |           |
| xxh64            | 0m0.040s     | **0m3.961s** |           |
| xxh64_twohash    | 0m0.041s     | **0m4.224s** |           |
| xxh2_32          | 0m0.051s     | **0m5.233s** |           |
| xxh2_64          | 0m0.049s     | **0m4.984s** |           |

## Benchmark by Speed (Sum of M2 + Xeon Real Time; Sorted by Speed)

| Algorithm | Real Time (10000 MB; Xeon) | Real Time (10000 MB; M2) | Real Time (10000 MB; Sum) |
| - | - | - | - |
| t1ha2            | 0m1.297s | 0m1.455s  | 0m2.752s  |
| b3sum/blake3     | 0m0.393s | 0m6.321s  | 0m6.714s  |
| adler32rolling   | 0m3.968s | 0m5.228s  | 0m9.196s  |
| sha512           | 0m4.834s | 0m5.427s  | 0m10.261s |
| blake3           | 0m2.590s | 0m8.068s  | 0m10.658s |
| sha256           | 0m5.064s | 0m5.439s  | 0m10.503s |
| openssl/sha256   | 0m4.969s | 0m6.673s  | 0m11.462s |
| siphash          | 0m8.084s | 0m8.969s  | 0m17.053s |
| blake2b          | 0m7.542s | 0m11.251s | 0m18.793s |
| blake2b_simd     | 0m7.471s | 0m10.696s | 0m18.167s |
