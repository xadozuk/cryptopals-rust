use std::fmt;

pub enum AesType
{
    Aes128,
    Aes192,
    Aes256
}

#[derive(PartialEq)]
pub enum AesMode
{
    ECB,
    CBC
}

impl fmt::Display for AesMode
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    { 
        match self
        {
            Self::ECB => write!(f, "ECB"),
            Self::CBC => write!(f, "CBC")
        }
    }
}