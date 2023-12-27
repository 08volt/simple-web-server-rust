use std::fmt::Display;

use axum::{
    http::{response, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;
use serde_with::SerializeDisplay;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,

    // -- Auth errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,

    // -- Model errors
    TicketDeleteFailNotFound { id: u64 },
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Error::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // -- Auth
            Error::AuthFailNoAuthTokenCookie
            | Error::AuthFailTokenWrongFormat
            | Error::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- Model
            Error::TicketDeleteFailNotFound { id } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            // -- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // Create a placeholder Acum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
