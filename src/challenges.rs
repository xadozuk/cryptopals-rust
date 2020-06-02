use crate::attacks;
use crate::lib;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};

use lib::byte::ByteVec;
use crate::lib::base64::Base64;

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

pub fn challenge6()
{
    const FILE_PATH: &str = "./data/6.txt";

    let content = fs::read_to_string(FILE_PATH).unwrap();
    let ciphertext = ByteVec::from_base64(&content);

    // Key : 37e02c95
    // let ciphertext = ByteVec::from_base64(
    //     "e49e8FrAReVElUG1U49A+kXAX/xDwE34UpQAtVSPQuZSg1jwQ5VetVaEReVek0/
    //     8WYcM8FuJWLsXpEP7UoMM9FTAQPRYkknwQ8BY4EWQReYZwH/wU8BJ81GJT/xDlV6
    //     1R4hN51KUXvQXjE32QpMAtUGFQLVUj0LxXo1J+0OVQbVSkkPmF4ND+1CVSbs="
    // );

    // Key : 633bb151099a4141b4c040cebb2ba043
    let ciphertext = ByteVec::from_base64(
        "KhvXPnz0JWHV4CyhzU6qBQxJkTxskAUgxqwpoNwHgCkWSMVxbfM3JJSyKanTX4AqDTHwP226Jy7YrC+5m0bZYw9e0DUDzSQt2Oxgh5tNzzYNX5EwKf0oM9jKAqvaXtQqBU7dcWj0JWHHtyWrzyHvK08b+HFn/zckxuAroN5cgDoMTpEmbOgkYcCoJe7IRM0mDFXUW177KDXdrifu3UTSYw5eu3ZK+zQy0eA3q5tcxTEGG9skeu5hKt2kM+7MQ8UtQ0zUcW//LS2UqS7u10TWJml13iUp8S8uw6kuqZtcyCIXG9glKe0gMr6JYLnSR8xjDVTFcW7zNySUuS+7m17QSTdT2CIp7igs0coCu88LxCIRV9g/brZhK8GzNO7QQtMwQ1bUcXr2Lja+mS+7yQvIJgJJxXFg6WEg2Kxgh5tE1y1pet81KfMvYc2vNbybTtkmEBeRKGbvZjPR4Cih10/JLQQb3Dhn/0sD1aI54ptihy5pf9A/avMvJpSpLu7PQ8VjB1rDOgPNKDXc4DmhzgvCJhdM1DRnuiw4lKEyo8gh4iIRXtc+Zu5hLtrgNKbeC8cxAkjCW0XzMjXRrimg3AvULENUxCMp/CA327Iput4L0ywNXLsGYf8vYc2vNe7ISsknQ0LeJCn2Li7fpSTu2gvNJhBIuxgp7Skox7AlvN5PgDYNX9QjZ/8gNdzgLbebSdImAk/ZW0vvNWHNrzXu007BMQcb2CUD3iAz2KkuqZcL2SwWG90+ZvFhMdGyJqvYX4xjF1TfOG7yNUvjpSyim2KAJQxO3zUp+2E2260hoLF41DEMVdY0e7o1KdWuYK/VUs8tBhv4cWL0Lja+kyirm1jIIhFewnFk42ElxqUho8gh6WMLVME0Ke4pIMDgM6HWTsQiGjH4dmX2YTLcoTKrm0PFMUNT3jxskAhh0q81oN8LwWMPVMc0A84uYdehMrzCC80sEV6RJWH7L2HetTO6m0bZYxBe0iNs7jJL4K9grdpZ0jpDV94nbJAVLpSjIbzJUoAgC1LdNXv/L0v7pmChzlmALBRVuwZsuiAz0eAzutJHzGMIUtUiJbojNMDgN6ucWcVjEFSROGe6LS7CpUqI0kzINwpV1nFo/SAo2rM07tpHzGMMX9UiA9NhKtqvN+7MTocvDxvTNCn7LTPdpyi6sX/IKhAbxThk/0sF1bIsp9VMjGMJTsIlKfIuLdDgLbebQ8EtBzHzNCn3OGHTqTKilwvpZA9XkTNsujguwbJgo9pFqgpDSNQ0Kfc4YdK1NLvJToAqDRvIPnzoYSTNpTPE+UrCOk8b+HZkkAUg2qMpoNwLyS1DT9k0Kf4gM9/KF6fPQ4A6DE6RM2zuNiTRrmCjwgvBMQ5IuxNo6CQn26807tRFgDcLXpE2e/syMr6MKb3PTs4qDVyRJWa6LjTG4CavzUTSKhdekSJm9CZL46gloJtigDACTJEoZu9hKNrgNKbaX4AnEV7CIiW6LS7bqymg3AvTLENZ1DB87ignwaxKh5tPzy1ET5E1bOkkM8KlYLrTQtNJJ1rDPWD0Jm2UuS+7m0fPLAgbwTR7/CQiwOxgutRFySQLT7sTaPg4bZSJZ6Oxb8EtAFLfNinzL2HAqCXu30rSKGls2CVhujguweAiq89cxSYNG9woKfszLMfKAq/JTsYsDE+RPme6NSnR4Ce82ljTSS9SwiVs9Cgv0+A0oZtE1TFDXdAnZugoNdHgM6HVTKoKQ1PQJ2y6JyDdtCju0kWANAtaxXFAujIk0coOocwL6WMIVd4mKdNhKdW2Je7WTtRjAlWRMGf9JC2UqS7uy07SMAxVuxBn/mEy3KVgotREyzBDS9Qjb/8iNb6JYKrURYc3Q1/UImzoNySUtCinyCH5LBYb3T5m8WEx0bImq9hfgDcMVdg2Ye4="
    );

    let result = attacks::xor::repeating_key::decrypt(&ciphertext);

    println!(
        "=== Guess ===\nKey: 0x{}\nPlain text:{}",
        result.0.to_hex(),
        result.1.to_string().unwrap_or(String::from("..."))
    );
}