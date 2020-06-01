use crate::lib::crypto::xor;
use crate::lib::byte::ByteVec;
use std::collections::HashMap;

const ENGLISH_LETTERS_FREQUENCIES: [(char, f64); 29] = [
    ('E', 0.11132),
    ('T', 0.09356),
    ('A', 0.08497),
    ('R', 0.07587),
    ('I', 0.07546),
    ('O', 0.07507),
    ('N', 0.06749),
    ('S', 0.06327),
    ('H', 0.06094),
    ('D', 0.04253),
    ('L', 0.04025),
    ('U', 0.02758),
    ('W', 0.02560),
    ('M', 0.02406),
    ('F', 0.02228),
    ('C', 0.02202),
    ('G', 0.02015),
    ('Y', 0.01994),
    ('P', 0.01929),
    ('B', 0.01492),
    ('K', 0.01292),
    ('V', 0.00978),
    ('J', 0.00153),
    ('X', 0.00150),
    ('Q', 0.00095),
    ('Z', 0.00077),
    // Add a little score to ponctuation
    ('.', 0.00001),
    (',', 0.00001),
    ('\n', 0.00001),
];

fn frequency_analysis(string: &String) -> HashMap::<char, f64>
{
    let mut occurences = HashMap::<char, f64>::new();
    let total_chars = string.len() as f64;

    for c in string.chars()
    {
        *occurences.entry(c).or_insert(0.0) += 1.0 / total_chars;
    }

    return occurences;
}

fn freq_english_score(freqs: &HashMap::<char, f64>) -> f64
{
    let mut score = 0.0;

    for (c, freq_ref) in ENGLISH_LETTERS_FREQUENCIES.iter()
    {
        let freq = if freqs.contains_key(c) { freqs[c] } else { 0.0 };
        score += (freq_ref * freq).sqrt();
    }

    return score;
}

fn frequency_score(bytes: &ByteVec) -> f64
{
    let string_result = bytes.to_string();

    if !string_result.is_ok() { return -f64::INFINITY; }

    let string  = string_result.unwrap().to_uppercase();
    let freqs   = frequency_analysis(&string);

    return freq_english_score(&freqs);
}

pub fn decrypt(ciphertext: &ByteVec) -> (u8, ByteVec, f64)
{
    let mut output = Vec::<(u8, ByteVec, f64)>::new();

    // Brute-force every key (except 0)
    for i in 0x01..0xFF
    {
        let key = ByteVec::from_hex(
            &format!("{:0>2x}", i).repeat(ciphertext.len())
        );

        let decrypted = xor::fixed(&ciphertext, &key);
        let score = frequency_score(&decrypted);        

        output.push((i, decrypted, score));
    }

    output.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    /*
    for kv in &output
    {
        println!("{:0>2x} {} ({})", kv.0, kv.1.to_string().unwrap_or(String::from("...")), kv.2);
    }
    */

    let first_result = output.remove(0);
    return (first_result.0, first_result.1, first_result.2);
}

#[cfg(test)]
mod tests
{
    use std::collections::HashMap;

    #[test]
    fn frequency_analysis()
    {
        let string = String::from("aabc");
        let freqs: HashMap<char, f64> = [('a', 0.5), ('b', 0.25), ('c', 0.25)].iter().cloned().collect();

        assert_eq!(freqs, super::frequency_analysis(&string));
    }
}