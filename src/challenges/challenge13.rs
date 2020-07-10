use crate::lib::types::ByteVec;
use crate::lib::crypto::aes;
use crate::lib::traits::{ToString, ToHex, Random};

fn profile_for(email: &str) -> String
{
    let cleaned_email = email.replace(&['&', '='][..], "");

    format!("email={0}&uid=10&role=user", cleaned_email)
}

fn encrypt_profile(profile: &str, key: &ByteVec) -> ByteVec
{
    let content = ByteVec::from(profile);
    aes::encrypt(aes::AesMode::ECB, &aes::Message::from(content, aes::Key::new(key)))
}

fn decrypt_profile(content: &ByteVec, key: &ByteVec) -> String
{
    let profile = aes::decrypt(
        aes::AesMode::ECB, 
        &aes::Message::from(
            content.to_vec(), 
            aes::Key::new(key)
        )
    );
    
    profile.to_string().unwrap()
}

fn detect_block_size() -> usize
{
    let tmp_key = ByteVec::random(16);
    let mut output_size = encrypt_profile(&"", &tmp_key).len();

    let mut n = 0;
    let mut sizes = vec![];

    loop
    {
        n += 1;

        let current_size = encrypt_profile(&"A".repeat(n), &tmp_key).len();

        // If adding a byte changes output size, we have found a block boundary
        if current_size > output_size
        {   
            sizes.push(n);
            output_size = current_size;
        }   

        if sizes.len() >= 2 { break }
    }

    let block_size = sizes[1] - sizes[0];
    println!("Block size : {}", block_size);

    block_size
}

fn get_admin_block(block_size: usize, key: &ByteVec) -> ByteVec
{
    let n_missing = block_size - "email=".len();
    let crafted   = format!(
        "{}{}{}",
        "A".repeat(n_missing),
        "admin",
        " ".repeat(block_size - "admin".len())
    );
    let profile   = profile_for(&crafted);

    let encrypted = encrypt_profile(&profile, &key);
    let start = "email=".len() + n_missing;

    encrypted[start..start + block_size].to_vec()
}

fn get_padding_block(block_size: usize, key: &ByteVec) -> ByteVec
{
    let n_missing = block_size - ("email=&uid=10&role=user".len() % block_size);
    let profile   = profile_for(&"A".repeat(n_missing));

    println!("[PADDING] Crafted : {}", profile);

    let encrypted = encrypt_profile(&profile, &key);

    encrypted[encrypted.len() - block_size..].to_vec()
}

pub fn challenge13()
{
    println!("=== [Challenge 13] ===");

    let email = "foo@bar.com";
    let key   = ByteVec::random(16);

    println!("Consistent key : {}", key.to_hex());
    println!("Email : {}", email);

    let profile = profile_for(email);

    let block_size = detect_block_size();

    // TODO
    // 1. Craft a block where 'admin           ' is on a full block (cut&paste)
    // 2. Craft a profile that is full-block sized (to get the full padding block)
    // 3. To get an admin profile, craft a block so role=[user] is starting on a new block
    //    Replace the last block with 1. (admin block) and concat 2. (full-padding block)

    // First craft a block with only "admin"
    let admin_block   = get_admin_block(block_size, &key);
    let padding_block = get_padding_block(block_size, &key);

    let base_size            = "email=&uid=10&role=".len();
    let extra_size           = base_size % block_size;
    let crafted_email        = "A".repeat(block_size - extra_size);
    let crafted_profile      = profile_for(&crafted_email);
    let encrypted_profile    = encrypt_profile(&crafted_profile, &key);

    println!("Crafted profile : {}", crafted_profile);
    println!("Encrypted profile : {}", encrypted_profile.to_hex());

    let mut morphed_profile = encrypted_profile[..(base_size + crafted_email.len())].to_vec();
    morphed_profile.extend(admin_block);
    morphed_profile.extend(padding_block);

    println!("Morphed encrypted profile : {}", morphed_profile.to_hex());

    let hacked_profile = decrypt_profile(&morphed_profile, &key);
    println!("Hacked profile : {}", hacked_profile);
}