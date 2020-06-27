use crate::lib::types::{Byte, ByteVec};
use crate::lib::traits::BlockIterable;

use super::types::{State, Context, Key};
use super::traits::{Ops, KeyExpansion};

pub fn encrypt(ctx: &Context, input: &ByteVec, key: &Key) -> ByteVec
{
    let mut state = State::from(ctx.block_size, input);
    let kr_size = 4 * ctx.block_size;

    let round_keys: Vec<ByteVec> = key.expand()
        // Break expanded key into 16 bytes blocks
        .blocks(4 * ctx.block_size)
        .collect();

    state.add_round_key(&round_keys[0]);

    for r in 1..ctx.nb_rounds
    {
        state.sub_bytes();
        state.shift_rows();
        state.mix_columns();

        state.add_round_key(&round_keys[r]);
    }

    state.sub_bytes();
    state.shift_rows();
    state.add_round_key(&round_keys[ctx.nb_rounds]);

    state.output()
}

pub fn decrypt(ctx: &Context, input: &[Byte], key: &Key) -> ByteVec
{
    let mut state = State::from(ctx.block_size, input);

    let round_keys: Vec<ByteVec> = key.expand()
        // Break expanded key into 16 bytes blocks
        .blocks(4 * ctx.block_size)
        .collect();

    state.add_round_key(&round_keys[ctx.nb_rounds]);

    for r in (1..ctx.nb_rounds).rev()
    {
        state.inv_shift_rows();
        state.inv_sub_bytes();
        state.add_round_key(&round_keys[r]);
        state.inv_mix_columns();
    }

    state.inv_shift_rows();
    state.inv_sub_bytes();
    state.add_round_key(&round_keys[0]);

    state.output()
}

#[cfg(test)]
mod tests
{
    use super::*;
    use super::super::enums::AesType;
    use crate::lib::types::ByteVec;
    use crate::lib::traits::FromHex;

    fn ctx() -> Context
    {
        Context::new(AesType::Aes128)
    }

    #[test]
    fn cipher()
    {
        let ctx = ctx();
        let key = Key::new(&ByteVec::from("YELLOW SUBMARINE"));

        let ciphertext = encrypt(&ctx, &ByteVec::from("Lorem ipsum dolo"), &key);

        assert_eq!(
            ByteVec::from_hex("eadcc5aa4800dff175a49cf3a0f2041d"),
            ciphertext
        )
    }

    #[test]
    fn uncipher()
    {
        let ctx = ctx();
        let key = Key::new(&ByteVec::from_hex("000102030405060708090a0b0c0d0e0f"));

        let plaintext = decrypt(&ctx, &ByteVec::from_hex("69c4e0d86a7b0430d8cdb78070b4c55a"), &key);

        assert_eq!(
            ByteVec::from_hex("00112233445566778899aabbccddeeff"),
            plaintext
        )
    }

