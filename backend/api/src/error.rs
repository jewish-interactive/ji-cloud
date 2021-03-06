//! Contains error types for api operations. Enums are declared with HTTP status codes
//! indicating the type of error with optional descriptions.
//!
//! Casting and handling of:
//!     * `sqlx::Error` -- database errors generated in database interactions, mostly in `db` module
//!     * `actix_web::Error` -- server errors, used by actix for error logging
//!     * `anyhow::Error` -- general intermediate error representation

use actix_web::error::BlockingError;
use actix_web::HttpResponse;
use paperclip::actix::api_v2_errors;
use shared::error::{ApiError, EmptyError, MetadataNotFound};

use crate::db::meta::MetaWrapperError;

mod oauth;
pub use oauth::{GoogleOAuth, OAuth};

mod storage;
pub use storage::Storage;

pub mod event_arc;
pub use event_arc::EventArc;
use http::StatusCode;

/// Represents an error returned by the api.
// mostly used in this module
#[allow(clippy::clippy::module_name_repetitions)]
pub type BasicError = ApiError<EmptyError>;

#[non_exhaustive]
#[api_v2_errors(code = 401, code = 403, code = 404, code = 500)]
pub enum Auth {
    InternalServerError(anyhow::Error),
    Forbidden,
}

impl<T: Into<anyhow::Error>> From<T> for Auth {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for Auth {
    fn into(self) -> actix_web::Error {
        match self {
            Self::Forbidden => BasicError::new(http::StatusCode::FORBIDDEN).into(),
            Self::InternalServerError(e) => ise(e),
        }
    }
}

pub fn ise(e: anyhow::Error) -> actix_web::Error {
    let mut resp = HttpResponse::InternalServerError();
    resp.extensions_mut().insert(e);
    resp.json(BasicError::new(http::StatusCode::INTERNAL_SERVER_ERROR))
        .into()
}

#[non_exhaustive]
#[api_v2_errors(code = 401, code = 403, code = 404, code = 500)]
pub enum Delete {
    Conflict,
    Forbidden,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for Delete {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl From<Auth> for Delete {
    fn from(e: Auth) -> Self {
        match e {
            Auth::InternalServerError(e) => Self::InternalServerError(e),
            Auth::Forbidden => Self::Forbidden,
        }
    }
}

impl Into<actix_web::Error> for Delete {
    fn into(self) -> actix_web::Error {
        match self {
            Self::Conflict => BasicError::new(http::StatusCode::CONFLICT).into(),
            Self::Forbidden => BasicError::new(http::StatusCode::FORBIDDEN).into(),
            Self::InternalServerError(e) => ise(e),
        }
    }
}

#[api_v2_errors(code = 400, code = 401, code = 403, code = 500)]
pub struct Server(pub anyhow::Error);

impl<T: Into<anyhow::Error>> From<T> for Server {
    fn from(e: T) -> Self {
        Self(e.into())
    }
}

impl Into<actix_web::Error> for Server {
    fn into(self) -> actix_web::Error {
        ise(self.0)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ServiceKind {
    Algolia,
    S3,
    GoogleCloudStorage,
    GoogleEventArc,
    GoogleOAuth,
    Mail,
    FirebaseCloudMessaging,
}

impl ServiceKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Algolia => "Algolia",
            Self::S3 => "S3",
            Self::GoogleCloudStorage => "Google Cloud Storage",
            Self::GoogleEventArc => "Google EventArc",
            Self::GoogleOAuth => "Google OAuth",
            Self::Mail => "Sendgrid Mail",
            Self::FirebaseCloudMessaging => "Firebase Cloud Messaging",
        }
    }
}

impl Into<actix_web::Error> for ServiceKind {
    fn into(self) -> actix_web::Error {
        BasicError::with_message(
            http::StatusCode::NOT_IMPLEMENTED,
            format!("{} is disabled", self.as_str()).to_owned(),
        )
        .into()
    }
}

#[api_v2_errors(code = 400, code = 401, code = 403, code = 500, code = 501)]
#[derive(Debug)]
pub enum Service {
    InternalServerError(anyhow::Error),
    DisabledService(ServiceKind),
}

impl<T: Into<anyhow::Error>> From<T> for Service {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for Service {
    fn into(self) -> actix_web::Error {
        match self {
            Self::InternalServerError(e) => ise(e),
            Self::DisabledService(s) => s.into(),
        }
    }
}

#[api_v2_errors(code = 400, code = 401, code = 403, code = 500, code = 501)]
#[derive(Debug)]
pub enum ServiceSession {
    InternalServerError(anyhow::Error),
    DisabledService(ServiceKind),
    Unauthorized,
}

impl<T: Into<anyhow::Error>> From<T> for ServiceSession {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for ServiceSession {
    fn into(self) -> actix_web::Error {
        match self {
            Self::InternalServerError(e) => ise(e),
            Self::DisabledService(s) => s.into(),
            Self::Unauthorized => BasicError::new(http::StatusCode::UNAUTHORIZED).into(),
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 401,
    code = 403,
    code = 404,
    description = "Not Found: Resource Not Found",
    code = 412,
    code = 500,
    code = 501
)]
#[derive(Debug)]
pub enum Refresh {
    InternalServerError(anyhow::Error),
    PreconditionFailed,
    ResourceNotFound,
}

impl<T: Into<anyhow::Error>> From<T> for Refresh {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for Refresh {
    fn into(self) -> actix_web::Error {
        match self {
            Self::InternalServerError(e) => ise(e),
            Self::PreconditionFailed => {
                BasicError::new(http::StatusCode::PRECONDITION_FAILED).into()
            }
            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource Not Found".to_owned(),
            )
            .into(),
        }
    }
}

#[api_v2_errors(
    code = 401,
    code = 403,
    code = 404,
    description = "Not Found: User not Found",
    code = 500
)]
pub enum UserNotFound {
    UserNotFound,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for UserNotFound {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for UserNotFound {
    fn into(self) -> actix_web::Error {
        match self {
            Self::UserNotFound => {
                BasicError::with_message(http::StatusCode::NOT_FOUND, "User Not Found".to_owned())
                    .into()
            }
            Self::InternalServerError(e) => ise(e),
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 401,
    code = 403,
    code = 404,
    description = "Not Found: Resource Not Found",
    code = 500
)]
pub enum NotFound {
    ResourceNotFound,
    Forbidden,
    InternalServerError(anyhow::Error),
}

impl From<Auth> for NotFound {
    fn from(e: Auth) -> Self {
        match e {
            Auth::InternalServerError(e) => Self::InternalServerError(e),
            Auth::Forbidden => Self::Forbidden,
        }
    }
}

impl<T: Into<anyhow::Error>> From<T> for NotFound {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for NotFound {
    fn into(self) -> actix_web::Error {
        match self {
            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource Not Found".to_owned(),
            )
            .into(),
            Self::Forbidden => BasicError::new(http::StatusCode::FORBIDDEN).into(),
            Self::InternalServerError(e) => ise(e),
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 401,
    code = 403,
    code = 404,
    description = "Not Found: Parent Category Not Found OR category not found",
    code = 420,
    description = "Unprocessable Entity: Cycle OR OutOfRange",
    code = 500
)]
pub enum CategoryUpdate {
    CategoryNotFound,
    ParentCategoryNotFound,
    Cycle,
    OutOfRange,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for CategoryUpdate {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for CategoryUpdate {
    fn into(self) -> actix_web::Error {
        match self {
            Self::CategoryNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Category Not Found".to_owned(),
            )
            .into(),

            Self::ParentCategoryNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Parent Category Not Found".to_owned(),
            )
            .into(),

            Self::Cycle => BasicError::with_message(
                http::StatusCode::UNPROCESSABLE_ENTITY,
                "Would cause a cycle".to_owned(),
            )
            .into(),

            Self::OutOfRange => BasicError::with_message(
                http::StatusCode::UNPROCESSABLE_ENTITY,
                "Out of range".to_owned(),
            )
            .into(),

            Self::InternalServerError(e) => ise(e),
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 401,
    code = 403,
    code = 404,
    description = "Not Found: Resource Not Found",
    code = 420,
    description = "Unprocessable Entity: Invalid Content",
    code = 500
)]
#[derive(Debug)]
pub enum Upload {
    ResourceNotFound,
    InvalidMedia,
    FileTooLarge,
    StorageClient(Storage),
    InternalServerError(anyhow::Error),
}

impl Upload {
    #[allow(dead_code)]
    pub fn blocking_error(err: BlockingError<Self>) -> Self {
        match err {
            BlockingError::Canceled => anyhow::anyhow!("Thread pool is gone").into(),
            BlockingError::Error(e) => e,
        }
    }
}

impl<T: Into<anyhow::Error>> From<T> for Upload {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for Upload {
    fn into(self) -> actix_web::Error {
        match self {
            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource Not Found".to_owned(),
            )
            .into(),
            Self::InvalidMedia => BasicError::with_message(
                http::StatusCode::UNPROCESSABLE_ENTITY,
                "Invalid Content".to_owned(),
            )
            .into(),
            Self::FileTooLarge => BasicError::with_message(
                http::StatusCode::PAYLOAD_TOO_LARGE,
                "File Exceeds Upload Limit".to_owned(),
            )
            .into(),
            Self::StorageClient(e) => e.into(),
            Self::InternalServerError(e) => ise(e),
        }
    }
}

impl From<Storage> for Upload {
    fn from(e: Storage) -> Self {
        match e {
            Storage::FileTooLarge => Upload::FileTooLarge,
            Storage::InternalServerError(e) => Upload::InternalServerError(e),
            e => Upload::StorageClient(e),
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 401,
    code = 403,
    code = 420,
    description = "Unprocessable Entity: Metadata not Found",
    code = 500
)]
pub enum CreateWithMetadata {
    InternalServerError(anyhow::Error),
    Forbidden,
    MissingMetadata(MetadataNotFound),
}

impl From<Auth> for CreateWithMetadata {
    fn from(e: Auth) -> Self {
        match e {
            Auth::InternalServerError(e) => Self::InternalServerError(e),
            Auth::Forbidden => Self::Forbidden,
        }
    }
}

impl<T: Into<anyhow::Error>> From<T> for CreateWithMetadata {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for CreateWithMetadata {
    fn into(self) -> actix_web::Error {
        match self {
            Self::MissingMetadata(data) => ApiError {
                code: http::StatusCode::UNPROCESSABLE_ENTITY,
                message: "Metadata not Found".to_owned(),
                extra: data,
            }
            .into(),
            Self::Forbidden => BasicError::new(http::StatusCode::FORBIDDEN).into(),
            Self::InternalServerError(e) => ise(e),
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 401,
    code = 403,
    code = 404,
    description = "Not Found: Resource Not Found",
    code = 420,
    description = "Unprocessable Entity: Metadata not Found",
    code = 500
)]
pub enum UpdateWithMetadata {
    ResourceNotFound,
    InternalServerError(anyhow::Error),
    MissingMetadata(MetadataNotFound),
    Forbidden,
}

impl From<Auth> for UpdateWithMetadata {
    fn from(e: Auth) -> Self {
        match e {
            Auth::InternalServerError(e) => Self::InternalServerError(e),
            Auth::Forbidden => Self::Forbidden,
        }
    }
}

impl<T: Into<anyhow::Error>> From<T> for UpdateWithMetadata {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for UpdateWithMetadata {
    fn into(self) -> actix_web::Error {
        match self {
            Self::MissingMetadata(data) => ApiError {
                code: http::StatusCode::UNPROCESSABLE_ENTITY,
                message: "Metadata not Found".to_owned(),
                extra: data,
            }
            .into(),

            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource Not Found".to_owned(),
            )
            .into(),

            Self::Forbidden => BasicError::new(http::StatusCode::FORBIDDEN).into(),

            Self::InternalServerError(e) => ise(e),
        }
    }
}

impl From<MetaWrapperError> for CreateWithMetadata {
    fn from(e: MetaWrapperError) -> Self {
        match e {
            MetaWrapperError::Sqlx(e) => Self::InternalServerError(e.into()),
            MetaWrapperError::MissingMetadata { id, kind } => {
                Self::MissingMetadata(MetadataNotFound { id, kind })
            }
        }
    }
}

impl From<MetaWrapperError> for UpdateWithMetadata {
    fn from(e: MetaWrapperError) -> Self {
        match e {
            MetaWrapperError::Sqlx(e) => Self::InternalServerError(e.into()),
            MetaWrapperError::MissingMetadata { id, kind } => {
                Self::MissingMetadata(MetadataNotFound { id, kind })
            }
        }
    }
}

#[api_v2_errors(code = 400, code = 409, code = 420, code = 500, code = 501)]
pub enum Register {
    InternalServerError(anyhow::Error),
    Username(RegisterUsername),
    VerifyEmail(VerifyEmail),
    Service(Service),
}

impl<T: Into<anyhow::Error>> From<T> for Register {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for Register {
    fn into(self) -> actix_web::Error {
        match self {
            Self::InternalServerError(e) => ise(e),
            Self::Username(e) => e.into(),
            Self::VerifyEmail(e) => e.into(),
            Self::Service(e) => e.into(),
        }
    }
}

impl From<RegisterUsername> for Register {
    fn from(err: RegisterUsername) -> Self {
        Self::Username(err)
    }
}

impl From<VerifyEmail> for Register {
    fn from(err: VerifyEmail) -> Self {
        Self::VerifyEmail(err)
    }
}

impl From<Service> for Register {
    fn from(err: Service) -> Self {
        Self::Service(err)
    }
}

impl From<Email> for Register {
    fn from(err: Email) -> Self {
        Self::VerifyEmail(err.into())
    }
}

#[api_v2_errors(
    code = 400,
    code = 409,
    description = "Conflict: Another user with the provided username already exists",
    code = 420,
    description = "Unprocessable Entity: No username was provided",
    code = 500,
    code = 501
)]
pub enum RegisterUsername {
    EmptyUsername,
    TakenUsername,
    InternalServerError(anyhow::Error),
    Service(Service),
}

impl<T: Into<anyhow::Error>> From<T> for RegisterUsername {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for RegisterUsername {
    fn into(self) -> actix_web::Error {
        match self {
            Self::EmptyUsername => BasicError::with_message(
                http::StatusCode::UNPROCESSABLE_ENTITY,
                "No username was provided".to_owned(),
            )
            .into(),

            Self::TakenUsername => BasicError::with_message(
                http::StatusCode::CONFLICT,
                "Username already taken".to_owned(),
            )
            .into(),

            Self::InternalServerError(e) => ise(e),

            Self::Service(e) => e.into(),
        }
    }
}

impl From<Service> for RegisterUsername {
    fn from(err: Service) -> Self {
        Self::Service(err)
    }
}

#[api_v2_errors(
    code = 400,
    code = 403,
    code = 404,
    description = "Not Found: Resource Not Found",
    code = 409,
    description = "Conflict: Another tag with the provided index already exists",
    code = 500
)]
pub enum Tag {
    TakenIndex,
    ResourceNotFound,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for Tag {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for Tag {
    fn into(self) -> actix_web::Error {
        match self {
            Self::TakenIndex => BasicError::with_message(
                http::StatusCode::CONFLICT,
                "Tag index already taken".to_owned(),
            )
            .into(),

            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource Not Found".to_owned(),
            )
            .into(),

            Self::InternalServerError(e) => ise(e),
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 404,
    code = 409,
    description = "Conflict: an image with the same ID already exists for this user",
    code = 500
)]
pub enum UserRecentImage {
    ResourceNotFound,
    Conflict,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for UserRecentImage {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for UserRecentImage {
    fn into(self) -> actix_web::Error {
        match self {
            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Image not found in recent images list for user".to_owned(),
            )
            .into(),

            Self::Conflict => BasicError::with_message(
                http::StatusCode::CONFLICT,
                "An image with the same ID already exists in recent images list for user"
                    .to_owned(),
            )
            .into(),

            Self::InternalServerError(e) => ise(e),
        }
    }
}

#[api_v2_errors(
    code = 404,
    code = 404,
    code = 409,
    description = "Conflict: a draft already exists for this jig",
    code = 500
)]
pub enum JigCloneDraft {
    ResourceNotFound,
    IsDraft,
    Conflict,
    Forbidden,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for JigCloneDraft {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl From<Auth> for JigCloneDraft {
    fn from(e: Auth) -> Self {
        match e {
            Auth::InternalServerError(e) => Self::InternalServerError(e),
            Auth::Forbidden => Self::Forbidden,
        }
    }
}

impl Into<actix_web::Error> for JigCloneDraft {
    fn into(self) -> actix_web::Error {
        match self {
            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource Not Found".to_owned(),
            )
            .into(),

            Self::IsDraft => BasicError::with_message(
                http::StatusCode::BAD_REQUEST,
                "Cannot create a draft from a draft".to_owned(),
            )
            .into(),

            Self::Conflict => BasicError::with_message(
                http::StatusCode::CONFLICT,
                "A draft already exists for this jig".to_owned(),
            )
            .into(),

            Self::Forbidden => BasicError::new(http::StatusCode::FORBIDDEN).into(),

            Self::InternalServerError(e) => ise(e),
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 409,
    description = "Conflict: Another user with the provided email already exists",
    code = 420,
    description = "Unprocessable Entity: No email was provided",
    code = 500,
    code = 501
)]
pub enum Email {
    InternalServerError(anyhow::Error),
    TakenEmailBasic,
    TakenEmailGoogle,
    EmptyEmail,
}

impl<T: Into<anyhow::Error>> From<T> for Email {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for Email {
    fn into(self) -> actix_web::Error {
        match self {
            Self::InternalServerError(e) => ise(e),
            Self::TakenEmailBasic | Self::TakenEmailGoogle => BasicError::with_message(
                StatusCode::CONFLICT,
                "A user with this email already exists".to_owned(),
            )
            .into(),
            Self::EmptyEmail => BasicError::with_message(
                StatusCode::UNPROCESSABLE_ENTITY,
                "No email address was provided".to_owned(),
            )
            .into(),
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 409,
    description = "Conflict: Another user with the provided username already exists",
    code = 420,
    description = "Unprocessable Entity: No username was provided",
    code = 500,
    code = 501
)]
pub enum VerifyEmail {
    InternalServerError(anyhow::Error),
    Email(Email),
    ServiceSession(ServiceSession),
}

impl<T: Into<anyhow::Error>> From<T> for VerifyEmail {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for VerifyEmail {
    fn into(self) -> actix_web::Error {
        match self {
            Self::InternalServerError(e) => ise(e),
            Self::Email(e) => e.into(),
            Self::ServiceSession(e) => e.into(),
        }
    }
}

impl From<ServiceSession> for VerifyEmail {
    fn from(err: ServiceSession) -> Self {
        Self::ServiceSession(err)
    }
}

impl From<Email> for VerifyEmail {
    fn from(err: Email) -> Self {
        Self::Email(err)
    }
}

#[api_v2_errors(
    code = 400,
    code = 409,
    description = "Conflict: Another user with the provided username already exists",
    code = 420,
    description = "Unprocessable Entity: No username was provided",
    code = 500,
    code = 501
)]
pub enum MediaProcessing {
    InternalServerError(anyhow::Error),
    EventArc(EventArc),
    ResourceNotFound,
}

impl<T: Into<anyhow::Error>> From<T> for MediaProcessing {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for MediaProcessing {
    fn into(self) -> actix_web::Error {
        match self {
            Self::InternalServerError(e) => ise(e),

            Self::EventArc(e) => e.into(),

            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource Not Found".to_owned(),
            )
            .into(),
        }
    }
}
