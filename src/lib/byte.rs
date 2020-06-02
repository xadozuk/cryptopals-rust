use std::ops::Range;
use std::string::FromUtf8Error;
use std::ops::Index;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ByteVec
{
    bytes: Vec<u8>
}

impl From<&str> for ByteVec
{
    fn from(string: &str) -> Self
    { 
        ByteVec { bytes: string.as_bytes().to_vec() }
    }
}

impl From<String> for ByteVec
{
    fn from(string: String) -> Self
    {
        ByteVec { bytes: string.as_bytes().to_vec() }
    }
}

impl Into<Vec<u8>> for ByteVec
{
    fn into(self) -> Vec<u8>
    { 
        self.bytes
    }
}

impl Index<usize> for ByteVec
{
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output 
    { 
        &self.bytes[index]
    }
}

impl ByteVec
{
    pub fn from_hex(string: &str) -> Self
    {
        // TODO: use Result
        if string.len() % 2 != 0
        {
            panic!("Invalid hex string (split byte)");
        }

        let vec = (0..string.len())
            .step_by(2)
            .map( |i| u8::from_str_radix(&string[i..i + 2], 16).unwrap())
            .collect();

        ByteVec { bytes: vec }
    }

    pub fn new() -> Self
    {
        ByteVec { bytes: Vec::<u8>::new() }
    }

    pub fn len(&self) -> usize
    {
        self.bytes.len()
    }

    pub fn push(&mut self, value: u8)
    {
        self.bytes.push(value);
    }

    pub fn slice(&self, index: Range<usize>) -> ByteVec
    {
        ByteVec { bytes: self.bytes[index].to_vec() }
    }

    pub fn to_hex(&self) -> String
    {
        self.bytes
            .iter()
            .map( |b| format!("{:0>2x}", b) )
            .collect()
    }

    pub fn bits(&self) -> Vec<u8>
    {
        let mut bits: Vec<u8> = vec![];

        for byte in &self.bytes
        {
            bits.push((byte & 0b10000000) >> 7);
            bits.push((byte & 0b01000000) >> 6);
            bits.push((byte & 0b00100000) >> 5);
            bits.push((byte & 0b00010000) >> 4);
            bits.push((byte & 0b00001000) >> 3);
            bits.push((byte & 0b00000100) >> 2);
            bits.push((byte & 0b00000010) >> 1);
            bits.push( byte & 0b00000001);
        }

        return bits;
    }

    pub fn get(&self, index: usize) -> Option<&u8>
    {
        self.bytes.get(index)
    }

    pub fn to_string(&self) -> Result<String, FromUtf8Error>
    {
        String::from_utf8(self.bytes.clone())
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