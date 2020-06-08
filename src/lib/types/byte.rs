use crate::lib::traits::Bitable;

pub type Byte = u8;

impl Bitable for Byte
{
    fn bit(&self, n: usize) -> u8
    { 
        if n > 7 { panic!("Out of range"); }

        // Use Big-Endian for simplicity
        ((self.reverse_bits() & 2u8.pow(n as u32)) >> n) as u8
    }

    fn bits(&self) -> Vec<u8>
    { 
        let mut bits = vec![];

        for i in 0..8
        {
            bits.push(self.bit(i));
        }

        return bits;
    }
}

#[cfg(test)]
mod tests
{
    use super::Bitable;

    #[test]
    fn bit()
    {
        // 0x01 = 0b00000001
        let byte = 0x01;

        assert_eq!(0, byte.bit(0));
        assert_eq!(0, byte.bit(1));
        assert_eq!(0, byte.bit(2));
        assert_eq!(0, byte.bit(3));
        assert_eq!(0, byte.bit(4));
        assert_eq!(0, byte.bit(5));
        assert_eq!(0, byte.bit(6));
        assert_eq!(1, byte.bit(7));
    }

    #[test]
    #[should_panic(expected = "Out of range")]
    fn bit_overflow()
    {
        0x0.bit(8);
    }
}