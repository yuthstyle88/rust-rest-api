use std::time;
use jsonwebtoken::{Header, Algorithm, EncodingKey};
use serde::{Deserialize, Serialize};
const MAX_TOKEN_LIFETIME_SECONDS: u64 = 3600;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub aud: String,
  pub iat: u64,
  pub exp: u64,
  pub iss: String,
  pub sub: String,
  pub uid: Option<String>,
}

impl Claims {
  pub fn new(audience: &str, client_email: &str, uid: Option<String>) -> Claims {
    let now = time::SystemTime::now();
    let iat = now.duration_since(time::UNIX_EPOCH).unwrap().as_secs();
    Claims {
      aud: audience.to_string(),
      iat,
      exp: iat + MAX_TOKEN_LIFETIME_SECONDS,
      iss: client_email.to_string(),
      sub: client_email.to_string(),
      uid,
    }
  }

  pub(crate) fn encode_jwt(
    audience: &str,
    private_key_id: &str,
    private_key: &str,
    client_email: &str,
    uid: Option<String>,
  ) -> Result<String, jsonwebtoken::errors::Error> {
    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some(private_key_id.to_string());

    let claims = Claims::new(audience, client_email, uid);
    let pem = str::as_bytes(&private_key);
    let secret_key = EncodingKey::from_rsa_pem(pem);
    match secret_key {
      Ok(key) => jsonwebtoken::encode(&header, &claims, &key),
      Err(err) => Err(err),
    }
  }
}
