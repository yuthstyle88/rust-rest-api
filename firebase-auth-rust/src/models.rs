#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Provider {
    Google,
    Facebook,
    Twitter,
    Line,
}

impl ToString for Provider {
    fn to_string(&self) -> String {
        match self {
            Provider::Google => { "google.com" }
            Provider::Facebook => { "facebook.com" }
            Provider::Twitter => { "twitter.com" }
            Provider::Line => { "line.me" }
        }.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignInIdpMessage {
    #[serde(rename = "postBody")]
    post_body: String,
    #[serde(rename = "requestUri")]
    request_uri: String,
    #[serde(rename = "idToken")]
    id_token: Option<String>,
    #[serde(rename = "returnIdpCredential")]
    return_idp_credential: bool,
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool
}

impl SignInIdpMessage {
    pub fn new(provider: Provider, access_token: Option<String>, id_token: Option<String>, request_uri: Option<String>) -> SignInIdpMessage {
        let post_body = PostBody::new(provider, access_token, id_token.clone());
        SignInIdpMessage {
            post_body: post_body.to_string(),
            request_uri: request_uri.unwrap_or("http://localhost".to_string()),
            id_token,
            return_idp_credential: true,
            return_secure_token: true
        }
    }
}

impl ToString for SignInIdpMessage {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostBody {
    #[serde(rename = "providerId")]
    provider_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_token: Option<String>,
}

impl PostBody {
    pub fn new(provider: Provider, access_token: Option<String>, id_token: Option<String>) -> PostBody {
        PostBody {
            provider_id: provider.to_string(),
            access_token,
            id_token,
        }
    }
}

impl ToString for PostBody {
    fn to_string(&self) -> String {
        serde_qs::to_string(&self).unwrap()
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SignInIdp {
    #[serde(rename = "federatedId")]
    federated_id: String,
    #[serde(rename = "providerId")]
    provider_id: String,
    #[serde(rename = "localId")]
    local_id: String,
    #[serde(rename = "emailVerified")]
    email_verified: bool,
    email: Option<String>,
    #[serde(rename = "oauthIdToken")]
    oauth_id_token: Option<String>,
    #[serde(rename = "oauthAccessToken")]
    oauth_access_token: String,
    #[serde(rename = "oauthTokenSecret")]
    oauth_token_secret: Option<String>,
    #[serde(rename = "rawUserInfo")]
    raw_user_info: Option<String>,
    #[serde(rename = "firstName")]
    first_name: Option<String>,
    #[serde(rename = "lastName")]
    last_name: Option<String>,
    #[serde(rename = "fullName")]
    full_name: Option<String>,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    #[serde(rename = "photoUrl")]
    photo_url: Option<String>,
    #[serde(rename = "idToken")]
    id_token: String,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    #[serde(rename = "expiresIn")]
    expires_in: Option<String>,
}

impl SignInIdp {
    pub fn federated_id(&self) -> &str {
        &self.federated_id
    }
    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }
    pub fn local_id(&self) -> String {
        self.local_id.clone()
    }
    pub fn email_verified(&self) -> bool {
        self.email_verified
    }
    pub fn email(&self) -> Option<String> {
        self.email.clone()
    }
    pub fn oauth_id_token(&self) -> &Option<String> {
        &self.oauth_id_token
    }
    pub fn oauth_access_token(&self) -> &str {
        &self.oauth_access_token
    }
    pub fn oauth_token_secret(&self) -> &Option<String> {
        &self.oauth_token_secret
    }
    pub fn raw_user_info(&self) -> &Option<String> {
        &self.raw_user_info
    }
    pub fn first_name(&self) -> &Option<String> {
        &self.first_name
    }
    pub fn last_name(&self) -> &Option<String> {
        &self.last_name
    }
    pub fn full_name(&self) -> &Option<String> {
        &self.full_name
    }
    pub fn display_name(&self) -> &Option<String> {
        &self.display_name
    }
    pub fn photo_url(&self) -> &Option<String> {
        &self.photo_url
    }
    pub fn id_token(&self) -> &str {
        &self.id_token
    }
    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }
    pub fn expires_in(&self) -> &Option<String> {
        &self.expires_in
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken {
    expires_in: Option<String>,
    token_type: Option<String>,
    refresh_token: String,
    id_token: String,
    user_id: String,
    project_id: String,
}

impl RefreshToken {
    pub fn expires_in(&self) -> &Option<String> {
        &self.expires_in
    }
    pub fn token_type(&self) -> &Option<String> {
        &self.token_type
    }
    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }
    pub fn id_token(&self) -> &str {
        &self.id_token
    }
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
    pub fn project_id(&self) -> &str {
        &self.project_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignUpWithEmail {
    #[serde(rename = "idToken")]
    id_token: String,
    email: String,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    #[serde(rename = "expiresIn")]
    expires_in: Option<String>,
    #[serde(rename = "localId")]
    local_id: String,
}

impl SignUpWithEmail {
    pub fn id_token(&self) -> &str {
        &self.id_token
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }
    pub fn expires_in(&self) -> &Option<String> {
        &self.expires_in
    }
    pub fn local_id(&self) -> &str {
        &self.local_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignInWithPassword {
    #[serde(rename = "localId")]
    local_id: String,
    email: String,
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "idToken")]
    id_token: String,
    registered: bool,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    #[serde(rename = "expiresIn")]
    expires_in: Option<String>,
}

impl SignInWithPassword {
    pub fn local_id(&self) -> String {
        self.local_id.clone()
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
    pub fn id_token(&self) -> &str {
        &self.id_token
    }
    pub fn registered(&self) -> bool {
        self.registered
    }
    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }
    pub fn expires_in(&self) -> &Option<String> {
        &self.expires_in
    }
}

impl Default for SignInWithPassword {
    fn default() -> Self {
        Self {
            local_id: "".to_string(),
            email: "".to_string(),
            display_name: "".to_string(),
            id_token: "".to_string(),
            registered: false,
            refresh_token: "".to_string(),
            expires_in: None
        }
    }
}