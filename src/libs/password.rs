use bcrypt::DEFAULT_COST;
use dotenvy::dotenv;
use sha2::{Digest, Sha512};

pub fn salt() -> uuid::Uuid {
    uuid::Uuid::new_v4()
}

pub fn preprocess_password(password: &str, salt: &uuid::Uuid) -> String {
    dotenv().ok();
    let pepper = std::env::var("pepper").expect("PEPPER not set");

    let combined = format!("{}{}{}", password, salt.to_string(), pepper);
    let md5_hashed = format!(
        "{:?}{}{}",
        md5::compute(combined),
        password,
        salt.to_string()
    );

    let final_hash = Sha512::new().chain_update(md5_hashed).finalize();

    hex::encode(final_hash)
}

pub fn encrypt_password(password: &str, salt: &uuid::Uuid) -> String {
    let prehashed = preprocess_password(password, salt);
    bcrypt::hash(prehashed, DEFAULT_COST).unwrap()
}

pub fn validate_password(password: &str, salt: &uuid::Uuid, hash: &str) -> bool {
    let prehashed = preprocess_password(password, salt);
    bcrypt::verify(prehashed, hash).unwrap_or(false)
}
