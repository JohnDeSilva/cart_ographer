use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RestaurantType {
    #[serde(rename = "Food Stall")]
    FoodStall,
    #[serde(rename = "Food Truck")]
    FoodTruck,
    #[serde(rename = "Brick and mortar Restaurant")]
    BrickAndMortar,
}

impl RestaurantType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FoodStall => "Food Stall",
            Self::FoodTruck => "Food Truck",
            Self::BrickAndMortar => "Brick and mortar Restaurant",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserRole {
    Admin,
    Customer,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Restaurant {
    pub id: i32,
    pub name: String,
    pub restaurant_type: RestaurantType,
    pub location: String,
    pub open_time: String,
    pub close_time: String,
    pub open_status: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestaurantCreate {
    pub name: String,
    pub restaurant_type: RestaurantType,
    pub location: String,
    pub open_time: String,
    pub close_time: String,
    pub open_status: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestaurantUpdate {
    pub name: Option<String>,
    pub restaurant_type: Option<RestaurantType>,
    pub location: Option<String>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
    pub open_status: Option<bool>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestaurantStatusUpdate {
    pub open_status: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub role: UserRole,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub role: UserRole,
}

#[derive(Debug)]
pub enum ApiError {
    Http(reqwest::Error),
    Unauthorized,
    Forbidden,
    ValidationError(String),
    Network,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(e) => write!(f, "HTTP error: {}", e),
            Self::Unauthorized => write!(f, "Incorrect username or password"),
            Self::Forbidden => write!(f, "Access denied: insufficient permissions"),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::Network => write!(f, "Network error: check connection"),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        if let Some(status) = err.status() {
            match status {
                reqwest::StatusCode::UNAUTHORIZED => ApiError::Unauthorized,
                reqwest::StatusCode::FORBIDDEN => ApiError::Forbidden,
                _ => ApiError::Http(err),
            }
        } else {
            ApiError::Network
        }
    }
}

pub struct ApiClient {
    base_url: String,
    client: reqwest::Client,
    token: Option<String>,
    pub username: Option<String>,
    pub role: Option<UserRole>,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
            token: None,
            username: None,
            role: None,
        }
    }

    pub fn set_token(&mut self, token: String, username: String, role: UserRole) {
        self.token = Some(token);
        self.username = Some(username);
        self.role = Some(role);
    }

    pub fn logout(&mut self) {
        self.token = None;
        self.username = None;
        self.role = None;
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Some(ref token) = self.token {
            if let Ok(val) = HeaderValue::from_str(&format!("Bearer {}", token)) {
                headers.insert(AUTHORIZATION, val);
            }
        }
        headers
    }

    pub async fn login(&mut self, username: &str, password: &str) -> Result<(), ApiError> {
        let url = format!("{}/auth/login", self.base_url);
        let payload = serde_json::json!({
            "username": username,
            "password": password,
        });
        
        let response = self.client.post(&url)
            .json(&payload)
            .send()
            .await?;
            
        if response.status().is_success() {
            let token_res: TokenResponse = response.json().await?;
            self.set_token(token_res.access_token, token_res.username, token_res.role);
            Ok(())
        } else {
            let status = response.status();
            if status == reqwest::StatusCode::UNAUTHORIZED {
                Err(ApiError::Unauthorized)
            } else {
                response.error_for_status()?;
                Err(ApiError::Network)
            }
        }
    }

    pub async fn signup(&self, username: &str, password: &str, role: UserRole) -> Result<UserResponse, ApiError> {
        let url = format!("{}/auth/signup", self.base_url);
        let payload = serde_json::json!({
            "username": username,
            "password": password,
            "role": role,
        });
        let response = self.client.post(&url)
            .json(&payload)
            .send()
            .await?;
            
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            response.error_for_status()?;
            Err(ApiError::Network)
        }
    }

    pub async fn reset_password(&self, username: &str, new_password: &str) -> Result<UserResponse, ApiError> {
        let url = format!("{}/auth/reset-password", self.base_url);
        let payload = serde_json::json!({
            "username": username,
            "new_password": new_password,
        });
        let response = self.client.post(&url)
            .json(&payload)
            .send()
            .await?;
            
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            response.error_for_status()?;
            Err(ApiError::Network)
        }
    }

    pub async fn get_restaurants(&self, name_filter: Option<&str>) -> Result<Vec<Restaurant>, ApiError> {
        let mut url = format!("{}/restaurants", self.base_url);
        if let Some(name) = name_filter {
            if !name.is_empty() {
                url = format!("{}?name={}", url, name);
            }
        }
        let response = self.client.get(&url)
            .headers(self.headers())
            .send()
            .await?;
            
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            response.error_for_status()?;
            Err(ApiError::Network)
        }
    }

    pub async fn create_restaurant(&self, r: &RestaurantCreate) -> Result<Restaurant, ApiError> {
        let url = format!("{}/restaurants", self.base_url);
        let response = self.client.post(&url)
            .headers(self.headers())
            .json(r)
            .send()
            .await?;
            
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            response.error_for_status()?;
            Err(ApiError::Network)
        }
    }

    pub async fn update_restaurant(&self, id: i32, r: &RestaurantUpdate) -> Result<Restaurant, ApiError> {
        let url = format!("{}/restaurants/{}", self.base_url, id);
        let response = self.client.put(&url)
            .headers(self.headers())
            .json(r)
            .send()
            .await?;
            
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            response.error_for_status()?;
            Err(ApiError::Network)
        }
    }

    pub async fn update_restaurant_status(&self, id: i32, open_status: bool) -> Result<Restaurant, ApiError> {
        let url = format!("{}/restaurants/{}/status", self.base_url, id);
        let payload = RestaurantStatusUpdate { open_status };
        let response = self.client.patch(&url)
            .headers(self.headers())
            .json(&payload)
            .send()
            .await?;
            
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            response.error_for_status()?;
            Err(ApiError::Network)
        }
    }

    pub async fn delete_restaurant(&self, id: i32) -> Result<(), ApiError> {
        let url = format!("{}/restaurants/{}", self.base_url, id);
        let response = self.client.delete(&url)
            .headers(self.headers())
            .send()
            .await?;
            
        if response.status().is_success() {
            Ok(())
        } else {
            response.error_for_status()?;
            Err(ApiError::Network)
        }
    }
}
