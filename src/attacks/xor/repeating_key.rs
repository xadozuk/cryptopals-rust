use std::collections::HashMap;
use std::cmp::min;

use crate::lib::num;
use crate::lib::byte::ByteVec;
use crate::lib::crypto::xor::repeating_key;
use crate::attacks::xor::single_byte::frequency_score;

const MAX_KEY_SIZE: usize = 40;
const KEY_BLOCKS: usize = 4;
const TOP_KEY_SIZES: usize = 10;
const DEFAULT_KEY_SIZE_COUNT: usize = 5;

fn hamming_distance(a: &ByteVec, b: &ByteVec) -> u32
{
    if a.len() != b.len() { panic!("ByteVecs must be the same length. ({}/{})", a.len(), b.len()); }

    let a_bits = a.bits();
    let b_bits = b.bits();

    let mut distance = 0;

    for i in 0..a_bits.len()
    {
        if a_bits[i] != b_bits[i] 
        {
            distance += 1;
        }
    }

    return distance;
}

fn score_distance(ciphertext: &ByteVec) -> Vec<(usize, f64)>
{
    let mut scores: Vec<(usize, f64)> = vec![];

    for key_size in 2..MAX_KEY_SIZE+1
    {
        let mut distances: Vec<u32> = vec![];

        for n_block in 0..KEY_BLOCKS-1
        {
            if (n_block+1)*key_size > ciphertext.len() || (n_block+2)*key_size > ciphertext.len()
            {
                //println!("Block outside ciphertext, skipping.");
                continue;
            }

            let a_block = ciphertext.slice(n_block*key_size..(n_block+1)*key_size);
            let b_block = ciphertext.slice((n_block+1)*key_size..(n_block+2)*key_size);

            let dist = hamming_distance(&a_block, &b_block);

            //println!("Distance [{0}..{1}]-[{1}..{2}] ({3}) = {4}", n_block*key_size, (n_block+1)*key_size, (n_block+2)*key_size, key_size, dist);

            distances.push(dist);
        }

        let total_distance: u32 = distances.iter().sum();        
        let score = total_distance as f64 / (key_size * distances.len()) as f64;

        //println!("=> Normalized distance [{}] = {} (td : {})", key_size, score, total_distance);

        scores.push((key_size, score));
    }   

    scores.sort_by( |a,b| a.1.partial_cmp(&b.1).unwrap() );

    // for kv in &scores
    // {
    //     println!("Size: {} ({})", kv.0, kv.1)
    // }

    return scores;
}

//TODO: refactor this ugly bit
fn guess_key_size(ciphertext: &ByteVec) -> Vec<(usize, f64)>
{
    let scores = score_distance(ciphertext);
    let top_key_sizes = &scores[0..TOP_KEY_SIZES];

    // Find all GCD between keys (to remove multiple of key)
    let mut gcds: HashMap<u32, u32> = HashMap::<u32, u32>::new();

    for i in 0..top_key_sizes.len()
    {
        for j in (i+1)..top_key_sizes.len()
        {
            let gcd = num::gcd(top_key_sizes[i].0 as u32, top_key_sizes[j].0 as u32);
            *gcds.entry(gcd).or_insert(0) += 1;
        }
    }

    // Find the most-frequent GCD
    let mut mfgcd = *gcds.keys().nth(0).unwrap();

    for (gcd, count) in &gcds
    {
        if count > &gcds[&mfgcd]
        {
            mfgcd = *gcd;
        }
    }

    // If the MF-GCD is in the top key_sizes, return it
    if let Some(_) = top_key_sizes.iter().find( |t| t.0 == mfgcd as usize )
    {
        return vec![(mfgcd as usize, 1.0)];
    }
    // Else return the littlest normalized-distance key_size
    else
    {
        // println!("[DEBUG] Default return key_size");
        return scores[0..DEFAULT_KEY_SIZE_COUNT].to_vec();
    }
}

fn slice_block(ciphertext: &ByteVec, block_size: usize) -> Vec<ByteVec>
{
    let mut result: Vec<ByteVec> = vec![];

    for i in (0..ciphertext.len()).step_by(block_size)
    {
        result.push(ciphertext.slice(i..min(i+block_size, ciphertext.len())));
    }

    return result;
}

