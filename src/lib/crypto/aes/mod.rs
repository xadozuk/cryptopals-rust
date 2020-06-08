use crate::lib::types::{Byte, Matrix, ByteVec};

const RCON: [u8; 16] = [
    0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a
];

const SBOX: [u8; 256] = [
    0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
    0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
    0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,
    0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,
    0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,
    0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,
    0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,
    0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,
    0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,
    0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,
    0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,
    0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,
    0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,
    0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,
    0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
    0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16
];

const SBOX_INV: [u8; 256] = [
    0x52, 0x09, 0x6A, 0xD5, 0x30, 0x36, 0xA5, 0x38, 0xBF, 0x40, 0xA3, 0x9E, 0x81, 0xF3, 0xD7, 0xFB,
    0x7C, 0xE3, 0x39, 0x82, 0x9B, 0x2F, 0xFF, 0x87, 0x34, 0x8E, 0x43, 0x44, 0xC4, 0xDE, 0xE9, 0xCB,
    0x54, 0x7B, 0x94, 0x32, 0xA6, 0xC2, 0x23, 0x3D, 0xEE, 0x4C, 0x95, 0x0B, 0x42, 0xFA, 0xC3, 0x4E,
    0x08, 0x2E, 0xA1, 0x66, 0x28, 0xD9, 0x24, 0xB2, 0x76, 0x5B, 0xA2, 0x49, 0x6D, 0x8B, 0xD1, 0x25,
    0x72, 0xF8, 0xF6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xD4, 0xA4, 0x5C, 0xCC, 0x5D, 0x65, 0xB6, 0x92,
    0x6C, 0x70, 0x48, 0x50, 0xFD, 0xED, 0xB9, 0xDA, 0x5E, 0x15, 0x46, 0x57, 0xA7, 0x8D, 0x9D, 0x84,
    0x90, 0xD8, 0xAB, 0x00, 0x8C, 0xBC, 0xD3, 0x0A, 0xF7, 0xE4, 0x58, 0x05, 0xB8, 0xB3, 0x45, 0x06,
    0xD0, 0x2C, 0x1E, 0x8F, 0xCA, 0x3F, 0x0F, 0x02, 0xC1, 0xAF, 0xBD, 0x03, 0x01, 0x13, 0x8A, 0x6B,
    0x3A, 0x91, 0x11, 0x41, 0x4F, 0x67, 0xDC, 0xEA, 0x97, 0xF2, 0xCF, 0xCE, 0xF0, 0xB4, 0xE6, 0x73,
    0x96, 0xAC, 0x74, 0x22, 0xE7, 0xAD, 0x35, 0x85, 0xE2, 0xF9, 0x37, 0xE8, 0x1C, 0x75, 0xDF, 0x6E,
    0x47, 0xF1, 0x1A, 0x71, 0x1D, 0x29, 0xC5, 0x89, 0x6F, 0xB7, 0x62, 0x0E, 0xAA, 0x18, 0xBE, 0x1B,
    0xFC, 0x56, 0x3E, 0x4B, 0xC6, 0xD2, 0x79, 0x20, 0x9A, 0xDB, 0xC0, 0xFE, 0x78, 0xCD, 0x5A, 0xF4,
    0x1F, 0xDD, 0xA8, 0x33, 0x88, 0x07, 0xC7, 0x31, 0xB1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xEC, 0x5F,
    0x60, 0x51, 0x7F, 0xA9, 0x19, 0xB5, 0x4A, 0x0D, 0x2D, 0xE5, 0x7A, 0x9F, 0x93, 0xC9, 0x9C, 0xEF,
    0xA0, 0xE0, 0x3B, 0x4D, 0xAE, 0x2A, 0xF5, 0xB0, 0xC8, 0xEB, 0xBB, 0x3C, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2B, 0x04, 0x7E, 0xBA, 0x77, 0xD6, 0x26, 0xE1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0C, 0x7D
];

struct State
{
    bytes: Matrix<Byte>
}

impl State
{
    const SIZE: usize = 4;

    pub fn new() -> Self
    {
        State {
            bytes: Matrix::<Byte>::new(Self::SIZE, Self::SIZE)
        }
    }

    pub fn from(input: &ByteVec) -> Self
    {
        Self::assert_size(input);

        let mut state = Self::new();

        for r in 0..Self::SIZE
        {
            for c in 0..Self::SIZE
            {
                state.bytes[r][c] = input[r + 4 * c]
            }
        }

        state
    }

    pub fn output(&self) -> ByteVec
    {
        let mut output = vec![0x0; 16];

        for r in 0..Self::SIZE
        {
            for c in 0..Self::SIZE
            {
                output[r + 4 * c] = self.bytes[r][c];
            }
        }

        return output;
    }