    #[test]
    fn cipher_vector_test()
    {
        let ctx = ctx();

        let plaintext = ByteVec::from_hex("00112233445566778899aabbccddeeff");
        let key = Key::new(&ByteVec::from_hex("000102030405060708090a0b0c0d0e0f"));

        let round_keys: Vec<ByteVec> = key.expand()
            .blocks(4 * ctx.block_size)
            .collect();

        let expected = vec![
            (
                "00102030405060708090a0b0c0d0e0f0",
                "63cab7040953d051cd60e0e7ba70e18c",
                "6353e08c0960e104cd70b751bacad0e7",
                "5f72641557f5bc92f7be3b291db9f91a",
                "d6aa74fdd2af72fadaa678f1d6ab76fe"
            ),
            (
                "89d810e8855ace682d1843d8cb128fe4",
                "a761ca9b97be8b45d8ad1a611fc97369",
                "a7be1a6997ad739bd8c9ca451f618b61",
                "ff87968431d86a51645151fa773ad009",
                "b692cf0b643dbdf1be9bc5006830b3fe",
            ),
            (
                "4915598f55e5d7a0daca94fa1f0a63f7",
                "3b59cb73fcd90ee05774222dc067fb68",
                "3bd92268fc74fb735767cbe0c0590e2d",
                "4c9c1e66f771f0762c3f868e534df256",
                "b6ff744ed2c2c9bf6c590cbf0469bf41",
            ),
            (
                "fa636a2825b339c940668a3157244d17",
                "2dfb02343f6d12dd09337ec75b36e3f0",
                "2d6d7ef03f33e334093602dd5bfb12c7",
                "6385b79ffc538df997be478e7547d691",
                "47f7f7bc95353e03f96c32bcfd058dfd",
            ),
            (
                "247240236966b3fa6ed2753288425b6c",
                "36400926f9336d2d9fb59d23c42c3950",
                "36339d50f9b539269f2c092dc4406d23",
                "f4bcd45432e554d075f1d6c51dd03b3c",
                "3caaa3e8a99f9deb50f3af57adf622aa",
            ),
            (
                "c81677bc9b7ac93b25027992b0261996",
                "e847f56514dadde23f77b64fe7f7d490",
                "e8dab6901477d4653ff7f5e2e747dd4f",
                "9816ee7400f87f556b2c049c8e5ad036",
                "5e390f7df7a69296a7553dc10aa31f6b",
            ),
            (
                "c62fe109f75eedc3cc79395d84f9cf5d",
                "b415f8016858552e4bb6124c5f998a4c",
                "b458124c68b68a014b99f82e5f15554c",
                "c57e1c159a9bd286f05f4be098c63439",
                "14f9701ae35fe28c440adf4d4ea9c026",
            ),
            (
                "d1876c0f79c4300ab45594add66ff41f",
                "3e175076b61c04678dfc2295f6a8bfc0",
                "3e1c22c0b6fcbf768da85067f6170495",
                "baa03de7a1f9b56ed5512cba5f414d23",
                "47438735a41c65b9e016baf4aebf7ad2",
            ),
            (
                "fde3bad205e5d0d73547964ef1fe37f1",
                "5411f4b56bd9700e96a0902fa1bb9aa1",
                "54d990a16ba09ab596bbf40ea111702f",
                "e9f74eec023020f61bf2ccf2353c21c7",
                "549932d1f08557681093ed9cbe2c974e",
            )
        ];


        let mut state = State::from(ctx.block_size, &plaintext);

        assert_eq!(
            ByteVec::from_hex("000102030405060708090a0b0c0d0e0f"),
            round_keys[0].to_vec(),
            "round[0] .k_sch"
        );

        state.add_round_key(&round_keys[0]);

        for r in 1..ctx.nb_rounds
        {
            assert_eq!(
                ByteVec::from_hex(expected[r-1].0),
                state.output(),
                "round[{}] .start", r
            );

            state.sub_bytes();

            assert_eq!(
                ByteVec::from_hex(expected[r-1].1),
                state.output(),
                "round[{}] .s_box", r
            );

            state.shift_rows();

            assert_eq!(
                ByteVec::from_hex(expected[r-1].2),
                state.output(),
                "round[{}] .s_row", r
            );

            state.mix_columns();

            assert_eq!(
                ByteVec::from_hex(expected[r-1].3),
                state.output(),
                "round[{}] .m_col", r
            );

            state.add_round_key(&round_keys[r]);

            assert_eq!(
                ByteVec::from_hex(expected[r-1].4),
                round_keys[r].to_vec(),
                "round[{}] .k_sch", r
            );
        }

        assert_eq!(
            ByteVec::from_hex("bd6e7c3df2b5779e0b61216e8b10b689"),
            state.output(),
            "round[10] .start"
        );

        state.sub_bytes();

        assert_eq!(
            ByteVec::from_hex("7a9f102789d5f50b2beffd9f3dca4ea7"),
            state.output(),
            "round[10] .s_box"
        );

        state.shift_rows();

        assert_eq!(
            ByteVec::from_hex("7ad5fda789ef4e272bca100b3d9ff59f"),
            state.output(),
            "round[10] .s_row"
        );

        state.add_round_key(&round_keys[ctx.nb_rounds]);

        assert_eq!(
            ByteVec::from_hex("13111d7fe3944a17f307a78b4d2b30c5"),
            round_keys[ctx.nb_rounds],
            "round[10] .k_sch"
        );

        assert_eq!(
            ByteVec::from_hex("69c4e0d86a7b0430d8cdb78070b4c55a"),
            state.output()
        );
    }

