use std::collections::HashMap;
use crate::{SignInIdpMessage, Error, SignInIdp, Provider, AccountInfo, RefreshToken, SignUpWithEmail, SignInWithPassword};
use note_utils::{MyError};
use anyhow::anyhow;
const API_BASE_SERVICE_URI: &str = "https://identitytoolkit.googleapis.com/v1";
const API_REFRESH_TOKEN_ENDPOINT: &str = "/token";
const API_SIGN_UP_ENDPOINT: &str = "/accounts:signUp";
// const API_SIGN_IN_WITH_CUSTOM_TOKEN_ENDPOINT: &str = "/accounts:signInWithCustomToken";
const API_SIGN_IN_WITH_PASSWORD_ENDPOINT: &str = "/accounts:signInWithPassword";
const API_SIGN_IN_WITH_IDP_ENDPOINT: &str = "/accounts:signInWithIdp";
// const API_CREATE_AUTH_URI_ENDPOINT: &str = "/accounts:createAuthUri";
// const API_SEND_OOB_CODE_ENDPOINT: &str = "/accounts:sendOobCode";
// const API_RESET_PASSWORD_ENDPOINT: &str = "/accounts:resetPassword";
// const API_UPDATE_ENDPOINT: &str = "/accounts:update";
const API_LOOKUP_ENDPOINT: &str = "/accounts:lookup";

#[derive(Debug, Clone)]
pub struct Service {
    api_key: String,
}
impl Service {
    pub fn new(api_key: &str) -> Service {
        Service {
            api_key: api_key.to_string(),
        }
    }

    pub fn set_api_key(&mut self, api_key: &str) {
        self.api_key = api_key.to_string();
    }
}

#[cfg(feature = "blocking")]
impl Service {
    pub fn refresh_token(&self, refresh_token: &str) -> Result<RefreshToken, MyError> {
        let client = reqwest::blocking::Client::new();

        let mut data = HashMap::new();
        data.insert("grant_type", "refresh_token");
        data.insert("refresh_token", refresh_token);

        let url = format!("{}{}?key={}", API_BASE_SERVICE_URI, API_REFRESH_TOKEN_ENDPOINT, &self.api_key);
        let response_text = client.post(&url)
            .form(&data)
            .send()?
            .text()?;

        // println!("{}", &response_text);

        if response_text.contains("error") {
            let error_message = serde_json::from_str::<Error>(&response_text).unwrap();
            Err(MyError::new( anyhow!(error_message.error.message)))
        } else {
            let response = serde_json::from_str::<RefreshToken>(&response_text).unwrap();
            Ok(response)
        }
    }

    pub fn sign_up_with_email(&self, email: &str, password: &str) -> Result<SignUpWithEmail, MyError> {
        let client = reqwest::blocking::Client::new();
        let data = serde_json::json!({
            "email": email,
            "password": password,
            "returnSecureToken": true
        });
        let url = format!("{}{}?key={}", API_BASE_SERVICE_URI, API_SIGN_UP_ENDPOINT, &self.api_key);
        let response_text = client.post(&url)
            .json(&data)
            .send()?
            .text()?;

        // println!("{}", &response_text);

        if response_text.contains("error") {
            let error_message = serde_json::from_str::<Error>(&response_text).unwrap();
            Err(MyError::new( anyhow!(error_message.error.message)))
        } else {
            let response = serde_json::from_str::<SignUpWithEmail>(&response_text).unwrap();
            Ok(response)
        }
    }

    pub fn sign_in_with_password(&self, email: &str, password: &str) -> Result<SignInWithPassword, MyError> {
        let client = reqwest::blocking::Client::new();

        let data = serde_json::json!({
            "email": email,
            "password": password,
            "returnSecureToken": true
        });
        let url = format!("{}{}?key={}", API_BASE_SERVICE_URI, API_SIGN_IN_WITH_PASSWORD_ENDPOINT, &self.api_key);
        let response_text = client.post(&url)
            .json(&data)
            .send()?
            .text()?;

        // println!("{}", &response_text);

        if response_text.contains("error") {
            let error_message = serde_json::from_str::<Error>(&response_text).unwrap();
            Err(MyError::new( anyhow!(error_message.error.message)))
        } else {
            let response = serde_json::from_str::<SignInWithPassword>(&response_text).unwrap();
            Ok(response)
        }
    }

    pub fn sign_in_with_idp(&self, provider: Provider, access_token: &str) -> Result<SignInIdp, MyError> {
        let client = reqwest::blocking::Client::new();

        let sign_in_message = SignInIdpMessage::new(
            provider,
            Some(access_token.to_string()),
                None,
            None
        );
        // println!("{}", &sign_in_message.to_string());

        let url = format!("{}{}?key={}", API_BASE_SERVICE_URI, API_SIGN_IN_WITH_IDP_ENDPOINT, &self.api_key);
        let response_text = client.post(&url)
            .json(&sign_in_message)
            .send()?
            .text()?;

        // println!("{}", &response_text);

        if response_text.contains("error") {
            let error_message = serde_json::from_str::<Error>(&response_text).unwrap();
            Err(MyError::new( anyhow!(error_message.error.message)))
        } else {
            let response = serde_json::from_str::<SignInIdp>(&response_text).unwrap();
            Ok(response)
        }
    }

