pub trait FromHex
{
    fn from_hex(string: &str) -> Self;
}

pub trait Bitable
{
    fn bit(&self, n: usize) -> u8;
    fn bits(&self) -> Vec<u8>;
}