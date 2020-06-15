use super::ByteVec;
use crate::lib::traits::{FromBase64, ToBase64, ToString};

const BASE64_DECODING_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
static BASE64_ENCODING_TABLE: &[u8] = BASE64_DECODING_TABLE.as_bytes();

impl FromBase64 for ByteVec
{
    fn from_base64(string: &str) -> Self
    {
        // Strip space and line return
        let string = string.replace(" ", "").replace("\n", "");

        if string.len() % 4 != 0 { panic!("Malformed base64 string."); }

        let mut result = ByteVec::new();

        for i in (0..string.len()).step_by(4)
        {
            let sextets = chars_to_sextets(&string[i..i + 4]);

            let bytes: Vec<u8> = match sextets.len()
            {
                2 => 
                {
                    vec![
                        sextets[0] << 2 | sextets[1] >> 4,
                    ]
                },
                3 =>
                {
                    vec![
                        sextets[0] << 2 | sextets[1] >> 4,
                        (sextets[1] & 0xF) << 4 | sextets[2] >> 2,
                    ]
                },
                _ => 
                {
                    vec![
                        sextets[0] << 2 | sextets[1] >> 4,
                        (sextets[1] & 0xF) << 4 | sextets[2] >> 2,
                        (sextets[2] & 0x3) << 6 | sextets[3]
                    ]
                }
            };

            for b in bytes { result.push(b); }
        }

        return result;
    }
}

impl ToBase64 for ByteVec
{
    fn to_base64(&self) -> String 
    {
        let n_bits  = self.len() * 8;
        // Shift can be 2, 4 or 6 (aka 0)
        let shift = (6 - (n_bits % 6)) % 6;
        let padding  = (4 - ((n_bits + shift) / 6) % 4) % 4;
        let last_full_block = self.len() - self.len() % 3;

        let mut output = String::new();

        for i in (0..last_full_block).step_by(3)
        {
            let sextets = [
                (self[i] >> 2),
                (self[i] & 0x3) << 4 | (self[i + 1] >> 4),
                (self[i + 1] & 0xF) << 2 | (self[i + 2] >> 6),
                self[i + 2] & 0x3F
            ];

            for b64_idx in sextets.iter()
            {
                output.push(BASE64_ENCODING_TABLE[*b64_idx as usize] as char);
            }
        }

        // 2-octet into 3-sextet (with 2bit zero-padding)
        if shift == 2
        {
            let sextets = [
                (self[last_full_block] >> 2),
                (self[last_full_block] & 0x3) << 4 | (self[last_full_block + 1] >> 4),
                (self[last_full_block] & 0xF) << 2
            ];

            for b64_idx in sextets.iter()
            {
                output.push(BASE64_ENCODING_TABLE[*b64_idx as usize] as char);
            }
        }
        // 1-octet into 2-sextet (with 4bit zero-padding)
        else if shift == 4
        {
            let sextets = [
                (self[last_full_block] >> 2),
                (self[last_full_block] & 0x3) << 4
            ];

            for b64_idx in sextets.iter()
            {
                output.push(BASE64_ENCODING_TABLE[*b64_idx as usize] as char);
            }
        }

        output.push_str(&"=".repeat(padding));

        output
    }
}

fn chars_to_sextets(string: &str) -> Vec<u8>
{
    let mut sextets = vec![];

    for c in string.chars()
    {
        if let Some(idx) = BASE64_DECODING_TABLE.chars().position( |b64_c| b64_c == c )
        {
            sextets.push(idx as u8);
        }
    }

    return sextets;
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn to_base64()
    {
        let data = [
            ("Test", "VGVzdA=="),
            ("I'm killing your brain like a poisonous mushroom", "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
        ];

        for kv in data.iter()
        {
            assert_eq!(kv.1, ByteVec::from(kv.0).to_base64())
        }
    }

    #[test]
    fn from_base64()
    {
        let data = [
            ("Test", "VGVzdA=="),
            ("I'm killing your brain like a poisonous mushroom", "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"),
            ("Lorem ipsum", "TG9yZW0g\naXBzdW0=")
        ];

        for kv in data.iter()
        {
            assert_eq!(
                kv.0, 
                ByteVec::from_base64(kv.1).to_string().unwrap()
            )
        }
    }
}