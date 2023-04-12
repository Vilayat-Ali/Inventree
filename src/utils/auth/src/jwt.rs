use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub email: String,
}

impl Claims {
    fn new(
        username: String,
        email: String,
    ) -> Self {
        Self { username, email }
    }
}

/// # Generating Access Token
/// The function takes username and email and returns the token wrapped in Ok.
/// The cryptographic algorithm used to hash the tokens is SHA-512 for optimal security.
pub fn generate_token(
    username: String,
    email: String,
) -> Result<String, Error> {
    let token = encode(
        &Header::new(Algorithm::HS512),
        &Claims::new(username, email),
        &EncodingKey::from_secret("secret".as_ref()),
    )?;
    Ok(token)
}

/// This function is used to verify a token passed into headers of the request by the client
pub fn verify_token(access_token: String) -> Result<TokenData<Claims>, Error> {
    let validation_result = decode::<Claims>(
        &access_token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS512),
    )?;
    Ok(validation_result)
}