    pub fn add_round_key(&mut self, round_key: &[Byte])
    {
        Self::assert_size(round_key);

        for c in 0..Self::SIZE
        {
            for r in 0..Self::SIZE
            {
                self.bytes[r][c] ^= round_key[c * 4 + r]
            }
        }
    }

    pub fn sub_bytes(&mut self)
    {
        for r in 0..Self::SIZE
        {
            for c in 0..Self::SIZE
            {
                self.bytes[r][c] = SBOX[self.bytes[r][c] as usize];
            }
        }
    }

    pub fn shift_rows(&mut self)
    {
        for i in 1..Self::SIZE
        {
            self.shift_row(i, i);
        }
    }

    pub fn mix_columns(&mut self)
    {
        let state = self.bytes.clone();

        for c in 0..Self::SIZE
        {
            self.bytes[0][c] = gf_mul(0x2, state[0][c]) ^ gf_mul(0x3, state[1][c]) ^ state[2][c] ^ state[3][c];
            self.bytes[1][c] = state[0][c] ^ gf_mul(0x2, state[1][c]) ^ gf_mul(0x3, state[2][c]) ^ state[3][c];
            self.bytes[2][c] = state[0][c] ^ state[1][c] ^ gf_mul(0x2, state[2][c]) ^ gf_mul(0x3, state[3][c]);
            self.bytes[3][c] = gf_mul(0x3, state[0][c]) ^ state[1][c] ^ state[2][c] ^ gf_mul(0x2, state[3][c]);
        }
    }

    fn shift_row(&mut self, row: usize, shift: usize)
    {
        self.bytes[row] = (0..Self::SIZE)
            .map( |i| self.bytes[row][(shift + i) % Self::SIZE])
            .collect();
    }

    fn assert_size(block: &[Byte])
    {
        if block.len() != Self::SIZE * Self::SIZE
        {
            panic!("Malformed block");
        }
    }
}

pub struct Aes128
{
    key_length: usize,
    block_size: usize,
    nb_rounds: usize
}

impl Aes128
{
    pub fn new() -> Self
    {
        Aes128 {
            key_length: 4,
            block_size: 4,
            nb_rounds: 10
        }
    }

    fn key_rounds(&self, key: &ByteVec) -> ByteVec
    {
        if key.len() != 16 { panic!("Malformed key") }

        let mut rounds_key = vec![];

        rounds_key.extend(key);

        for i in self.key_length..self.block_size * (self.nb_rounds + 1)
        {
            let mut t: ByteVec = rounds_key[(i - 1)*4..4*i].to_vec();

            if i % self.key_length == 0
            {
                t = xor(&self.sub_word(&self.rot_word(&t)), &[RCON[i / self.key_length], 0x0, 0x0, 0x0]);
            }
            
            rounds_key.extend(
                xor(&rounds_key[4*(i - self.key_length)..4*(i - self.key_length + 1)], &t)
            );
        }

        return rounds_key;
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

    fn cipher(&self, input: &ByteVec, round_keys: &ByteVec) -> ByteVec
    {
        let mut state = State::from(input);
        let kr_size = 4 * self.block_size;

        state.add_round_key(&round_keys[0..kr_size]);

        for r in 1..self.nb_rounds
        {
            state.sub_bytes();
            state.shift_rows();
            state.mix_columns();

            state.add_round_key(&round_keys[r*kr_size..(r+1)*kr_size]);
        }

        state.sub_bytes();
        state.shift_rows();
        state.add_round_key(&round_keys[self.nb_rounds*kr_size..(self.nb_rounds+1)*kr_size]);

        state.output()
    }
}

fn gf_mul(a: Byte, b: Byte) -> Byte
{
    let mut a16 = a as u16;
    let mut b16 = b as u16;
    let mut result = 0;

    for c in 0..8
    {
        if b16 & 0x1 == 0x1
        {
            result ^= a16;
        }

        a16 <<= 1;

        if a16 & 0x100 == 0x100
        {
            a16 ^= 0x11B;
        }

        b16 >>= 1;
    }

    result as Byte
}

fn xor(a: &[Byte], b: &[Byte]) -> Vec<Byte>
{
    if a.len() != b.len() { panic!("a & b must be of the same size") }

    a.iter().zip(b)
        .map( |(a, b)| a ^ b )
        .collect()
}

#[cfg(test)]
mod tests
{
    mod state
    {
        use super::super::State;
        use crate::lib::types::Matrix;
        use crate::lib::types::Byte;

