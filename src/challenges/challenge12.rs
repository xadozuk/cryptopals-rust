use crate::lib::types::{Byte, ByteVec};
use crate::lib::traits::{Random, FromBase64, ToHex, BlockIterable, ToString};
use crate::lib::crypto::aes;
use crate::attacks;

use std::collections::HashMap;
use std::time::{Duration, Instant};
use rand::Rng;

const UNKNOWN_STRING: &str = 
"Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg
aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq
dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg
YnkK";

const DEBUG: bool = false;

macro_rules! debugln
{
    () => {
        if DEBUG { std::println!() }
    };
    ($($arg:tt)*) => {
        if DEBUG
        {
            std::print!("[DEBUG] ");
            std::println!($($arg)*);
        }
    }
}

fn encryption_oracle(input: &[Byte], key: &[Byte]) -> ByteVec
{
    let mut data = input.to_vec();
    data.extend(ByteVec::from_base64(UNKNOWN_STRING));

    let msg = aes::Message::from(data, aes::Key::new(key));

    aes::encrypt(aes::AesMode::ECB, &msg)
}

fn detect_block_size() -> usize
{
    let tmp_key = ByteVec::random(16);
    let mut output_size = encryption_oracle(&vec![], &tmp_key).len();

    debugln!("Output size with zero-input : {}", output_size);

    let mut n = 0;
    let mut sizes = vec![];

    loop
    {
        n += 1;

        let current_size = encryption_oracle(&vec![0x0; n], &tmp_key).len();

        // If adding a byte change output size, we have found a block boundary
        if current_size > output_size
        {   
            debugln!("Found a block boundary when adding {} bytes", n);

            sizes.push(n);
            output_size = current_size;
        }   

        if sizes.len() >= 2 { break }
    }

    let block_size = sizes[1] - sizes[0];
    println!("Block size : {}", block_size);

    block_size
}

fn build_dictionnary(n_block: usize, block_size: usize, input: &ByteVec, key: &[Byte]) -> HashMap<ByteVec, Byte>
{
    println!("### Building dictionnary");

    let mut rainbow = HashMap::<ByteVec, Byte>::new();

    for b in 0x00..=0xFF
    {
        let crafted = [&input[..], &[b]].concat();
        let cipher = encryption_oracle(&crafted, &key).blocks(block_size).nth(n_block).unwrap();

        debugln!("oracle(input || 0x{:0>2x})[{}] = {}...", b, n_block, cipher.to_hex());

        rainbow.insert(cipher, b);
    }

    debugln!();

    rainbow
}

pub fn challenge12()
{
    println!("=== [ Challenge 12 ] ===");

    let key = ByteVec::random(16);

    println!("Consistent key : {}", key.to_hex());    

    println!("\n# Detecting block size");
    let block_size = detect_block_size();

    println!("\n# Detecting AES mode");
    let ciphertext = encryption_oracle(&vec![0x0; block_size * 4], &key);
    let ecb_score = attacks::aes::detect_ecb(&ciphertext, block_size);

    println!("ECB mode : {}", ecb_score > 0.0);

    println!("\n# Finding number of blocks to decrypt");
    let n_blocks_to_decrypt = encryption_oracle(&vec![], &key).blocks(block_size).count();

    println!("Blocks to decrypt : {}", n_blocks_to_decrypt);

    println!("\n# Decrypting one byte at a time");

    let mut plaintext: ByteVec = vec![];

    for block in 0..n_blocks_to_decrypt
    {
        println!("## Decrypting block {}", block);

        for i in 0..block_size
        {
            let start = Instant::now();

            let pre_block     = vec![0x0; block_size - 1 - i];
            let crafted_input = [&pre_block, &plaintext[..]].concat();
            let needle        = encryption_oracle(&pre_block, &key).blocks(block_size).nth(block).unwrap();

            println!("Crafted input : {}", crafted_input.to_hex());
            println!("Needle        : {}", needle.to_hex());

            let rainbow = build_dictionnary(block, block_size, &crafted_input, &key);

            let decrypted_byte = rainbow.get(&needle);

            // Last block, we are hitting the padding
            if decrypted_byte == None && block == n_blocks_to_decrypt - 1
            {
                debugln!("Hitting padding, stopping !");
                break;
            }

            plaintext.push(*decrypted_byte.unwrap());

            println!("[{}][{}] = 0x{:0>2x}", block, i, *decrypted_byte.unwrap());
            println!("Decrypted byte in {:?}", start.elapsed());
        }
    }

    println!("\n# Decrypting done\nPlaintext :\n{}", plaintext.to_string().unwrap_or(String::from("Non UTF8")));
}
