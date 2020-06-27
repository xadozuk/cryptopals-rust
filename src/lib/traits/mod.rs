use std::string::FromUtf8Error;

use crate::lib::iterators::BlockIterator;

pub trait Bitable
{
    fn bit(&self, n: usize) -> u8;
    fn bits(&self) -> Vec<u8>;
}

pub trait FromHex
{
    fn from_hex(string: &str) -> Self;
}

pub trait ToHex
{
    fn to_hex(&self) -> String;
}

pub trait FromBase64
{
    fn from_base64(string: &str) -> Self;
}

pub trait ToBase64
{
    fn to_base64(&self) -> String;
}

pub trait ToString
{
    fn to_string(&self) -> Result<String, FromUtf8Error>;
}

pub trait BlockIterable
{
    fn blocks(&self, block_size: usize) -> BlockIterator;
}