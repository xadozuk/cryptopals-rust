use crate::lib::crypto::aes::enums::AesType;
use crate::lib::crypto::aes::Key;

// TODO: should protect fields private/getter ?
pub struct Context
{
    pub key_length: usize,
    pub block_size: usize,
    pub nb_rounds: usize,
}

impl Context
{
    pub fn new(t: AesType) -> Self
    {
        match t
        {
            AesType::Aes128 => Context { key_length: 4, block_size: 4, nb_rounds: 10 },
            AesType::Aes192 => Context { key_length: 6, block_size: 4, nb_rounds: 12 },
            AesType::Aes256 => Context { key_length: 8, block_size: 4, nb_rounds: 14 }
        }
    }

    pub fn from_key(key: &Key) -> Self
    {
        match key.length()
        {
            4 => Self::new(AesType::Aes128),
            6 => Self::new(AesType::Aes192),
            8 => Self::new(AesType::Aes256),
            _ => panic!("Malformed key")
        }
    }
}