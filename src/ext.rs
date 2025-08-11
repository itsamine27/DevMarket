use crate::user::Clains;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;

pub struct IsAuth(pub Clains);

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for IsAuth
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(header_value) = parts.headers.get("Authorization") {
            if let Ok(token_str) = header_value.to_str() {
                if let Ok(claims) = Clains::from_token(token_str) {
                    return Ok(IsAuth(claims));
                }
            }
        }

        Err((
            StatusCode::UNAUTHORIZED,
            "You're not allowed to be in here, please login",
        ))
    }
}
