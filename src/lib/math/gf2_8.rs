use crate::lib::types::Byte;

pub fn mul(a: Byte, b: Byte) -> Byte
{
    let mut a16 = a as u16;
    let mut b16 = b as u16;
    let mut result = 0;

    for c in 0..8
    {
        if b16 & 0x1 == 0x1
        {
            result ^= a16;
        }

        a16 <<= 1;

        if a16 & 0x100 == 0x100
        {
            a16 ^= 0x11B;
        }

        b16 >>= 1;
    }

    result as Byte
}

#[cfg(test)]
mod tests
{
    use super::mul;

    #[test]
    fn gf_mul()
    {
        assert_eq!(0x57, mul(0x57, 0x01));
        assert_eq!(0xae, mul(0x57, 0x02));
        assert_eq!(0x47, mul(0x57, 0x04));
        assert_eq!(0x8e, mul(0x57, 0x08));
        assert_eq!(0x07, mul(0x57, 0x10));
        assert_eq!(0xfe, mul(0x57, 0x13));
    }
}