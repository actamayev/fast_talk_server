use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use argon2::password_hash::Error;

pub struct Hash;

impl Hash {
    pub fn hash_credentials(unhashed_data: &str) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(unhashed_data.as_bytes(), &salt)?.to_string();
        Ok(password_hash)
    }

    pub fn check_password(plaintext_password: &str, hashed_password: &str) -> Result<bool, Error> {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(hashed_password)?;

        match argon2.verify_password(plaintext_password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
