use crate::lib::byte::ByteVec;

pub fn fixed(bytes1: &ByteVec, bytes2: &ByteVec) -> ByteVec
{
    if bytes1.len() != bytes2.len() { panic!("ByteVec must be of the same length ({}|{})", bytes1.len(), bytes2.len()) }

    let mut result = ByteVec::new();

    for i in 0..bytes1.len()
    {
        let xored_value = bytes1[i] ^ bytes2[i];
        result.push(xored_value);
    }

    return result;
}

pub fn repeating_key(text: &ByteVec, key: &ByteVec) -> ByteVec
{
    if key.len() == 0 { panic!("Key must not by empty"); }

    let mut result = ByteVec::new();

    for i in 0..text.len()
    {
        result.push(
            text[i] ^ key[i % key.len()]
        );
    }

    return result;
}

#[cfg(test)]
mod tests
{
    use super::*;

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