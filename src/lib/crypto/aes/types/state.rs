use crate::lib::types::{Byte, ByteVec, Matrix};
use crate::lib::crypto::aes::traits::Ops;
use crate::lib::crypto::aes::consts::{SBOX, SBOX_INV};
use crate::lib::math::gf2_8::mul;

use std::fmt;

pub struct State
{
    bytes: Matrix<Byte>
}

impl State
{
    const ROW_COUNT: usize = 4;

    fn new(block_size: usize) -> Self
    {
        State {
            bytes: Matrix::<Byte>::new(Self::ROW_COUNT, block_size)
        }
    }

    pub fn from(block_size: usize, input: &[Byte]) -> Self
    {
        let mut state = Self::new(block_size);

        state.assert_size(input);

        for r in 0..Self::ROW_COUNT
        {
            for c in 0..state.block_size()
            {
                state.bytes[r][c] = input[r + 4 * c]
            }
        }

        state
    }

    pub fn output(&self) -> ByteVec
    {
        let mut output = vec![0x0; 16];

        for r in 0..Self::ROW_COUNT
        {
            for c in 0..self.block_size()
            {
                output[r + 4 * c] = self.bytes[r][c];
            }
        }

        return output;
    }

    fn assert_size(&self, block: &[Byte])
    {
        if block.len() != Self::ROW_COUNT * self.bytes.height()
        {
            panic!("Malformed byte-array {} != {}", block.len(), Self::ROW_COUNT * self.bytes.height());
        }
    }

    fn block_size(&self) -> usize
    {
        self.bytes.width()
    }

    fn shift_row(&mut self, row: usize, shift: usize)
    {
        self.bytes[row] = (0..Self::ROW_COUNT)
            .map( |i| self.bytes[row][(shift + i) % self.block_size()])
            .collect();
    }

    fn inv_shift_row(&mut self, row: usize, shift: usize)
    {
        self.bytes[row] = (0..Self::ROW_COUNT)
            .map( |i| self.bytes[row][(self.block_size() - shift + i) % self.block_size()])
            .collect();
    }
}

impl Ops for State
{
    fn add_round_key(&mut self, round_key: &[Byte])
    {
        self.assert_size(round_key);

        for c in 0..self.block_size()
        {
            for r in 0..Self::ROW_COUNT
            {
                self.bytes[r][c] ^= round_key[c * 4 + r]
            }
        }
    }

    fn sub_bytes(&mut self)
    {
        for r in 0..Self::ROW_COUNT
        {
            for c in 0..self.block_size()
            {
                self.bytes[r][c] = SBOX[self.bytes[r][c] as usize];
            }
        }
    }

    fn shift_rows(&mut self)
    {
        for i in 1..Self::ROW_COUNT
        {
            self.shift_row(i, i);
        }
    }

    fn mix_columns(&mut self)
    {
        let state = self.bytes.clone();

        for c in 0..self.block_size()
        {
            self.bytes[0][c] = mul(0x2, state[0][c]) ^ mul(0x3, state[1][c]) ^ state[2][c] ^ state[3][c];
            self.bytes[1][c] = state[0][c] ^ mul(0x2, state[1][c]) ^ mul(0x3, state[2][c]) ^ state[3][c];
            self.bytes[2][c] = state[0][c] ^ state[1][c] ^ mul(0x2, state[2][c]) ^ mul(0x3, state[3][c]);
            self.bytes[3][c] = mul(0x3, state[0][c]) ^ state[1][c] ^ state[2][c] ^ mul(0x2, state[3][c]);
        }
    }

    fn inv_sub_bytes(&mut self)
    {
        for r in 0..Self::ROW_COUNT
        {
            for c in 0..self.block_size()
            {
                self.bytes[r][c] = SBOX_INV[self.bytes[r][c] as usize];
            }
        }
    }

    fn inv_shift_rows(&mut self)
    {
        for i in 1..Self::ROW_COUNT
        {
            self.inv_shift_row(i, i);
        }
    }

    fn inv_mix_columns(&mut self)
    {
        let state = self.bytes.clone();

        for c in 0..self.block_size()
        {
            self.bytes[0][c] = mul(0xe, state[0][c]) ^ mul(0xb, state[1][c]) ^ mul(0xd, state[2][c]) ^ mul(0x9, state[3][c]);
            self.bytes[1][c] = mul(0x9, state[0][c]) ^ mul(0xe, state[1][c]) ^ mul(0xb, state[2][c]) ^ mul(0xd, state[3][c]);
            self.bytes[2][c] = mul(0xd, state[0][c]) ^ mul(0x9, state[1][c]) ^ mul(0xe, state[2][c]) ^ mul(0xb, state[3][c]);
            self.bytes[3][c] = mul(0xb, state[0][c]) ^ mul(0xd, state[1][c]) ^ mul(0x9, state[2][c]) ^ mul(0xe, state[3][c]);
        }
    }
}

impl fmt::Debug for State
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    { 
        writeln!(f, "{:?}", self.bytes)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn from()
    {
        let s = State::from(
            4,
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
        let s = State::from(4,
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

    // TODO: test transforms
    #[test]
    fn inv_shift_rows()
    {
        let mut s = State {
            bytes: Matrix::<Byte>::from(&vec![
                vec![0x0, 0x1, 0x2, 0x3],
                vec![0x4, 0x5, 0x6, 0x7],
                vec![0x8, 0x9, 0xA, 0xB],
                vec![0xC, 0xD, 0xE, 0xF],
            ])
        };

        s.inv_shift_rows();

        assert_eq!(
            Matrix::<Byte>::from(&vec![
                vec![0x0, 0x1, 0x2, 0x3],
                vec![0x7, 0x4, 0x5, 0x6],
                vec![0xA, 0xB, 0x8, 0x9],
                vec![0xD, 0xE, 0xF, 0xC]
            ]),
            s.bytes
        );
    }
}