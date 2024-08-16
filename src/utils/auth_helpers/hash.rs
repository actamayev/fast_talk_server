use bcrypt::{hash, verify, DEFAULT_COST};

pub struct Hash;

impl Hash {
    pub fn hash_credentials(unhashed_data: &str) -> Result<String, bcrypt::BcryptError> {
        let hashed = hash(unhashed_data, DEFAULT_COST)?;
        Ok(hashed)
    }

    pub fn check_password(plaintext_password: &str, hashed_password: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(plaintext_password, hashed_password)
    }
}
