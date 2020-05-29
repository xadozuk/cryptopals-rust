use crate::attacks;
use crate::lib;

use lib::byte::ByteVec;

pub fn challenge3()
{
    let result = attacks::xor::single_byte::decrypt(
        &ByteVec::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
    );

    println!("Guess : (Key: 0x{:0>2x}) {}", result.0, result.1.to_string().unwrap());
}