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

#[cfg(test)]
mod tests {
    use crate::auth::Password;

    #[test]
    fn test_hash() {
        let passwords = vec!["8nLpNaeJ", "9poyvjJN", "L3Chj2ne"];
        let salt = b"samplesalt";
        let test_hashes = vec![
            "$argon2i$v=19$m=4096,t=3,p=1$c2FtcGxlc2FsdA$75kN1JTjZ+AwNg3f5PvLU4Dp+4biUIo2BOqo9dYdXVE".to_string().into_bytes(),
            "$argon2i$v=19$m=4096,t=3,p=1$c2FtcGxlc2FsdA$yU0Lgj66mhc2a7HT6z9RTP6myZgssy99snipJyrAku4".to_string().into_bytes(),
            "$argon2i$v=19$m=4096,t=3,p=1$c2FtcGxlc2FsdA$d9UXA+y9LsGj89WB/3DNV6JpDwDr4fyo2rbjo02vilk".to_string().into_bytes(),
        ];

        let hashes: Vec<Password> = passwords
            .iter()
            .map(|x| Password::hash(x, Some(salt)).unwrap())
            .collect();

        assert_eq!(hashes[0].hash, test_hashes[0]);
        assert_eq!(hashes[1].hash, test_hashes[1]);
        assert_eq!(hashes[2].hash, test_hashes[2]);
    }

    #[test]
    fn test_is_valid() {
        let password = "k2uEa77H";
        let salt = b"samplesalt";

        let hash = Password::hash(password, Some(salt)).unwrap();

        assert_eq!(hash.is_valid(password).unwrap(), true);
    }
}
