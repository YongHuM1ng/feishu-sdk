pub mod application;
pub mod approval;
pub mod calendar;
pub mod contact;
pub mod drive;
pub mod im;

pub use application::*;
pub use approval::*;
pub use calendar::*;
pub use contact::*;
pub use drive::*;
pub use im::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventHeader {
    pub event_id: Option<String>,
    pub event_type: Option<String>,
    pub app_id: Option<String>,
    pub tenant_key: Option<String>,
    pub create_time: Option<String>,
    pub token: Option<String>,
    pub app_ticket: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Event<T = serde_json::Value> {
    pub schema: Option<String>,
    pub header: Option<EventHeader>,
    #[serde(flatten)]
    pub event: Option<T>,
    pub event_type: Option<String>,
    pub token: Option<String>,
    pub challenge: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl<T> Event<T> {
    pub fn event_id(&self) -> Option<&str> {
        self.header.as_ref()?.event_id.as_deref()
    }

    pub fn event_type(&self) -> Option<&str> {
        self.header
            .as_ref()?
            .event_type
            .as_deref()
            .or(self.event_type.as_deref())
    }

    pub fn tenant_key(&self) -> Option<&str> {
        self.header.as_ref()?.tenant_key.as_deref()
    }

    pub fn app_id(&self) -> Option<&str> {
        self.header.as_ref()?.app_id.as_deref()
    }

    pub fn is_challenge(&self) -> bool {
        self.type_.as_deref() == Some("url_verification") || self.challenge.is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeResponse {
    pub challenge: String,
}

impl ChallengeResponse {
    pub fn new(challenge: impl Into<String>) -> Self {
        Self {
            challenge: challenge.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventReq {
    pub header: std::collections::HashMap<String, Vec<String>>,
    pub body: Vec<u8>,
    pub request_uri: String,
}

impl EventReq {
    pub fn get_header(&self, key: &str) -> Option<&str> {
        self.header
            .get(key)
            .and_then(|v| v.first())
            .map(String::as_str)
    }
}

#[derive(Debug, Clone)]
pub struct EventResp {
    pub status_code: u16,
    pub headers: std::collections::HashMap<String, Vec<String>>,
    pub body: Vec<u8>,
}

impl EventResp {
    pub fn ok(body: Vec<u8>) -> Self {
        let mut headers = std::collections::HashMap::new();
        headers.insert(
            "Content-Type".to_string(),
            vec!["application/json".to_string()],
        );
        Self {
            status_code: 200,
            headers,
            body,
        }
    }

    pub fn challenge(challenge: impl Into<String>) -> Self {
        let body = serde_json::to_string(&ChallengeResponse::new(challenge))
            .unwrap_or_default()
            .into_bytes();
        Self::ok(body)
    }

    pub fn error(status: u16, message: impl Into<String>) -> Self {
        let mut headers = std::collections::HashMap::new();
        headers.insert(
            "Content-Type".to_string(),
            vec!["application/json".to_string()],
        );
        Self {
            status_code: status,
            headers,
            body: message.into().into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_is_challenge() {
        let event: Event = Event {
            type_: Some("url_verification".to_string()),
            ..Default::default()
        };
        assert!(event.is_challenge());

        let event: Event = Event {
            challenge: Some("test".to_string()),
            ..Default::default()
        };
        assert!(event.is_challenge());
    }

    #[test]
    fn test_event_helpers() {
        let event: Event = Event {
            header: Some(EventHeader {
                event_id: Some("evt_123".to_string()),
                event_type: Some("im.message.receive".to_string()),
                tenant_key: Some("tenant_456".to_string()),
                app_id: Some("app_789".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(event.event_id(), Some("evt_123"));
        assert_eq!(event.event_type(), Some("im.message.receive"));
        assert_eq!(event.tenant_key(), Some("tenant_456"));
        assert_eq!(event.app_id(), Some("app_789"));
    }

    #[test]
    fn test_challenge_response() {
        let resp = EventResp::challenge("test_challenge");
        assert_eq!(resp.status_code, 200);
        let body_str = String::from_utf8_lossy(&resp.body);
        assert!(body_str.contains("test_challenge"));
    }
}
