//use cryptopals::{hex_to_base64, hex_to_bytes};
/*
#[test]
fn test_hex_to_bytes()
{
    assert_eq!(
        vec![0xAA, 0x00, 0xFF], 
        hex_to_bytes(String::from("AA00FF"))
    );

    assert_eq!(
        vec![0x49, 0x27, 0x6d, 0x20, 0x6b, 0x69, 0x6c],
        hex_to_bytes(String::from("49276d206b696c"))
    )
}

#[test]
fn test_hex_to_base64()
{
    let hex_string = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let expected_b64 = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

    assert_eq!(expected_b64, hex_to_base64(hex_string));

    assert_eq!(
        String::from("TWFu"),
        hex_to_base64(String::from("4D616E"))
    );

    assert_eq!(
        String::from("TWE="),
        hex_to_base64(String::from("4D61"))
    );

    assert_eq!(
        String::from("TQ=="),
        hex_to_base64(String::from("4D"))
    );
}

#[test]
fn test_fixed_xor()
{
    assert_eq!(
        String::from("746865206b696420646f6e277420706c6179"),
        cryptopals::fixed_xor(
            String::from("1c0111001f010100061a024b53535009181c"),
            String::from("686974207468652062756c6c277320657965")
        )
    )
}
*/