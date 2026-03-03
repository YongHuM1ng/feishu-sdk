use std::collections::HashMap;
use std::time::Duration;

use reqwest::Method;
use reqwest::header::{HeaderName, HeaderValue};
use serde::Serialize;
use serde_json::Value;

use crate::core::{
    ApiRequest, ApiResponse, CacheRef, CoreClient, Error, HttpClientRef, LogLevel, LoggerRef,
    RequestOptions,
};
use crate::generated::{Endpoint, find_endpoint};
use crate::utils::SerializerRef;

pub type ClientOption = Box<dyn FnOnce(&mut ClientBuilder) + Send>;

#[derive(Debug, Clone)]
pub struct Client {
    core: CoreClient,
}

pub struct ClientBuilder {
    config: crate::core::Config,
}

impl ClientBuilder {
    pub fn new(config: crate::core::Config) -> Self {
        Self { config }
    }

    pub fn with(mut self, option: ClientOption) -> Self {
        option(&mut self);
        self
    }

    pub fn build(self) -> Result<Client, Error> {
        Ok(Client {
            core: CoreClient::new(self.config)?,
        })
    }
}

pub fn with_logger(logger: LoggerRef) -> ClientOption {
    Box::new(move |builder: &mut ClientBuilder| {
        builder.config.logger = logger;
    })
}

pub fn with_log_level(level: LogLevel) -> ClientOption {
    Box::new(move |builder: &mut ClientBuilder| {
        builder.config.log_level = level;
        builder.config.logger = crate::core::new_logger(level);
    })
}

pub fn with_token_cache(cache: CacheRef) -> ClientOption {
    Box::new(move |builder: &mut ClientBuilder| {
        builder.config.token_cache = cache;
    })
}

pub fn with_http_client(client: HttpClientRef) -> ClientOption {
    Box::new(move |builder: &mut ClientBuilder| {
        builder.config.http_client = Some(client);
    })
}

pub fn with_serializer(serializer: SerializerRef) -> ClientOption {
    Box::new(move |builder: &mut ClientBuilder| {
        builder.config.serializer = serializer;
    })
}

pub fn with_request_timeout(timeout: Duration) -> ClientOption {
    Box::new(move |builder: &mut ClientBuilder| {
        builder.config.request_timeout = Some(timeout);
    })
}

pub fn with_headers(headers: reqwest::header::HeaderMap) -> ClientOption {
    Box::new(move |builder: &mut ClientBuilder| {
        builder.config.default_headers = headers;
    })
}

impl Client {
    pub fn new(config: crate::core::Config) -> Result<Self, Error> {
        Ok(Self {
            core: CoreClient::new(config)?,
        })
    }

    pub fn builder(config: crate::core::Config) -> ClientBuilder {
        ClientBuilder::new(config)
    }

    pub fn endpoint(&self, operation_id: &str) -> Option<&'static Endpoint> {
        find_endpoint(operation_id)
    }

    pub fn operation(&self, operation_id: impl Into<String>) -> OperationBuilder<'_> {
        OperationBuilder {
            client: self,
            operation_id: operation_id.into(),
            path_params: HashMap::new(),
            query: Vec::new(),
            body: None,
            options: RequestOptions::default(),
        }
    }

    pub async fn call(
        &self,
        operation_id: &str,
        path_params: HashMap<String, String>,
        query: Vec<(String, String)>,
        body: Option<Value>,
        options: RequestOptions,
    ) -> Result<ApiResponse, Error> {
        let endpoint = find_endpoint(operation_id)
            .ok_or_else(|| Error::InvalidUrl(format!("unknown operation id: {operation_id}")))?;
        let method = http_method(endpoint.http_method)?;

        let mut req = ApiRequest::new(method, endpoint.path);
        req.path_params = path_params;
        req.query = query;
        req.body = body;
        req.supported_token_types = endpoint.token_types.to_vec();
        self.core.request(&req, &options).await
    }

    pub async fn call_with_body<T: Serialize>(
        &self,
        operation_id: &str,
        path_params: HashMap<String, String>,
        query: Vec<(String, String)>,
        body: &T,
        options: RequestOptions,
    ) -> Result<ApiResponse, Error> {
        let body = serde_json::to_value(body)?;
        self.call(operation_id, path_params, query, Some(body), options)
            .await
    }
}

