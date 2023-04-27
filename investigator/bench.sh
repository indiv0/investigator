#!/usr/bin/env bash
INVESTIGATOR=../target/release/investigator
set -e
cargo build --release
FILE=${1:-benches/random_data}
echo -n "openssl/sha256   " && time openssl         sha256           $FILE | cut -d' ' -f2
echo -n "b3sum/blake3     " && time ~/.cargo/bin/b3sum               $FILE
echo -n "adler32          " && time ${INVESTIGATOR} adler32          $FILE
echo -n "adler32_rolling  " && time ${INVESTIGATOR} adler32_rolling  $FILE
echo -n "belt_hash        " && time ${INVESTIGATOR} belt_hash        $FILE
echo -n "blake2b          " && time ${INVESTIGATOR} blake2b          $FILE
echo -n "blake2b_simd     " && time ${INVESTIGATOR} blake2b_simd     $FILE
echo -n "blake2s          " && time ${INVESTIGATOR} blake2s          $FILE
echo -n "blake2s_simd     " && time ${INVESTIGATOR} blake2s_simd     $FILE
echo -n "blake3           " && time ${INVESTIGATOR} blake3           $FILE
echo -n "crc32_fast       " && time ${INVESTIGATOR} crc32_fast       $FILE
echo -n "farm_hash        " && time ${INVESTIGATOR} farm_hash        $FILE
echo -n "fnv              " && time ${INVESTIGATOR} fnv              $FILE
echo -n "fsb256           " && time ${INVESTIGATOR} fsb256           $FILE
echo -n "fsb512           " && time ${INVESTIGATOR} fsb512           $FILE
echo -n "fx_hasher        " && time ${INVESTIGATOR} fx_hasher        $FILE
echo -n "fx_hasher32      " && time ${INVESTIGATOR} fx_hasher32      $FILE
echo -n "fx_hasher64      " && time ${INVESTIGATOR} fx_hasher64      $FILE
echo -n "fx_hasher_rustc  " && time ${INVESTIGATOR} fx_hasher_rustc  $FILE
echo -n "groestl256       " && time ${INVESTIGATOR} groestl256       $FILE
echo -n "groestl512       " && time ${INVESTIGATOR} groestl512       $FILE
echo -n "md5              " && time ${INVESTIGATOR} md5              $FILE
echo -n "metro_hash128    " && time ${INVESTIGATOR} metro_hash128    $FILE
echo -n "metro_hash64     " && time ${INVESTIGATOR} metro_hash64     $FILE
echo -n "ripemd160        " && time ${INVESTIGATOR} ripemd160        $FILE
echo -n "seahash          " && time ${INVESTIGATOR} seahash          $FILE
echo -n "sha256           " && time ${INVESTIGATOR} sha256           $FILE
echo -n "sha512           " && time ${INVESTIGATOR} sha256           $FILE
echo -n "sha3_256         " && time ${INVESTIGATOR} sha3_256         $FILE
echo -n "sha3_512         " && time ${INVESTIGATOR} sha3_512         $FILE
echo -n "shabal512        " && time ${INVESTIGATOR} shabal512        $FILE
echo -n "siphash          " && time ${INVESTIGATOR} siphash          $FILE
echo -n "sm3              " && time ${INVESTIGATOR} sm3              $FILE
echo -n "t1ha             " && time ${INVESTIGATOR} t1ha             $FILE
echo -n "t1ha2            " && time ${INVESTIGATOR} t1ha2            $FILE
echo -n "tiger            " && time ${INVESTIGATOR} tiger            $FILE
echo -n "tiger2           " && time ${INVESTIGATOR} tiger2           $FILE
echo -n "whirlpool        " && time ${INVESTIGATOR} whirlpool        $FILE
echo -n "xxh3             " && time ${INVESTIGATOR} xxh3             $FILE
echo -n "xxh64            " && time ${INVESTIGATOR} xxh64            $FILE
echo -n "xxh64_two_hash   " && time ${INVESTIGATOR} xxh64_two_hash   $FILE
echo -n "xxh2_32          " && time ${INVESTIGATOR} xxh2_32          $FILE
echo -n "xxh2_64          " && time ${INVESTIGATOR} xxh2_64          $FILE
