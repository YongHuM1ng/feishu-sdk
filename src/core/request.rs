use std::collections::HashMap;
use std::time::Duration;

use reqwest::header::HeaderMap;
use reqwest::{Method, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;

use super::error::Error;

pub const HEADER_LOG_ID: &str = "x-tt-logid";
pub const HEADER_REQUEST_ID: &str = "x-request-id";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AccessTokenType {
    None,
    App,
    Tenant,
    User,
}

impl AccessTokenType {
    pub fn as_str(self) -> &'static str {
        match self {
            AccessTokenType::None => "none_access_token",
            AccessTokenType::App => "app_access_token",
            AccessTokenType::Tenant => "tenant_access_token",
            AccessTokenType::User => "user_access_token",
        }
    }

    pub fn from_go_name(input: &str) -> Option<Self> {
        if input.ends_with("AccessTokenTypeNone") || input == "none_access_token" {
            return Some(Self::None);
        }
        if input.ends_with("AccessTokenTypeApp") || input == "app_access_token" {
            return Some(Self::App);
        }
        if input.ends_with("AccessTokenTypeTenant") || input == "tenant_access_token" {
            return Some(Self::Tenant);
        }
        if input.ends_with("AccessTokenTypeUser") || input == "user_access_token" {
            return Some(Self::User);
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct RequestOptions {
    pub headers: HeaderMap,
    pub user_access_token: Option<String>,
    pub tenant_access_token: Option<String>,
    pub app_access_token: Option<String>,
    pub tenant_key: Option<String>,
    pub app_ticket: Option<String>,
    pub timeout: Option<Duration>,
    pub retry_count: Option<u32>,
    pub retry_delay: Option<Duration>,
}

impl Default for RequestOptions {
    fn default() -> Self {
        Self {
            headers: HeaderMap::new(),
            user_access_token: None,
            tenant_access_token: None,
            app_access_token: None,
            tenant_key: None,
            app_ticket: None,
            timeout: None,
            retry_count: None,
            retry_delay: None,
        }
    }
}

impl RequestOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        use reqwest::header::{HeaderName, HeaderValue};
        if let Ok(header_name) = key.parse::<HeaderName>()
            && let Ok(header_value) = value.parse::<HeaderValue>()
        {
            self.headers.insert(header_name, header_value);
        }
        self
    }

    pub fn user_access_token(mut self, token: impl Into<String>) -> Self {
        self.user_access_token = Some(token.into());
        self
    }

    pub fn tenant_access_token(mut self, token: impl Into<String>) -> Self {
        self.tenant_access_token = Some(token.into());
        self
    }

    pub fn app_access_token(mut self, token: impl Into<String>) -> Self {
        self.app_access_token = Some(token.into());
        self
    }

    pub fn tenant_key(mut self, key: impl Into<String>) -> Self {
        self.tenant_key = Some(key.into());
        self
    }

    pub fn app_ticket(mut self, ticket: impl Into<String>) -> Self {
        self.app_ticket = Some(ticket.into());
        self
    }

    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    pub fn retry(mut self, count: u32, delay: Duration) -> Self {
        self.retry_count = Some(count);
        self.retry_delay = Some(delay);
        self
    }
}

#[derive(Debug, Clone)]
pub struct ApiRequest {
    pub method: Method,
    pub api_path: String,
    pub query: Vec<(String, String)>,
    pub path_params: HashMap<String, String>,
    pub body: Option<Value>,
    pub supported_token_types: Vec<AccessTokenType>,
}

impl ApiRequest {
    pub fn new(method: Method, api_path: impl Into<String>) -> Self {
        Self {
            method,
            api_path: api_path.into(),
            query: Vec::new(),
            path_params: HashMap::new(),
            body: None,
            supported_token_types: vec![AccessTokenType::None],
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiResponse {
    pub status: u16,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
}

impl ApiResponse {
    pub fn json<T: DeserializeOwned>(&self) -> Result<T, Error> {
        Ok(serde_json::from_slice(&self.body)?)
    }

    pub fn json_value(&self) -> Result<Value, Error> {
        Ok(serde_json::from_slice(&self.body)?)
    }

    pub fn request_id(&self) -> Option<String> {
        self.headers
            .get(HEADER_LOG_ID)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .or_else(|| {
                self.headers
                    .get(HEADER_REQUEST_ID)
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.to_string())
            })
    }
}

pub fn build_url(
    base_url: &str,
    api_path: &str,
    path_params: &HashMap<String, String>,
    query: &[(String, String)],
) -> Result<Url, Error> {
    let replaced_path = if api_path.starts_with("http://") || api_path.starts_with("https://") {
        api_path.to_string()
    } else {
        let mut segments = Vec::new();
        for segment in api_path.split('/') {
            if let Some(name) = segment.strip_prefix(':') {
                let value = path_params
                    .get(name)
                    .ok_or_else(|| Error::MissingPathParam(name.to_string()))?;
                segments.push(urlencoding::encode(value).to_string());
            } else {
                segments.push(segment.to_string());
            }
        }
        let merged = segments.join("/");
        format!("{}{}", base_url.trim_end_matches('/'), merged)
    };

    let mut url = Url::parse(&replaced_path).map_err(|e| Error::InvalidUrl(e.to_string()))?;
    if !query.is_empty() {
        let mut pairs = url.query_pairs_mut();
        for (k, v) in query {
            pairs.append_pair(k, v);
        }
    }
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_url_replaces_path_and_query() {
        let mut path_params = HashMap::new();
        path_params.insert("chat_id".to_string(), "oc_123/abc".to_string());
        let query = vec![("page_size".to_string(), "20".to_string())];
        let url = build_url(
            "https://open.feishu.cn",
            "/open-apis/im/v1/chats/:chat_id",
            &path_params,
            &query,
        )
        .expect("url must build");

        assert_eq!(
            url.as_str(),
            "https://open.feishu.cn/open-apis/im/v1/chats/oc_123%2Fabc?page_size=20"
        );
    }

    #[test]
    fn parse_access_token_type_from_go_name() {
        assert_eq!(
            AccessTokenType::from_go_name("larkcore.AccessTokenTypeTenant"),
            Some(AccessTokenType::Tenant)
        );
        assert_eq!(
            AccessTokenType::from_go_name("user_access_token"),
            Some(AccessTokenType::User)
        );
        assert_eq!(AccessTokenType::from_go_name("unknown"), None);
    }

    #[test]
    fn test_request_options_builder() {
        let options = RequestOptions::new()
            .user_access_token("u_token")
            .tenant_key("tenant_123")
            .timeout(Duration::from_secs(30))
            .retry(3, Duration::from_secs(1))
            .header("X-Custom-Header", "value");

        assert_eq!(options.user_access_token, Some("u_token".to_string()));
        assert_eq!(options.tenant_key, Some("tenant_123".to_string()));
        assert_eq!(options.timeout, Some(Duration::from_secs(30)));
        assert_eq!(options.retry_count, Some(3));
        assert_eq!(options.retry_delay, Some(Duration::from_secs(1)));
    }
}
