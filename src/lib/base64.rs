use crate::lib::byte::ByteVec;

static BASE64_TABLE: &[u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();

trait Base64
{
    fn to_base64(self) -> String;
}

impl Base64 for ByteVec
{
    fn to_base64(self) -> String 
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
                output.push(BASE64_TABLE[*b64_idx as usize] as char);
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
                output.push(BASE64_TABLE[*b64_idx as usize] as char);
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
                output.push(BASE64_TABLE[*b64_idx as usize] as char);
            }
        }

        output.push_str(&"=".repeat(padding));

        output
    }
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
}