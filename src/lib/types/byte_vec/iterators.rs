use crate::lib::traits::BlockIterable;
use crate::lib::types::{Byte, ByteVec};
use crate::lib::iterators::BlockIterator;

impl BlockIterable<Byte> for ByteVec
{
    fn blocks(&self, block_size: usize) -> BlockIterator<Byte>
    { 
        BlockIterator::new(self, block_size)
    }
}