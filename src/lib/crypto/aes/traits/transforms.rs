use crate::lib::types::Byte;

pub trait Transforms
{
    fn sub_bytes(&mut self);
    fn shift_rows(&mut self);
    fn mix_columns(&mut self);
    fn add_round_key(&mut self, round_key: &[Byte]); // TODO: change to word
}