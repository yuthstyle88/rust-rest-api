#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub kind: String,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "localId")]
    pub local_id: String,
    pub email: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(rename = "providerUserInfo")]
    pub provider_user_info: Vec<UserInfoProvider>,
    #[serde(rename = "passwordHash")]
    pub password_hash: Option<String>,
    #[serde(rename = "passwordUpdatedAt")]
    pub password_updated_at: Option<u64>,
    #[serde(rename = "validSince")]
    pub valid_since: String,
    pub disabled: Option<bool>,
    #[serde(rename = "lastLoginAt")]
    pub last_login_at: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "customAuth")]
    pub custom_auth: bool,
    #[serde(rename = "lastRefreshAt")]
    pub last_refresh_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoProvider {
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    #[serde(rename = "federatedId")]
    pub federated_id: String,
    pub email: String,
    #[serde(rename = "rawId")]
    pub raw_id: String,
    #[serde(rename = "screenName")]
    pub screen_name: Option<String>,
}