use crate::lib::types::ByteVec;
use crate::lib::traits::BlockIterable;

use std::collections::HashMap;

pub fn detect_ecb(ciphertexts: Vec<ByteVec>, block_size: usize) -> Vec<(usize, ByteVec, f64)>
{
    let mut results = vec![];

    for (i, cipher) in ciphertexts.iter().enumerate()
    {
        let blocks = cipher.blocks(block_size);
        let mut set = HashMap::<ByteVec, u32>::new();

        for block in blocks
        {
            *set.entry(block).or_insert(0) += 1;
        }

        let score = 1. - (set.len() as f64 / set.values().sum::<u32>() as f64);

        results.push((i, cipher.to_vec(), score));
    }

    results.sort_by( |a, b| b.2.partial_cmp(&a.2).unwrap() );

    results
}