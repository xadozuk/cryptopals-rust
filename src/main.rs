#![allow(unused_variables)]
#![allow(dead_code)]

mod challenges;
mod lib;
mod attacks;

fn main()
{
    //challenges::challenge3();
    //challenges::challenge4();
    //challenges::challenge6();

    println!("{:0>8b} << 1 = {:0>8b}", 0x1, 0x1u8.overflowing_shl(1).0);

    let v = 0b11111111u8.overflowing_shl(1);
    println!("{:0>8b} << 1 = {:0>8b} [{}]", 0b11111111, v.0, v.1);
}

