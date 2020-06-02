pub fn gcd(a: u32, b: u32) -> u32
{
    let mut t: u32;
    let mut a = a;
    let mut b = b;

    while b != 0
    {
        t = b;
        b = a % b;
        a = t;
    }

    return a;
}

#[cfg(test)]
mod tests
{
    #[test]
    fn gcd()
    {
        assert_eq!(1, super::gcd(45, 52));
        assert_eq!(1, super::gcd(52, 45));

        assert_eq!(4, super::gcd(20, 16));
    }
}