use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    // let salt = b"485uWUHE47362@(#$*&^$#%#^$%&^$%";
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, argon2::password_hash::Error>{
    let parsed_hash = PasswordHash::new(&hash)?;

    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).map(|_| true).map_err(|e| e)
}