        #[test]
        fn from()
        {
            let s = State::from(
                &vec![
                    0x0, 0x1, 0x2, 0x3,
                    0x4, 0x5, 0x6, 0x7,
                    0x8, 0x9, 0xA, 0xB,
                    0xC, 0xD, 0xE, 0xF
                ]
            );

            assert_eq!(
                Matrix::<Byte>::from(&vec![
                    vec![0x0, 0x4, 0x8, 0xC],
                    vec![0x1, 0x5, 0x9, 0xD],
                    vec![0x2, 0x6, 0xA, 0xE],
                    vec![0x3, 0x7, 0xB, 0xF]
                ]),
                s.bytes
            );
        }

        #[test]
        fn output()
        {
            let s = State::from(
                &vec![
                    0x0, 0x1, 0x2, 0x3,
                    0x4, 0x5, 0x6, 0x7,
                    0x8, 0x9, 0xA, 0xB,
                    0xC, 0xD, 0xE, 0xF
                ]
            );

            assert_eq!(
                vec![
                    0x0, 0x1, 0x2, 0x3,
                    0x4, 0x5, 0x6, 0x7,
                    0x8, 0x9, 0xA, 0xB,
                    0xC, 0xD, 0xE, 0xF
                ],
                s.output()
            );
        }
    }

    mod aes128
    {
        use super::super::*;
        use crate::lib::types::ByteVec;
        use crate::lib::traits::FromHex;

        #[test]
        fn new()
        {
            let aes = Aes128::new();

            assert_eq!(4, aes.key_length);
            assert_eq!(4, aes.block_size);
            assert_eq!(10, aes.nb_rounds);
        }

        #[test]
        fn cipher()
        {
            let aes = Aes128::new();
            let key_rounds = aes.key_rounds(&ByteVec::from("YELLOW SUBMARINE"));

            let ciphertext = aes.cipher(&ByteVec::from("Lorem ipsum dolo"), &key_rounds);

            assert_eq!(
                ByteVec::from_hex("eadcc5aa4800dff175a49cf3a0f2041d"),
                ciphertext
            )
        }

        #[test]
        fn sub_word()
        {
            let aes = Aes128::new();

            assert_eq!(
                vec![0x82, 0x93, 0xc3, 0x1b],
                aes.sub_word(&vec![0x11, 0x22, 0x33, 0x44])
            );
        }

        #[test]
        fn rot_word()
        {
            let aes = Aes128::new();

            assert_eq!(
                vec![0x2, 0x3, 0x4, 0x1],
                aes.rot_word(&vec![0x1, 0x2, 0x3, 0x4])
            );
        }

        #[test]
        fn key_rounds()
        {
            let aes = Aes128::new();
            let key = vec![0x00; 16];

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
                aes.key_rounds(&key)
            );

            let key = vec![0xff; 16];

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
                aes.key_rounds(&key)
            )
        }

        fn do_cipher_round(round: usize, state: &mut State, round_keys: &ByteVec)
        {
            state.sub_bytes();
            state.shift_rows();
            state.mix_columns();

            state.add_round_key(&round_keys[round*16..(round+1)*16]);
        }

