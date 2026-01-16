use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Path, Request},
};
use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::error::CompositeError;
use common::{ErrorCode, ValidateFrom};

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub format: Option<String>,
}

#[derive(Debug)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: ValidateFrom + Send,
    T::Input: DeserializeOwned + Send,
{
    type Rejection = CompositeError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(input) = Json::<T::Input>::from_request(req, state)
            .await
            .map_err(|e| {
                CompositeError::new(ErrorCode::Ebadrequest, &format!("Invalid JSON format: {e}"))
            })?;

        let validated = T::validate_from(input)?;
        Ok(Self(validated))
    }
}

pub struct ValidatedPath<T>(pub T);

impl<T, S> FromRequestParts<S> for ValidatedPath<T>
where
    S: Send + Sync,
    T: ValidateFrom + Send,
    T::Input: DeserializeOwned + Send,
{
    type Rejection = CompositeError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Path(input) = Path::<T::Input>::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                CompositeError::new(
                    ErrorCode::Ebadrequest,
                    &format!("Invalid path parameter: {e}"),
                )
            })?;

        let validated = T::validate_from(input)?;
        Ok(Self(validated))
    }
}
