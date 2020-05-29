use crate::lib::byte::ByteVec;

pub fn fixed_xor(bytes1: &ByteVec, bytes2: &ByteVec) -> ByteVec
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

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn fixed_xor()
    {
        assert_eq!(
            ByteVec::from_hex("072a2b2e287922352a3e28"),
            super::fixed_xor(&ByteVec::from("Lorem ipsum"), &ByteVec::from("KEYKEYKEYKE"))
        );
    }
}