        #[test]
        fn cipher_vector_test()
        {
            let aes = Aes128::new();

            let plaintext = ByteVec::from_hex("00112233445566778899aabbccddeeff");
            let key = ByteVec::from_hex("000102030405060708090a0b0c0d0e0f");

            let round_keys = aes.key_rounds(&key);

            let expected = vec![
                (
                    "00102030405060708090a0b0c0d0e0f0",
                    "63cab7040953d051cd60e0e7ba70e18c",
                    "6353e08c0960e104cd70b751bacad0e7",
                    "5f72641557f5bc92f7be3b291db9f91a",
                    "d6aa74fdd2af72fadaa678f1d6ab76fe"
                ),
                (
                    "89d810e8855ace682d1843d8cb128fe4",
                    "a761ca9b97be8b45d8ad1a611fc97369",
                    "a7be1a6997ad739bd8c9ca451f618b61",
                    "ff87968431d86a51645151fa773ad009",
                    "b692cf0b643dbdf1be9bc5006830b3fe",
                ),
                (
                    "4915598f55e5d7a0daca94fa1f0a63f7",
                    "3b59cb73fcd90ee05774222dc067fb68",
                    "3bd92268fc74fb735767cbe0c0590e2d",
                    "4c9c1e66f771f0762c3f868e534df256",
                    "b6ff744ed2c2c9bf6c590cbf0469bf41",
                ),
                (
                    "fa636a2825b339c940668a3157244d17",
                    "2dfb02343f6d12dd09337ec75b36e3f0",
                    "2d6d7ef03f33e334093602dd5bfb12c7",
                    "6385b79ffc538df997be478e7547d691",
                    "47f7f7bc95353e03f96c32bcfd058dfd",
                ),
                (
                    "247240236966b3fa6ed2753288425b6c",
                    "36400926f9336d2d9fb59d23c42c3950",
                    "36339d50f9b539269f2c092dc4406d23",
                    "f4bcd45432e554d075f1d6c51dd03b3c",
                    "3caaa3e8a99f9deb50f3af57adf622aa",
                ),
                (
                    "c81677bc9b7ac93b25027992b0261996",
                    "e847f56514dadde23f77b64fe7f7d490",
                    "e8dab6901477d4653ff7f5e2e747dd4f",
                    "9816ee7400f87f556b2c049c8e5ad036",
                    "5e390f7df7a69296a7553dc10aa31f6b",
                ),
                (
                    "c62fe109f75eedc3cc79395d84f9cf5d",
                    "b415f8016858552e4bb6124c5f998a4c",
                    "b458124c68b68a014b99f82e5f15554c",
                    "c57e1c159a9bd286f05f4be098c63439",
                    "14f9701ae35fe28c440adf4d4ea9c026",
                ),
                (
                    "d1876c0f79c4300ab45594add66ff41f",
                    "3e175076b61c04678dfc2295f6a8bfc0",
                    "3e1c22c0b6fcbf768da85067f6170495",
                    "baa03de7a1f9b56ed5512cba5f414d23",
                    "47438735a41c65b9e016baf4aebf7ad2",
                ),
                (
                    "fde3bad205e5d0d73547964ef1fe37f1",
                    "5411f4b56bd9700e96a0902fa1bb9aa1",
                    "54d990a16ba09ab596bbf40ea111702f",
                    "e9f74eec023020f61bf2ccf2353c21c7",
                    "549932d1f08557681093ed9cbe2c974e",
                )
            ];


            let mut state = State::from(&plaintext);
            let kr_size = 4 * aes.block_size;

            assert_eq!(
                ByteVec::from_hex("000102030405060708090a0b0c0d0e0f"),
                round_keys[0..kr_size].to_vec(),
                "round[0] .k_sch"
            );

            state.add_round_key(&round_keys[0..kr_size]);

            for r in 1..aes.nb_rounds
            {
                assert_eq!(
                    ByteVec::from_hex(expected[r-1].0),
                    state.output(),
                    "round[{}] .start", r
                );

                state.sub_bytes();

                assert_eq!(
                    ByteVec::from_hex(expected[r-1].1),
                    state.output(),
                    "round[{}] .s_box", r
                );

                state.shift_rows();

                assert_eq!(
                    ByteVec::from_hex(expected[r-1].2),
                    state.output(),
                    "round[{}] .s_row", r
                );

                state.mix_columns();

                assert_eq!(
                    ByteVec::from_hex(expected[r-1].3),
                    state.output(),
                    "round[{}] .m_col", r
                );

                state.add_round_key(&round_keys[r*kr_size..(r+1)*kr_size]);

                assert_eq!(
                    ByteVec::from_hex(expected[r-1].4),
                    round_keys[r*kr_size..(r+1)*kr_size].to_vec(),
                    "round[{}] .k_sch", r
                );
            }

            assert_eq!(
                ByteVec::from_hex("bd6e7c3df2b5779e0b61216e8b10b689"),
                state.output(),
                "round[10] .start"
            );

            state.sub_bytes();

            assert_eq!(
                ByteVec::from_hex("7a9f102789d5f50b2beffd9f3dca4ea7"),
                state.output(),
                "round[10] .s_box"
            );

            state.shift_rows();

            assert_eq!(
                ByteVec::from_hex("7ad5fda789ef4e272bca100b3d9ff59f"),
                state.output(),
                "round[10] .s_row"
            );

            state.add_round_key(&round_keys[aes.nb_rounds*kr_size..(aes.nb_rounds+1)*kr_size]);

            assert_eq!(
                ByteVec::from_hex("13111d7fe3944a17f307a78b4d2b30c5"),
                &round_keys[aes.nb_rounds*kr_size..(aes.nb_rounds+1)*kr_size],
                "round[10] .k_sch"
            );

            assert_eq!(
                ByteVec::from_hex("69c4e0d86a7b0430d8cdb78070b4c55a"),
                state.output()
            );
        }
    }

    #[test]
    fn gf_mul()
    {
        assert_eq!(0x57, super::gf_mul(0x57, 0x01));
        assert_eq!(0xae, super::gf_mul(0x57, 0x02));
        assert_eq!(0x47, super::gf_mul(0x57, 0x04));
        assert_eq!(0x8e, super::gf_mul(0x57, 0x08));
        assert_eq!(0x07, super::gf_mul(0x57, 0x10));
        assert_eq!(0xfe, super::gf_mul(0x57, 0x13));
    }
}