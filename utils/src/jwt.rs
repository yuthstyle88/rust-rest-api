use serde::{Deserialize};
use crate::claims::Claims;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct JWK {
    pub e: String,
    pub alg: String,
    pub kty: String,
    pub kid: String,
    pub n: String,
}

#[derive(Debug, Deserialize)]
pub struct KeysResponse {
    pub keys: Vec<JWK>,
}

pub fn encode(
    audience: &str,
    private_key_id: &str,
    private_key: &str,
    client_email: &str,
    uid: Option<String>,
) -> Result<String, jsonwebtoken::errors::Error> {
    Claims::encode_jwt(audience, private_key_id, private_key, client_email, uid)
}
