use std::error::Error;
use std::str;
use argon2;
use getrandom;

pub struct Password {
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
}

impl Password {
    pub fn hash(password: &str) -> Result<Self, Box<dyn Error>> {
        let mut salt = vec![0u8; 32];
        getrandom::getrandom(&mut salt)?;

        let hash = argon2::hash_encoded(
            password.as_bytes(),
            &salt,
            &argon2::Config::default()
        )?;

        Ok(Password{
            hash: hash.into_bytes(),
            salt: salt,
        })
    }

    pub fn is_valid(&self, password: &str) -> Result<bool, Box<dyn Error>> {
        let hash = str::from_utf8(&self.hash)?;
        let result = argon2::verify_encoded(hash, password.as_bytes())?;

        Ok(result)
    }
}