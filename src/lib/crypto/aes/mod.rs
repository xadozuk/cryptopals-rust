pub mod consts;
pub mod traits;

mod types;

use crate::lib::types::ByteVec;

use types::{Key, State};
use traits::{Transforms, KeyExpansion};

pub struct Aes128
{
    key_length: usize,
    block_size: usize,
    nb_rounds: usize
}

impl Aes128
{
    pub fn new() -> Self
    {
        Aes128 {
            key_length: 4,
            block_size: 4,
            nb_rounds: 10
        }
    }

    fn cipher(&self, input: &ByteVec, key: &Key) -> ByteVec
    {
        let mut state = State::from(self.block_size, input);
        let kr_size = 4 * self.block_size;

        let round_keys = key.expand();

        state.add_round_key(&round_keys[0..kr_size]);

        for r in 1..self.nb_rounds
        {
            state.sub_bytes();
            state.shift_rows();
            state.mix_columns();

            state.add_round_key(&round_keys[r*kr_size..(r+1)*kr_size]);
        }

        state.sub_bytes();
        state.shift_rows();
        state.add_round_key(&round_keys[self.nb_rounds*kr_size..(self.nb_rounds+1)*kr_size]);

        state.output()
    }
}

#[cfg(test)]
mod tests
{
    mod aes128
    {
        use super::super::*;
        use crate::lib::types::ByteVec;
        use crate::lib::traits::FromHex;

        #[test]
        fn new()
        {
            let aes = Aes128::new();

            assert_eq!(4, aes.key_length);
            assert_eq!(4, aes.block_size);
            assert_eq!(10, aes.nb_rounds);
        }

        #[test]
        fn cipher()
        {
            let aes = Aes128::new();
            let key = Key::new(&ByteVec::from("YELLOW SUBMARINE"));

            let ciphertext = aes.cipher(&ByteVec::from("Lorem ipsum dolo"), &key);

            assert_eq!(
                ByteVec::from_hex("eadcc5aa4800dff175a49cf3a0f2041d"),
                ciphertext
            )
        }

        fn do_cipher_round(round: usize, state: &mut State, round_keys: &ByteVec)
        {
            state.sub_bytes();
            state.shift_rows();
            state.mix_columns();

            state.add_round_key(&round_keys[round*16..(round+1)*16]);
        }

        #[test]
        fn cipher_vector_test()
        {
            let aes = Aes128::new();

            let plaintext = ByteVec::from_hex("00112233445566778899aabbccddeeff");
            let key = Key::new(&ByteVec::from_hex("000102030405060708090a0b0c0d0e0f"));

            let round_keys = key.expand();

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


            let mut state = State::from(aes.block_size, &plaintext);
            let kr_size = 4 * aes.block_size;

            assert_eq!(
                ByteVec::from_hex("000102030405060708090a0b0c0d0e0f"),
                round_keys[0..kr_size].to_vec(),
                "round[0] .k_sch"
            );

            state.add_round_key(&round_keys[0..kr_size]);

            for r in 1..aes.nb_rounds
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

                state.add_round_key(&round_keys[r*kr_size..(r+1)*kr_size]);

                assert_eq!(
                    ByteVec::from_hex(expected[r-1].4),
                    round_keys[r*kr_size..(r+1)*kr_size].to_vec(),
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

            state.add_round_key(&round_keys[aes.nb_rounds*kr_size..(aes.nb_rounds+1)*kr_size]);

            assert_eq!(
                ByteVec::from_hex("13111d7fe3944a17f307a78b4d2b30c5"),
                &round_keys[aes.nb_rounds*kr_size..(aes.nb_rounds+1)*kr_size],
                "round[10] .k_sch"
            );

            assert_eq!(
                ByteVec::from_hex("69c4e0d86a7b0430d8cdb78070b4c55a"),
                state.output()
            );
        }
    }
}