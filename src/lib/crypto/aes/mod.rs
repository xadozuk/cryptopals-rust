pub mod consts;
pub mod traits;
pub mod cipher;

mod types;
mod enums;

pub use types::Key;
pub use types::Context;
pub use types::Message;

pub use enums::{AesType, AesMode};

use crate::lib::types::ByteVec;
use crate::lib::traits::BlockIterable;
use crate::lib::math::byte::xor;

pub fn encrypt(mode: AesMode, message: &Message) -> ByteVec
{
    let ctx = Context::from_key(&message.key);

    match mode
    {
        AesMode::ECB => encrypt_ecb(&ctx, message),
        AesMode::CBC => encrypt_cbc(&ctx, message)
    }
}

pub fn decrypt(mode: AesMode, message: &Message) -> ByteVec
{
    let ctx = Context::from_key(&message.key);

    match mode
    {
        AesMode::ECB => decrypt_ecb(&ctx, message),
        AesMode::CBC => decrypt_cbc(&ctx, message)
    }
}

fn encrypt_ecb(ctx: &Context, message: &Message) -> ByteVec
{
    let mut result = vec![];

    for block in message.content.blocks(ctx.block_size * 4).with_padding()
    {
        result.extend(cipher::encrypt(ctx, &block, &message.key));
    }

    result
}

fn encrypt_cbc(ctx: &Context, message: &Message) -> ByteVec
{
    let mut result = vec![];
    let mut iv = message.iv();

    for block in message.content.blocks(ctx.block_size * 4).with_padding()
    {
        let xored_block = xor(&block, &iv);
        let cipher = cipher::encrypt(ctx, &xored_block, &message.key);

        iv = cipher.to_vec();

        result.extend(cipher);
    }

    result
}

fn decrypt_ecb(ctx: &Context, message: &Message) -> ByteVec
{
    let mut result = vec![];

    for block in message.content.blocks(ctx.block_size * 4)
    {
        result.extend(cipher::decrypt(ctx, &block, &message.key));
    }

    result
}

fn decrypt_cbc(ctx: &Context, message: &Message) -> ByteVec
{
    let mut result = vec![];
    let mut iv = message.iv();

    for block in message.content.blocks(ctx.block_size * 4)
    {
        let unciphered = cipher::decrypt(ctx, &block, &message.key);
        let plain = xor(&iv, &unciphered);

        iv = block;

        result.extend(plain);
    }

    result
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::lib::traits::FromHex;

    #[test]
    fn cipher_cbc()
    {
        let ctx = Context::new(AesType::Aes128);
        let msg = Message::from(
                            ByteVec::from_hex("00112233445566778899AABBCCDDEEFF"),
                            Key::new(&ByteVec::from_hex("000102030405060708090A0B0C0D0E0F"))
                        )
                        .with_iv(vec![0x0; 16]);

        let expected = ByteVec::from_hex("69c4e0d86a7b0430d8cdb78070b4c55a");

        assert_eq!(
            expected,
            super::encrypt_cbc(&ctx, &msg)
        );
    }
}