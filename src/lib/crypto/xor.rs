use crate::lib::types::ByteVec;
use crate::lib::math::byte::xor;

pub fn fixed(a: &ByteVec, b: &ByteVec) -> ByteVec
{
    xor(a, b)
}

pub fn repeating_key(text: &ByteVec, key: &ByteVec) -> ByteVec
{
    if key.len() == 0 { panic!("Key must not by empty"); }

    let expanded_key: ByteVec = (0..text.len())
        .map( |i| key[i % key.len()] )
        .collect();


    fixed(text, &expanded_key)
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::lib::traits::FromHex;

    #[test]
    fn fixed()
    {
        assert_eq!(
            ByteVec::from_hex("072a2b2e287922352a3e28"),
            super::fixed(&ByteVec::from("Lorem ipsum"), &ByteVec::from("KEYKEYKEYKE"))
        );
    }

    #[test]
    fn repeating_key()
    {
        assert_eq!(
            ByteVec::from_hex("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20"),
            super::repeating_key(&ByteVec::from("Burning 'em, if you ain't quick and nimble"), &ByteVec::from("ICE"))
        );

        assert_eq!(
            ByteVec::from_hex("0063222663263b223f30633221262b690a652126243b632469203c24212425"),
            super::repeating_key(&ByteVec::from("I go crazy when I hear a cymbal"), &ByteVec::from("ICE"))
        );
    }
}