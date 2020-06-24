use std::iter;
use std::cmp;

pub struct BlockIterator<T>
    where T: Clone
{
    values:     Vec<T>,
    block_size: usize,
    index:      usize
}

impl<T> BlockIterator<T>
    where T: Clone
{
    pub fn new(values: &[T], block_size: usize) -> Self
    {
        BlockIterator {
            values:     values.to_vec(),
            block_size: block_size,
            index:      0
        }
    }
}

impl<T> iter::Iterator for BlockIterator<T>
    where T: Clone
{
    type Item = Vec<T>;

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
}