    #[test]
    fn inv_cipher_vector_test()
    {
        let ctx = ctx();

        let ciphertext = ByteVec::from_hex("69c4e0d86a7b0430d8cdb78070b4c55a");
        let key = Key::new(&ByteVec::from_hex("000102030405060708090a0b0c0d0e0f"));

        let round_keys: Vec<ByteVec> = key.expand()
            .blocks(4 * ctx.block_size)
            .collect();

        let mut expected = vec![
            (
                "7ad5fda789ef4e272bca100b3d9ff59f",
                "7a9f102789d5f50b2beffd9f3dca4ea7",
                "bd6e7c3df2b5779e0b61216e8b10b689",
                "549932d1f08557681093ed9cbe2c974e",
                "e9f74eec023020f61bf2ccf2353c21c7"
            ),
            (
                "54d990a16ba09ab596bbf40ea111702f",
                "5411f4b56bd9700e96a0902fa1bb9aa1",
                "fde3bad205e5d0d73547964ef1fe37f1",
                "47438735a41c65b9e016baf4aebf7ad2",
                "baa03de7a1f9b56ed5512cba5f414d23"
            ),
            (
                "3e1c22c0b6fcbf768da85067f6170495",
                "3e175076b61c04678dfc2295f6a8bfc0",
                "d1876c0f79c4300ab45594add66ff41f",
                "14f9701ae35fe28c440adf4d4ea9c026",
                "c57e1c159a9bd286f05f4be098c63439"
            ),
            (
                "b458124c68b68a014b99f82e5f15554c",
                "b415f8016858552e4bb6124c5f998a4c",
                "c62fe109f75eedc3cc79395d84f9cf5d",
                "5e390f7df7a69296a7553dc10aa31f6b",
                "9816ee7400f87f556b2c049c8e5ad036"
            ),
            (
                "e8dab6901477d4653ff7f5e2e747dd4f",
                "e847f56514dadde23f77b64fe7f7d490",
                "c81677bc9b7ac93b25027992b0261996",
                "3caaa3e8a99f9deb50f3af57adf622aa",
                "f4bcd45432e554d075f1d6c51dd03b3c"
            ),
            (
                "36339d50f9b539269f2c092dc4406d23",
                "36400926f9336d2d9fb59d23c42c3950",
                "247240236966b3fa6ed2753288425b6c",
                "47f7f7bc95353e03f96c32bcfd058dfd",
                "6385b79ffc538df997be478e7547d691"
            ),
            (
                "2d6d7ef03f33e334093602dd5bfb12c7",
                "2dfb02343f6d12dd09337ec75b36e3f0",
                "fa636a2825b339c940668a3157244d17",
                "b6ff744ed2c2c9bf6c590cbf0469bf41",
                "4c9c1e66f771f0762c3f868e534df256"
            ),
            (
                "3bd92268fc74fb735767cbe0c0590e2d",
                "3b59cb73fcd90ee05774222dc067fb68",
                "4915598f55e5d7a0daca94fa1f0a63f7",
                "b692cf0b643dbdf1be9bc5006830b3fe",
                "ff87968431d86a51645151fa773ad009"
            ),
            (
                "a7be1a6997ad739bd8c9ca451f618b61",
                "a761ca9b97be8b45d8ad1a611fc97369",
                "89d810e8855ace682d1843d8cb128fe4",
                "d6aa74fdd2af72fadaa678f1d6ab76fe",
                "5f72641557f5bc92f7be3b291db9f91a"
            )
        ];

        expected.reverse();

        let mut state = State::from(ctx.block_size, &ciphertext);

        assert_eq!(
            ByteVec::from_hex("13111d7fe3944a17f307a78b4d2b30c5"),
            round_keys[ctx.nb_rounds].to_vec(),
            "round[0] .ik_sch"
        );

        state.add_round_key(&round_keys[ctx.nb_rounds]);

        for r in (1..ctx.nb_rounds).rev()
        {
            assert_eq!(
                ByteVec::from_hex(expected[r-1].0),
                state.output(),
                "round[{}] .istart", ctx.nb_rounds - r
            );

            state.inv_shift_rows();

            assert_eq!(
                ByteVec::from_hex(expected[r-1].1),
                state.output(),
                "round[{}] .is_row", ctx.nb_rounds - r
            );

            state.inv_sub_bytes();

            assert_eq!(
                ByteVec::from_hex(expected[r-1].2),
                state.output(),
                "round[{}] .is_box", ctx.nb_rounds - r
            );
            
            assert_eq!(
                ByteVec::from_hex(expected[r-1].3),
                round_keys[r].to_vec(),
                "round[{}] .ik_sch", ctx.nb_rounds - r
            );

            state.add_round_key(&round_keys[r]);

            assert_eq!(
                ByteVec::from_hex(expected[r-1].4),
                state.output(),
                "round[{}] .ik_add", ctx.nb_rounds - r
            );                

            state.inv_mix_columns();
        }

        assert_eq!(
            ByteVec::from_hex("6353e08c0960e104cd70b751bacad0e7"),
            state.output(),
            "round[10] .istart"
        );

        state.inv_shift_rows();

        assert_eq!(
            ByteVec::from_hex("63cab7040953d051cd60e0e7ba70e18c"),
            state.output(),
            "round[10] .is_row"
        );

        state.inv_sub_bytes();

        assert_eq!(
            ByteVec::from_hex("00102030405060708090a0b0c0d0e0f0"),
            state.output(),
            "round[10] .is_box"
        );

        assert_eq!(
            ByteVec::from_hex("000102030405060708090a0b0c0d0e0f"),
            round_keys[0],
            "round[10] .ik_sch"
        );

        state.add_round_key(&round_keys[0]);

        assert_eq!(
            ByteVec::from_hex("00112233445566778899aabbccddeeff"),
            state.output()
        );
    }
}