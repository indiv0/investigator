# investigator

## Quickstart

```
# Install XCode Command Line Developer Tools.
xcode-select --install
# Install Rust & Cargo via Nix.
nix-shell --packages rustup libiconv cmake
rustup update nightly
# Install cargo-watch to automatically rebuild during development.
cargo install cargo-watch
# Run benchmarks
cargo bench
```

# Performance

## Benchmark by File Size (Slowest Highlighted)

```shell
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
| blake2b          | 0m0.013s     | **0m1.031s** |           |
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
