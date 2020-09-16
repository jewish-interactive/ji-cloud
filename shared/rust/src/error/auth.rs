//! Errors for Auth routes.

#[cfg(feature = "backend")]
use super::anyhow_to_ise;
#[cfg(feature = "backend")]
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
/// Represents an error with Registration.
pub enum RegisterError {
    /// No username was provided.
    EmptyDisplayName,

    /// Another user with the provided email already exists.
    TakenEmail,

    /// Another user with the provided firebase-id already exists
    TakenId,

    /// An internal server error occurred.
    #[serde(skip)]
    InternalServerError(anyhow::Error),
    // add todo: `TakenUserName` (also, EmptyDisplayName -> EmptyUserName)
}

#[cfg(feature = "backend")]
impl From<RegisterError> for actix_web::Error {
    fn from(e: RegisterError) -> actix_web::Error {
        match e {
            RegisterError::InternalServerError(e) => anyhow_to_ise(e),
            e => HttpResponse::UnprocessableEntity().json(e).into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
/// Represents an error with when authorizing a firebase token.
pub enum FirebaseError {
    /// The `Authorization` header didn't exist or didn't start with `Bearer` (case-insensitive).
    MissingBearerToken,

    /// The provided JWT token was invalid.
    InvalidToken,

    /// An internal server error occurred.
    #[serde(skip)]
    InternalServerError(anyhow::Error),
}

#[cfg(feature = "backend")]
impl From<FirebaseError> for actix_web::Error {
    fn from(e: FirebaseError) -> Self {
        match e {
            FirebaseError::InternalServerError(e) => anyhow_to_ise(e),
            e => HttpResponse::Unauthorized().json(e).into(),
        }
    }
}

from_anyhow![RegisterError, FirebaseError];
