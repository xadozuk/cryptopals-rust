use crate::lib::types::{Byte, ByteVec};
use crate::lib::traits::ToString;
use crate::lib::crypto::aes::traits::KeyExpansion;

use crate::lib::crypto::aes::consts::{RCON, SBOX};

use crate::lib::math::byte::xor;

use std::fmt;

pub struct Key
{
    key: ByteVec,
    block_size: usize,
    rounds: usize
}

impl Key
{
    pub fn new(key: &[Byte]) -> Key
    {
        let rounds = match key.len() 
        {
            16 => 10,  // 16*8 = 128
            24 => 12,  // 24*8 = 192
            32 => 14,  // 32*8 = 256
            _ => panic!("Malformed key")
        };

        Key {
            key: ByteVec::from(key),
            block_size: 4,
            rounds: rounds,
        }
    }

    pub fn length(&self) -> usize
    {
        // ByteVec (already 8 bits)
        return self.key.len() / 4;
    }

    fn sub_word(&self, w: &[Byte]) -> ByteVec
    {
        if w.len() != 4 { panic!("Malformed word") }

        (0..4).map( |i| SBOX[w[i] as usize] ).collect()
    }

    fn rot_word(&self, w: &[Byte]) -> ByteVec
    {
        if w.len() != 4 { panic!("Malformed word") }

        return vec![w[1], w[2], w[3], w[0]]
    }
}

impl KeyExpansion for Key
{
    fn expand(&self) -> ByteVec
    {
        let mut rounds_key = vec![];

        rounds_key.extend(self.key.iter());

        for i in self.length()..self.block_size * (self.rounds + 1)
        {
            let mut t: ByteVec = rounds_key[(i - 1)*4..4*i].to_vec();

            if i % self.length() == 0
            {
                t = xor(&self.sub_word(&self.rot_word(&t)), &[RCON[i / self.length()], 0x0, 0x0, 0x0]);
            }
            else if self.length() > 6 && i % self.length() == 4
            {
                t = self.sub_word(&t);
            }
            
            rounds_key.extend(
                xor(&rounds_key[4*(i - self.length())..4*(i - self.length() + 1)], &t)
            );
        }

        return rounds_key;
    }
}

impl fmt::Display for Key
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        match self.key.to_string()
        {
            Ok(s) => writeln!(f, "{}", s),
            Err(e) => Err(fmt::Error)
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::Key;
    use crate::lib::crypto::aes::traits::KeyExpansion;

    #[test]
    fn expand()
    {
        let key = Key::new(&vec![0x00; 16]);

        assert_eq!(
            vec![
                0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
                0x62,0x63,0x63,0x63,0x62,0x63,0x63,0x63,0x62,0x63,0x63,0x63,0x62,0x63,0x63,0x63,
                0x9b,0x98,0x98,0xc9,0xf9,0xfb,0xfb,0xaa,0x9b,0x98,0x98,0xc9,0xf9,0xfb,0xfb,0xaa,
                0x90,0x97,0x34,0x50,0x69,0x6c,0xcf,0xfa,0xf2,0xf4,0x57,0x33,0x0b,0x0f,0xac,0x99,
                0xee,0x06,0xda,0x7b,0x87,0x6a,0x15,0x81,0x75,0x9e,0x42,0xb2,0x7e,0x91,0xee,0x2b,
                0x7f,0x2e,0x2b,0x88,0xf8,0x44,0x3e,0x09,0x8d,0xda,0x7c,0xbb,0xf3,0x4b,0x92,0x90,
                0xec,0x61,0x4b,0x85,0x14,0x25,0x75,0x8c,0x99,0xff,0x09,0x37,0x6a,0xb4,0x9b,0xa7,
                0x21,0x75,0x17,0x87,0x35,0x50,0x62,0x0b,0xac,0xaf,0x6b,0x3c,0xc6,0x1b,0xf0,0x9b,
                0x0e,0xf9,0x03,0x33,0x3b,0xa9,0x61,0x38,0x97,0x06,0x0a,0x04,0x51,0x1d,0xfa,0x9f,
                0xb1,0xd4,0xd8,0xe2,0x8a,0x7d,0xb9,0xda,0x1d,0x7b,0xb3,0xde,0x4c,0x66,0x49,0x41,
                0xb4,0xef,0x5b,0xcb,0x3e,0x92,0xe2,0x11,0x23,0xe9,0x51,0xcf,0x6f,0x8f,0x18,0x8e
            ],
            key.expand()
        );

        let key = Key::new(&vec![0xff; 16]);

        assert_eq!(
            vec![
                0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,
                0xe8,0xe9,0xe9,0xe9,0x17,0x16,0x16,0x16,0xe8,0xe9,0xe9,0xe9,0x17,0x16,0x16,0x16,
                0xad,0xae,0xae,0x19,0xba,0xb8,0xb8,0x0f,0x52,0x51,0x51,0xe6,0x45,0x47,0x47,0xf0,
                0x09,0x0e,0x22,0x77,0xb3,0xb6,0x9a,0x78,0xe1,0xe7,0xcb,0x9e,0xa4,0xa0,0x8c,0x6e,
                0xe1,0x6a,0xbd,0x3e,0x52,0xdc,0x27,0x46,0xb3,0x3b,0xec,0xd8,0x17,0x9b,0x60,0xb6,
                0xe5,0xba,0xf3,0xce,0xb7,0x66,0xd4,0x88,0x04,0x5d,0x38,0x50,0x13,0xc6,0x58,0xe6,
                0x71,0xd0,0x7d,0xb3,0xc6,0xb6,0xa9,0x3b,0xc2,0xeb,0x91,0x6b,0xd1,0x2d,0xc9,0x8d,
                0xe9,0x0d,0x20,0x8d,0x2f,0xbb,0x89,0xb6,0xed,0x50,0x18,0xdd,0x3c,0x7d,0xd1,0x50,
                0x96,0x33,0x73,0x66,0xb9,0x88,0xfa,0xd0,0x54,0xd8,0xe2,0x0d,0x68,0xa5,0x33,0x5d,
                0x8b,0xf0,0x3f,0x23,0x32,0x78,0xc5,0xf3,0x66,0xa0,0x27,0xfe,0x0e,0x05,0x14,0xa3,
                0xd6,0x0a,0x35,0x88,0xe4,0x72,0xf0,0x7b,0x82,0xd2,0xd7,0x85,0x8c,0xd7,0xc3,0x26
            ],
            key.expand()
        )
    }
}