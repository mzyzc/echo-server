use std::error::Error;
use std::str;
use argon2;
use getrandom;

pub struct Login {
    pub email: Option<String>,
    pub is_authenticated: bool,
}

impl Login {
    // Set a user as authenticated
    pub fn authenticate(&mut self, email: String) {
        self.email = Some(email);
        self.is_authenticated = true;
    }
}

pub struct Password {
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
}

impl Password {
    // Create a password hash from a string and an (optionally provided) salt
    pub fn hash(password: &str, salt: Option<&[u8]>) -> Result<Self, Box<dyn Error>> {
        // Use provided salt or generate a new one
        let salt = match salt {
            Some(s) => s.to_owned(),
            None => {
                let mut salt = vec![0u8; 32];
                getrandom::getrandom(&mut salt)?;
                salt
            }
        };

        let hash = argon2::hash_encoded(
            password.as_bytes(),
            &salt,
            &argon2::Config::default()
        )?;

        Ok(Password{
            hash: hash.into_bytes(),
            salt,
        })
    }

    // Check if a password matches the stored hash
    pub fn is_valid(&self, password: &str) -> Result<bool, Box<dyn Error>> {
        let hash = str::from_utf8(&self.hash)?;
        let result = argon2::verify_encoded(hash, password.as_bytes())?;

        Ok(result)
    }
}