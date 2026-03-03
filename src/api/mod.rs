pub mod all_services;
mod auth_v3;
mod common;
mod contact_v3_user;
mod generated_models;
mod im_v1_chat;

pub use auth_v3::{
    AppAccessTokenResp, AuthV3Api, MarketplaceAppAccessTokenReq, MarketplaceTenantAccessTokenReq,
    ResendAppTicketReq, ResendAppTicketResp, SelfBuiltAppAccessTokenReq,
    SelfBuiltTenantAccessTokenReq, TenantAccessTokenResp,
};
pub use common::ApiEnvelope;
pub use contact_v3_user::{
    ContactUserInfo, ContactV3UserApi, GetContactUserQuery, ListContactUserQuery,
};
pub use generated_models::*;
pub use im_v1_chat::{
    ChatInfo, CreateChatBody, CreateChatQuery, DeleteChatQuery, GetChatQuery, ImV1ChatApi,
    ListChatQuery,
};

use crate::Client;

impl Client {
    pub fn auth_v3(&self) -> AuthV3Api<'_> {
        AuthV3Api::new(self)
    }

    pub fn im_v1_chat(&self) -> ImV1ChatApi<'_> {
        ImV1ChatApi::new(self)
    }

    pub fn contact_v3_user(&self) -> ContactV3UserApi<'_> {
        ContactV3UserApi::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Config;
    use crate::generated::ops;

    #[test]
    fn all_services_wrapper_builds_expected_operation() {
        let client = Client::new(Config::builder("app", "secret").build()).expect("client");
        let builder = all_services::im::v1::chat::get(&client, "oc_123")
            .query_param("user_id_type", "open_id");

        let (operation_id, path_params, query, ..) = builder.into_inner();
        assert_eq!(operation_id, ops::im::v1::chat::GET);
        assert_eq!(
            path_params.get("chat_id").map(String::as_str),
            Some("oc_123")
        );
        assert_eq!(
            query,
            vec![("user_id_type".to_string(), "open_id".to_string())]
        );
    }

    #[test]
    fn all_services_wrapper_handles_multiple_path_params() {
        let client = Client::new(Config::builder("app", "secret").build()).expect("client");
        let builder = all_services::admin::v1::badge_grant::get(&client, "badge_1", "grant_2");

        let (operation_id, path_params, ..) = builder.into_inner();
        assert_eq!(operation_id, ops::admin::v1::badge_grant::GET);
        assert_eq!(
            path_params.get("badge_id").map(String::as_str),
            Some("badge_1")
        );
        assert_eq!(
            path_params.get("grant_id").map(String::as_str),
            Some("grant_2")
        );
    }
}
