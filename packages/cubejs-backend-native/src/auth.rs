use async_trait::async_trait;
use cubesql::{
    di_service,
    sql::{AuthContext, AuthenticateResponse, SqlAuthService},
    transport::LoadRequestMeta,
    CubeError,
};
use log::trace;
use neon::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::sync::Arc;
use uuid::Uuid;

use crate::channel::call_js_with_channel_as_callback;

#[derive(Debug)]
pub struct NodeBridgeAuthService {
    channel: Arc<Channel>,
    check_sql_auth: Arc<Root<JsFunction>>,
}

impl NodeBridgeAuthService {
    pub fn new(channel: Channel, check_sql_auth: Root<JsFunction>) -> Self {
        Self {
            channel: Arc::new(channel),
            check_sql_auth: Arc::new(check_sql_auth),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TransportRequest {
    pub id: String,
    pub meta: Option<LoadRequestMeta>,
}

#[derive(Debug, Serialize)]
struct CheckSQLAuthRequest {
    request: TransportRequest,
    user: Option<String>,
    password: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CheckSQLAuthResponse {
    password: Option<String>,
    superuser: bool,
    #[serde(rename = "securityContext", skip_serializing_if = "Option::is_none")]
    security_context: Option<serde_json::Value>,
    #[serde(rename = "skipPasswordCheck", skip_serializing_if = "Option::is_none")]
    skip_password_check: Option<bool>,
}

#[derive(Debug)]
pub struct NativeAuthContext {
    pub user: Option<String>,
    pub superuser: bool,
    pub security_context: Option<serde_json::Value>,
}

impl AuthContext for NativeAuthContext {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[async_trait]
impl SqlAuthService for NodeBridgeAuthService {
    async fn authenticate(
        &self,
        user: Option<String>,
        password: Option<String>,
    ) -> Result<AuthenticateResponse, CubeError> {
        trace!("[auth] Request ->");

        let request_id = Uuid::new_v4().to_string();

        let extra = serde_json::to_string(&CheckSQLAuthRequest {
            request: TransportRequest {
                id: format!("{}-span-1", request_id),
                meta: None,
            },
            user: user.clone(),
            password: password.clone(),
        })?;
        let response: CheckSQLAuthResponse = call_js_with_channel_as_callback(
            self.channel.clone(),
            self.check_sql_auth.clone(),
            Some(extra),
        )
        .await?;
        trace!("[auth] Request <- {:?}", response);

        Ok(AuthenticateResponse {
            context: Arc::new(NativeAuthContext {
                user,
                superuser: response.superuser,
                security_context: response.security_context,
            }),
            password: response.password,
            skip_password_check: response.skip_password_check.unwrap_or(false),
        })
    }
}

di_service!(NodeBridgeAuthService, [SqlAuthService]);
