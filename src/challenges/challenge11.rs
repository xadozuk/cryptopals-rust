use crate::lib::types::{Byte, ByteVec};
use crate::lib::traits::{Random};
use crate::lib::crypto::aes;
use crate::attacks;

use rand::Rng;

fn encryption_oracle(input: &[Byte]) -> (ByteVec, aes::AesMode)
{
    let mut rng = rand::thread_rng();

    let mut data = ByteVec::random(rng.gen_range(5, 11));
    data.extend(input);
    data.extend(ByteVec::random(rng.gen_range(5, 11)));

    let msg = aes::Message::from(data, aes::Key::new(&ByteVec::random(16)))
                            .with_iv(ByteVec::random(16));

    match rng.gen_range(1, 3)
    {
        1 => (aes::encrypt(aes::AesMode::ECB, &msg), aes::AesMode::ECB),
        2 => (aes::encrypt(aes::AesMode::CBC, &msg), aes::AesMode::CBC),
        _ => (vec![], aes::AesMode::ECB) // Not possible
    }
}

pub fn challenge11()
{
    println!("=== [ Challenge 11 ] ===");

    const INPUT_SIZE: usize = 16 * 4;
    const ROUNDS: usize     = 100;
    const THRESHOLD: f64    = 0.2;

    let mut results = vec![];

    for r in 0..ROUNDS
    {
        let chosen_input = vec![0xA; INPUT_SIZE];
        let encryption = encryption_oracle(&chosen_input);

        let score = attacks::aes::detect_ecb(&encryption.0, 16);
        let detected_mode = match score > THRESHOLD
        {
            true    => aes::AesMode::ECB,
            false   => aes::AesMode::CBC
        };

        //println!("Round #{:<2} ({}) : guessed_mode({}), score({})", r, encryption.1, detected_mode, score);

        results.push((encryption.1 == detected_mode) as u32);
    }

    let total_score: f64 = results.iter().map( |n| *n as f64 ).sum::<f64>() / ROUNDS as f64;

    println!("Detection score : {}%", total_score * 100.0)    ;
}