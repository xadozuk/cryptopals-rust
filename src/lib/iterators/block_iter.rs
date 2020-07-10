use crate::lib::types::{Byte, ByteVec};

use std::iter;
use std::cmp;

pub struct BlockIterator
{
    values:     ByteVec,
    block_size: usize,
    index:      usize,
    padding:    bool
}

impl BlockIterator
{
    pub fn new(values: &[Byte], block_size: usize) -> Self
    {
        BlockIterator {
            values:     values.to_vec(),
            block_size: block_size,
            index:      0,
            padding:    false
        }
    }

    pub fn with_padding(mut self) -> Self
    {
        if self.block_size > 32 { panic!("PCKS#7 does not support with blocks greater than 32 bytes"); }

        self.padding = true;

        let n_missing = self.block_size - (self.values.len() % self.block_size);
        self.values.extend(vec![n_missing as u8; n_missing]);

        self
    }

    pub fn remove_padding(self) -> ByteVec
    {
        if self.block_size > 32 { panic!("PCKS#7 does not support with blocks greater than 32 bytes"); }

        let padding_size = *self.values.last().unwrap_or(&0) as usize;
        let end = self.values.len().checked_sub(padding_size);

        match end 
        {
            None => panic!("Invalid padding."),
            Some(e) => self.values[..e].to_vec()
        }
    }
}

impl iter::Iterator for BlockIterator
{
    type Item = ByteVec;

    fn next(&mut self) -> Option<Self::Item>
    { 
        if self.index >= self.values.len()
        {
            return None;
        }

        let start = self.index;
        let end   = cmp::min(start + self.block_size, self.values.len());

        self.index = end;

        Some(self.values[start..end].to_vec())
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn next()
    {
        let bytes = vec![
            0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7,
            0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
            0x0
        ];

        let mut iter = BlockIterator::new(&bytes, 8);

        assert_eq!(
            Some(vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7]),
            iter.next()
        );

        assert_eq!(
            Some(vec![0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF]),
            iter.next()
        );

        assert_eq!(
            Some(vec![0x0]),
            iter.next()
        );

        assert_eq!(None, iter.next());
    }

    #[test]
    fn with_padding()
    {
        let bytes = vec![
            0x0, 0x1, 0x2, 0x3,
            0x4
        ];

        let mut iter = BlockIterator::new(&bytes, 4).with_padding();

        assert_eq!(
            Some(vec![0x0, 0x1, 0x2, 0x3]),
            iter.next()
        );

        assert_eq!(
            Some(vec![0x4, 0x3, 0x3, 0x3]),
            iter.next()
        );

        assert_eq!(None, iter.next());
    }

    #[test]
    fn with_padding_on_full_block()
    {
        let bytes = vec![
            0x0, 0x1, 0x2, 0x3,
            0x4, 0x5, 0x6, 0x7
        ];

        let mut iter = BlockIterator::new(&bytes, 4).with_padding();

        iter.next();
        iter.next();

        assert_eq!(
            Some(vec![0x4, 0x4, 0x4, 0x4]),
            iter.next()
        );
    }
}