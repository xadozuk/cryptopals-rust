pub mod base64;

mod iterators;

use std::string::FromUtf8Error;
use crate::lib::traits::{FromHex, Bitable, ToString, ToHex, Random};

use rand::Rng;

pub type ByteVec = Vec<u8>;

impl Bitable for ByteVec
{
    fn bit(&self, n: usize) -> u8
    { 
        if n >= self.len() * 8 { panic!("Out of range"); }

        self[n / 8].bit(n % 8)
    }

    fn bits(&self) -> Vec<u8>
    { 
        let mut bits = vec![];

        for byte in self
        {
            bits.extend(byte.bits());
        }

        return bits;
    }
}

impl Random for ByteVec
{
    fn random(size: usize) -> Self
    { 
        let mut rng = rand::thread_rng();

        (0..size)
            .map( |_| rng.gen::<u8>() )
            .collect()
    }
}

impl FromHex for ByteVec
{
    fn from_hex(string: &str) -> Self
    {
        // TODO: use Result
        if string.len() % 2 != 0
        {
            panic!("Invalid hex string (split byte)");
        }

        (0..string.len())
            .step_by(2)
            .map( |i| u8::from_str_radix(&string[i..i + 2], 16).unwrap() )
            .collect()
    }
}

impl ToHex for ByteVec
{
    fn to_hex(&self) -> String
    {
        self.iter()
            .map( |b| format!("{:0>2x}", b) )
            .collect()
    }
}

impl ToString for ByteVec
{
    fn to_string(&self) -> Result<String, FromUtf8Error>
    { 
        String::from_utf8(self.clone())
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn from_str()
    {
        let bytes = ByteVec::from("Test");
        let actual: Vec<u8> = bytes.into();

        assert_eq!(vec![0x54, 0x65, 0x73, 0x74], actual);
    }

    #[test]
    fn from_hex()
    {
        let bytes = ByteVec::from_hex("0123456789abcdef");
        let actual: Vec<u8> = bytes.into();

        assert_eq!(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef], actual);
    }

    #[test]
    fn bits()
    {
        assert_eq!(
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            ByteVec::from_hex("ff").bits()
        );

        assert_eq!(
            vec![0, 0, 0, 1, 0, 0, 0, 0],
            ByteVec::from_hex("10").bits()
        );
    }
}