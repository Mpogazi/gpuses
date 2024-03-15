use openssl::error::ErrorStack;
use openssl::rsa::Rsa;
pub struct Keys;

impl Keys {
    pub fn new() -> Keys {
        Keys
    }

    pub fn generate(&self) -> Result<(String, String), ErrorStack> {
        let rsa = Rsa::generate(2048)?;
        let private_key_pem = rsa.private_key_to_pem()?;
        let public_key_pem = rsa.public_key_to_pem()?;

        let public_key = String::from_utf8(public_key_pem).unwrap();
        let private_key = String::from_utf8(private_key_pem).unwrap();
        Ok((private_key, public_key))
    }
}
