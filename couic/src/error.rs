use std::ops::{Deref, DerefMut};

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use common::{CompositeError as CompositeErrorBase, ErrorCode};

#[derive(Debug)]
pub struct CompositeError(pub CompositeErrorBase);

impl CompositeError {
    pub fn new(code: ErrorCode, message: &str) -> Self {
        Self(CompositeErrorBase::new(code, message))
    }

    pub fn to_status_code(&self) -> StatusCode {
        match self.0.code {
            ErrorCode::Eprocessing => StatusCode::ACCEPTED,
            ErrorCode::Eunauthorized => StatusCode::UNAUTHORIZED,
            ErrorCode::Enotfound => StatusCode::NOT_FOUND,
            ErrorCode::Econflict => StatusCode::CONFLICT,
            ErrorCode::Ebadrequest => StatusCode::BAD_REQUEST,
            ErrorCode::Einvalid => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorCode::Einternal => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::Enotimplemented => StatusCode::NOT_IMPLEMENTED,
        }
    }

    pub fn render_json(&self) -> Response {
        let status = self.to_status_code();
        (
            status,
            Json(json!({
                "code": self.0.code,
                "message": self.0.message,
                "errors": self.0.errors
            })),
        )
            .into_response()
    }
}

impl Deref for CompositeError {
    type Target = CompositeErrorBase;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CompositeError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<CompositeErrorBase> for CompositeError {
    fn from(e: CompositeErrorBase) -> Self {
        Self(e)
    }
}

impl std::fmt::Display for CompositeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for CompositeError {}

impl IntoResponse for CompositeError {
    fn into_response(self) -> Response {
        self.render_json()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_error_status_codes() {
        assert_eq!(
            CompositeError::new(ErrorCode::Eprocessing, "").to_status_code(),
            StatusCode::ACCEPTED
        );
        assert_eq!(
            CompositeError::new(ErrorCode::Eunauthorized, "").to_status_code(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            CompositeError::new(ErrorCode::Enotfound, "").to_status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            CompositeError::new(ErrorCode::Econflict, "").to_status_code(),
            StatusCode::CONFLICT
        );
        assert_eq!(
            CompositeError::new(ErrorCode::Einvalid, "").to_status_code(),
            StatusCode::UNPROCESSABLE_ENTITY
        );
        assert_eq!(
            CompositeError::new(ErrorCode::Einternal, "").to_status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            CompositeError::new(ErrorCode::Enotimplemented, "").to_status_code(),
            StatusCode::NOT_IMPLEMENTED
        );
    }

    #[test]
    fn test_composite_error_deref() {
        let mut error = CompositeError::new(ErrorCode::Einvalid, "Test error");
        error.add_detail("field1", ErrorCode::Einvalid, "Field1 is invalid");

        assert!(error.has_errors());
        assert_eq!(error.errors.len(), 1);
        assert_eq!(error.message, "Test error");
    }
}