pub struct OperationBuilder<'a> {
    client: &'a Client,
    operation_id: String,
    path_params: HashMap<String, String>,
    query: Vec<(String, String)>,
    body: Option<Value>,
    options: RequestOptions,
}

impl<'a> OperationBuilder<'a> {
    pub fn path_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.path_params.insert(key.into(), value.into());
        self
    }

    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.push((key.into(), value.into()));
        self
    }

    pub fn body_value(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    pub fn body_json<T: Serialize>(mut self, body: &T) -> Result<Self, Error> {
        self.body = Some(serde_json::to_value(body)?);
        Ok(self)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }

    pub fn tenant_key(mut self, tenant_key: impl Into<String>) -> Self {
        self.options.tenant_key = Some(tenant_key.into());
        self
    }

    pub fn app_ticket(mut self, app_ticket: impl Into<String>) -> Self {
        self.options.app_ticket = Some(app_ticket.into());
        self
    }

    pub fn user_access_token(mut self, access_token: impl Into<String>) -> Self {
        self.options.user_access_token = Some(access_token.into());
        self
    }

    pub fn tenant_access_token(mut self, access_token: impl Into<String>) -> Self {
        self.options.tenant_access_token = Some(access_token.into());
        self
    }

    pub fn app_access_token(mut self, access_token: impl Into<String>) -> Self {
        self.options.app_access_token = Some(access_token.into());
        self
    }

    pub fn header(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Result<Self, Error> {
        let header_name: HeaderName = key.as_ref().parse()?;
        let header_value = HeaderValue::from_str(value.as_ref())?;
        self.options.headers.insert(header_name, header_value);
        Ok(self)
    }

    pub async fn send(self) -> Result<ApiResponse, Error> {
        self.client
            .call(
                &self.operation_id,
                self.path_params,
                self.query,
                self.body,
                self.options,
            )
            .await
    }

    #[cfg(test)]
    #[allow(clippy::type_complexity)]
    pub(crate) fn into_inner(
        self,
    ) -> (
        String,
        HashMap<String, String>,
        Vec<(String, String)>,
        Option<Value>,
        RequestOptions,
    ) {
        (
            self.operation_id,
            self.path_params,
            self.query,
            self.body,
            self.options,
        )
    }
}

fn http_method(method: &str) -> Result<Method, Error> {
    match method {
        "GET" => Ok(Method::GET),
        "POST" => Ok(Method::POST),
        "PUT" => Ok(Method::PUT),
        "PATCH" => Ok(Method::PATCH),
        "DELETE" => Ok(Method::DELETE),
        other => Err(Error::InvalidHttpMethod(other.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Config;

    #[test]
    fn operation_builder_collects_parts() {
        let client = Client::new(Config::builder("app", "secret").build()).expect("client");
        let builder = client
            .operation("im.v1.chat.get")
            .path_param("chat_id", "oc_123")
            .query_param("user_id_type", "open_id")
            .tenant_key("tenant_1");

        let (operation_id, path_params, query, body, options) = builder.into_inner();
        assert_eq!(operation_id, "im.v1.chat.get");
        assert_eq!(
            path_params.get("chat_id").map(String::as_str),
            Some("oc_123")
        );
        assert_eq!(
            query,
            vec![("user_id_type".to_string(), "open_id".to_string())]
        );
        assert!(body.is_none());
        assert_eq!(options.tenant_key.as_deref(), Some("tenant_1"));
    }
}