fn transpose(blocks: &Vec<ByteVec>) -> Vec<ByteVec>
{
    let mut result = vec![];

    for i in 0..blocks[0].len()
    {
        let mut bytes = ByteVec::new();

        for j in 0..blocks.len()
        {
            if let Some(&b) = blocks[j].get(i)
            {
                bytes.push(b);
            }
        }

        result.push(bytes);
    }

    return result;
}

fn guess_key(ciphertext: &ByteVec, key_size: usize) -> ByteVec
{
    // Break cipher_text into block of *key_size*
    let blocks = slice_block(ciphertext, key_size);

    // Break cipher_text into block of *key_size*
    let blocks = slice_block(ciphertext, key_size);

    // Transpose blocks
    let transposed_blocks = transpose(&blocks);

    println!("[DEBUG] Decrypting blocks with a key size of {} ...", key_size);

    // Run single-byte attacks on transposed blocks
    let mut key = ByteVec::new();

    for block in transposed_blocks
    {
        // println!("=== START BLOCK ===");
        let decrypted_block = super::single_byte::decrypt(&block);
        key.push(decrypted_block.0);
    }

    return key;
}

pub fn decrypt(ciphertext: &ByteVec) -> (ByteVec, ByteVec)
{
    // Find key_size by computing hamming distance for blocks (search min distance normalized)
    let key_sizes = guess_key_size(ciphertext);

    if key_sizes.len() == 1
    {
        let key_size = key_sizes[0].0;
        
        println!("[DEBUG] High-confidence key size found : {}", key_size);
        let key = guess_key(ciphertext, key_size);
        let plaintext = repeating_key(ciphertext, &key);

        return (key, plaintext);
    }

    println!("[DEBUG] No high-confidence key size found, trying top-{} key sizes...", key_sizes.len());

    let mut scores: Vec<(ByteVec, ByteVec, f64)> = vec![];

    for (key_size, _) in key_sizes
    {
        let key       = guess_key(ciphertext, key_size);
        let plaintext = repeating_key(ciphertext, &key);
        let plaintext_score = frequency_score(&plaintext);

        scores.push((key, plaintext, plaintext_score));
    }

    scores.sort_by( |a, b| b.2.partial_cmp(&a.2).unwrap() );

    for score in &scores
    {
        let extract = match score.1.to_string()
        {
            Ok(string) => string[..min(10, string.len())].to_string(),
            Err(_) => String::from("ERR"),
        };

        println!("Score: {} / Plain text (extract): {} [...] / Key: {}", score.2, extract, score.0.to_hex());
    }

    let result = scores.remove(0);

    return (result.0, result.1);
}

#[cfg(test)]
mod tests
{
    use crate::lib::byte::ByteVec;

    #[test]
    fn hamming_distance()
    {
        assert_eq!(
            37,
            super::hamming_distance(
                &ByteVec::from("this is a test"), 
                &ByteVec::from("wokka wokka!!!")
            )
        )
    }

    #[test]
    fn slice_block()
    {
        assert_eq!(
            vec![
                ByteVec::from_hex("010203"),
                ByteVec::from_hex("040506"),
                ByteVec::from_hex("070809")
            ],
            super::slice_block(
                &ByteVec::from_hex("010203040506070809"), 
                3
            )
        );

        assert_eq!(
            vec![
                ByteVec::from_hex("01020304"),
                ByteVec::from_hex("05060708"),
                ByteVec::from_hex("09")
            ],
            super::slice_block(
                &ByteVec::from_hex("010203040506070809"), 
                4
            )
        );
    }

    #[test]
    fn transpose()
    {
        assert_eq!(
            vec![
                ByteVec::from_hex("0105"),
                ByteVec::from_hex("0206"),
                ByteVec::from_hex("0307"),
                ByteVec::from_hex("0408"),
            ],
            super::transpose(
                &vec![
                    ByteVec::from_hex("01020304"),
                    ByteVec::from_hex("05060708")
                ]
            )
        );

        assert_eq!(
            vec![
                ByteVec::from_hex("010509"),
                ByteVec::from_hex("02060A"),
                ByteVec::from_hex("03070B"),
                ByteVec::from_hex("0408"),
            ],
            super::transpose(
                &vec![
                    ByteVec::from_hex("01020304"),
                    ByteVec::from_hex("05060708"),
                    ByteVec::from_hex("090A0B"),
                ]
            )
        );
    }
}