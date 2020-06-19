use crate::lib::types::ByteVec;

pub trait KeyExpansion
{
    fn expand(&self) -> ByteVec;
}