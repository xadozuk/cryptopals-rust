use crate::lib::types::ByteVec;
use crate::lib::crypto::aes::{Context, Key};

pub struct Message
{
    pub key:     Key,
    pub content: ByteVec,
    pub iv:      Option<ByteVec>
}

impl Message
{
    pub fn from(content: ByteVec, key: Key) -> Self
    {
        Message {
            content: content,
            key:     key,
            iv:      None
        }
    }

    pub fn with_iv(mut self, iv: ByteVec) -> Self
    {
        if iv.len() != 4 * Context::from_key(&self.key).block_size
        {
            panic!("Malformed IV");
        }

        self.iv = Some(iv);
        self
    }

    pub fn iv(&self) -> ByteVec
    {
        if self.iv == None
        {
            panic!("No IV specified");
        }

        self.iv.as_ref().unwrap().to_vec()
    }
}