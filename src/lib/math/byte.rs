use crate::lib::types::{Byte, ByteVec};

pub fn xor(a: &[Byte], b: &[Byte]) -> ByteVec
{
    if a.len() != b.len() { panic!("a(len={}) and b(len={}) must be of the same size.", a.len(), b.len()) }

    a.iter().zip(b)
        .map( |(a, b)| a ^ b )
        .collect()
}