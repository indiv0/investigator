#!/usr/bin/env bash
set -e
cargo build --release
FILE=${1:-benches/random_data}
echo -n "openssl/sha256   " && time openssl                     sha256           $FILE | cut -d' ' -f2
echo -n "b3sum/blake3     " && time ~/.cargo/bin/b3sum                           $FILE
echo -n "adler32          " && time target/release/investigator adler32          $FILE
echo -n "adler32rolling   " && time target/release/investigator adler32rolling   $FILE
echo -n "belthash         " && time target/release/investigator belthash         $FILE
echo -n "blake2b          " && time target/release/investigator blake2b          $FILE
echo -n "blake2b_simd     " && time target/release/investigator blake2b_simd     $FILE
echo -n "blake2s          " && time target/release/investigator blake2s          $FILE
echo -n "blake2s_simd     " && time target/release/investigator blake2s_simd     $FILE
echo -n "blake3           " && time target/release/investigator blake3           $FILE
echo -n "crc32fast        " && time target/release/investigator crc32fast        $FILE
echo -n "farm_hash        " && time target/release/investigator farm_hash        $FILE
echo -n "fnv              " && time target/release/investigator fnv              $FILE
echo -n "fsb256           " && time target/release/investigator fsb256           $FILE
echo -n "fsb512           " && time target/release/investigator fsb512           $FILE
echo -n "fxhasher         " && time target/release/investigator fxhasher         $FILE
echo -n "fxhasher32       " && time target/release/investigator fxhasher32       $FILE
echo -n "fxhasher64       " && time target/release/investigator fxhasher64       $FILE
echo -n "fxhasher_rustc   " && time target/release/investigator fxhasher_rustc   $FILE
echo -n "groestl256       " && time target/release/investigator groestl256       $FILE
echo -n "groestl512       " && time target/release/investigator groestl512       $FILE
echo -n "md5              " && time target/release/investigator md5              $FILE
echo -n "metrohash128     " && time target/release/investigator metrohash128     $FILE
echo -n "metrohash64      " && time target/release/investigator metrohash64      $FILE
echo -n "ripemd160        " && time target/release/investigator ripemd160        $FILE
echo -n "seahash          " && time target/release/investigator seahash          $FILE
echo -n "sha256           " && time target/release/investigator sha256           $FILE
echo -n "sha512           " && time target/release/investigator sha256           $FILE
echo -n "sha3_256         " && time target/release/investigator sha3_256         $FILE
echo -n "sha3_512         " && time target/release/investigator sha3_512         $FILE
echo -n "shabal512        " && time target/release/investigator shabal512        $FILE
echo -n "siphash          " && time target/release/investigator siphash          $FILE
echo -n "sm3              " && time target/release/investigator sm3              $FILE
echo -n "t1ha             " && time target/release/investigator t1ha             $FILE
echo -n "t1ha2            " && time target/release/investigator t1ha2            $FILE
echo -n "tiger            " && time target/release/investigator tiger            $FILE
echo -n "tiger2           " && time target/release/investigator tiger2           $FILE
echo -n "whirlpool        " && time target/release/investigator whirlpool        $FILE
echo -n "xxh3             " && time target/release/investigator xxh3             $FILE
echo -n "xxh64            " && time target/release/investigator xxh64            $FILE
echo -n "xxh64_twohash    " && time target/release/investigator xxh64_twohash    $FILE
echo -n "xxh2_32          " && time target/release/investigator xxh2_32          $FILE
echo -n "xxh2_64          " && time target/release/investigator xxh2_64          $FILE
