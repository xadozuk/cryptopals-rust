use crate::lib::types::{Byte, ByteVec, Matrix};
use crate::lib::crypto::aes::traits::Transforms;
use crate::lib::crypto::aes::consts::SBOX;
use crate::lib::math::gf2_8::mul;

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
        self.bytes[row] = (0..self.block_size())
            .map( |i| self.bytes[row][(shift + i) % self.block_size()])
            .collect();
    }
}

impl Transforms for State
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
}

#[cfg(test)]
mod tests
{
    use super::State;
    use crate::lib::types::Matrix;
    use crate::lib::types::Byte;

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
}