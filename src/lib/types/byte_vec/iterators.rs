use crate::lib::traits::BlockIterable;
use crate::lib::types::ByteVec;
use crate::lib::iterators::BlockIterator;

impl BlockIterable for ByteVec
{
    fn blocks(&self, block_size: usize) -> BlockIterator
    { 
        BlockIterator::new(self, block_size)
    }
}