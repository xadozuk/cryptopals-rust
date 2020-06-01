use crate::attacks;
use crate::lib;
use std::fs::File;
use std::io::{self, BufRead};

use lib::byte::ByteVec;

pub fn challenge3()
{
    let result = attacks::xor::single_byte::decrypt(
        &ByteVec::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
    );

    println!("Score: {}\nKey: 0x{:0>2x}\nPlain text:{}", result.2, result.0, result.1.to_string().unwrap());
}

pub fn challenge4()
{
    const FILE_PATH: &str = "./data/4.txt";

    let file = File::open(FILE_PATH).unwrap();
    let lines: Vec<String> = io::BufReader::new(file).lines().map( |l| l.unwrap()).collect();
    let line_count = lines.len();

    let mut decrypted_lines: Vec<(usize, (u8, ByteVec, f64))> = vec![];

    println!("Loaded {} lines from file.", line_count);
    println!("Decrypting...");

    for (i, line) in lines.iter().enumerate()
    {
        decrypted_lines.push(
            (
                i,
                attacks::xor::single_byte::decrypt(
                    &ByteVec::from_hex(&line)
                )
            )
        );
    }

    println!("Done.\nSearching...");

    decrypted_lines.sort_by(|a, b| (b.1).2.partial_cmp(&(a.1).2).unwrap());
    let first_result = &decrypted_lines[0];
    let guess = &first_result.1;

    println!(
        "=== Guess ===\nLine: {} ({})\nScore: {}\nKey: {:0>2x}\nPlain tex: {}", 
        first_result.0 + 1, lines[first_result.0],
        guess.2, 
        guess.0, 
        guess.1.to_string().unwrap()
    );
}