    pub fn lookup(&self, id_token: &str) -> Result<AccountInfo, MyError> {
        let client = reqwest::blocking::Client::new();

        let data = serde_json::json!({
            "idToken": id_token
        });

        let url = format!("{}{}?key={}", API_BASE_SERVICE_URI, API_LOOKUP_ENDPOINT, &self.api_key);
        let response_text = client.post(&url)
            .json(&data)
            .send()?
            .text()?;

        // println!("{}", &response_text);

        if response_text.contains("error") {
            let error_message = serde_json::from_str::<Error>(&response_text).unwrap();
            Err(MyError::new( anyhow!(error_message.error.message)))
        } else {
            let response = serde_json::from_str::<AccountInfo>(&response_text).unwrap();
            Ok(response)
        }
    }

}


#[cfg(feature = "async")]
impl Service {
    pub async fn refresh_token_async(&self, refresh_token: &str) -> Result<RefreshToken, MyError> {
        let client = reqwest::Client::new();

        let mut data = HashMap::new();
        data.insert("grant_type", "refresh_token");
        data.insert("refresh_token", refresh_token);

        let url = format!("{}{}?key={}", API_BASE_SERVICE_URI, API_REFRESH_TOKEN_ENDPOINT, &self.api_key);
        let response_text = client.post(&url)
            .form(&data)
            .send().await?
            .text().await?;

        // println!("{}", &response_text);

        if response_text.contains("errors") {
            let error_message = serde_json::from_str::<Error>(&response_text).unwrap();
            Err(MyError::new( anyhow!(error_message.error.message)))
        } else {
            println!("{}", &response_text);
            let response = serde_json::from_str::<RefreshToken>(&response_text).unwrap();
            Ok(response)
        }
    }

    pub async fn sign_up_with_email_async(&self, email: &str, password: &str) -> Result<SignUpWithEmail, MyError> {
        let client = reqwest::Client::new();

        let data = serde_json::json!({
            "email": email,
            "password": password,
            "returnSecureToken": true
        });
        let url = format!("{}{}?key={}", API_BASE_SERVICE_URI, API_SIGN_UP_ENDPOINT, &self.api_key);
        let response_text = client.post(&url)
            .json(&data)
            .send().await?
            .text().await?;

        // println!("{}", &response_text);

        if response_text.contains("error") {
            let error_message = serde_json::from_str::<Error>(&response_text).unwrap();
            Err(MyError::new( anyhow!(error_message.error.message)))
        } else {
            let response = serde_json::from_str::<SignUpWithEmail>(&response_text).unwrap();
            Ok(response)
        }
    }

    pub async fn sign_in_with_password_async(&self, email: &str, password: &str) -> Result<SignInWithPassword, MyError> {
        let client = reqwest::Client::new();

        let data = serde_json::json!({
            "email": email,
            "password": password,
            "returnSecureToken": true
        });
        let url = format!("{}{}?key={}", API_BASE_SERVICE_URI, API_SIGN_IN_WITH_PASSWORD_ENDPOINT, &self.api_key);
        let response_text = client.post(&url)
            .json(&data)
            .send().await?
            .text().await?;

        // println!("{}", &response_text);

        if response_text.contains("error") {
            let error_message = serde_json::from_str::<Error>(&response_text).unwrap();
            Err(MyError::new( anyhow!(error_message.error.message)))
        } else {
            let response = serde_json::from_str::<SignInWithPassword>(&response_text).unwrap();
            Ok(response)
        }
    }

    pub async fn sign_in_with_idp_async(&self, provider: Provider, access_token: &str) -> Result<SignInIdp, MyError> {
        let client = reqwest::Client::new();

        let sign_in_message = SignInIdpMessage::new(
            provider,
            Some(access_token.to_string()),
            None,
            None
        );
        // println!("{}", &sign_in_message.to_string());

        let url = format!("{}{}?key={}", API_BASE_SERVICE_URI, API_SIGN_IN_WITH_IDP_ENDPOINT, &self.api_key);
        let response = client.post(&url)
            .json(&sign_in_message)
            .send().await?
            .json::<SignInIdp>().await?;
            Ok(response)

    }

    pub async fn lookup_async(&self, id_token: &str) -> Result<AccountInfo, MyError> {
        let client = reqwest::Client::new();

        let data = serde_json::json!({
            "idToken": id_token
        });

        let url = format!("{}{}?key={}", API_BASE_SERVICE_URI, API_LOOKUP_ENDPOINT, &self.api_key);
        let response_text = client.post(&url)
            .json(&data)
            .send().await?
            .text().await?;

        // println!("{}", &response_text);

        if response_text.contains("error") {
            let error_message = serde_json::from_str::<Error>(&response_text).unwrap();
            Err(MyError::new( anyhow!(error_message.error.message)))
        } else {
            let response = serde_json::from_str::<AccountInfo>(&response_text).unwrap();
            Ok(response)
        }
    }

}

