use crate::lib::types::ByteVec;
use crate::lib::traits::BlockIterable;

use std::collections::HashMap;

pub fn detect_ecb(ciphertext: &ByteVec, block_size: usize) -> f64
{
    let blocks = ciphertext.blocks(block_size);
    let mut set = HashMap::<ByteVec, u32>::new();

    for block in blocks
    {
        *set.entry(block).or_insert(0) += 1;
    }

    1. - (set.len() as f64 / set.values().sum::<u32>() as f64)
}