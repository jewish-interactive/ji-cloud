use crate::{
    jwkkeys::{self, JwkVerifier},
    jwt::{check_no_csrf, check_no_db},
};
use actix_web::{
    cookie::{Cookie, CookieBuilder, SameSite},
    http::{header, HeaderMap, HeaderValue},
    web::Data,
    FromRequest, HttpMessage, HttpResponse,
};
use config::{COOKIE_DOMAIN, MAX_SIGNIN_COOKIE_DURATION};
use core::settings::Settings;
use futures::future::FutureExt;
use futures_util::future::BoxFuture;
use jsonwebtoken as jwt;
use jwt::EncodingKey;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use shared::domain::auth::{AuthClaims, CSRF_HEADER_NAME, JWT_COOKIE_NAME};
use shared::error::auth::FirebaseError;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

fn try_insecure_decode(token: &str) -> Option<FirebaseId> {
    let claims: jwkkeys::Claims = jsonwebtoken::dangerous_insecure_decode(&token).ok()?.claims;
    let user_id = claims.sub;
    Some(FirebaseId(user_id))
}

pub struct FirebaseUser {
    pub id: FirebaseId,
}

pub struct FirebaseId(pub String);

// stolen from the stdlib and modified (to work on stable)
fn split_once<'a>(s: &'a str, delimiter: char) -> Option<(&'a str, &'a str)> {
    let start = s.find(delimiter)?;
    let end = start + delimiter.len_utf8();
    Some((&s[..start], &s[end..]))
}

fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    let header: &HeaderValue = headers.get(header::AUTHORIZATION)?;

    split_once(header.to_str().ok()?, ' ')
        .filter(|(kind, _)| kind.eq_ignore_ascii_case("bearer"))
        .map(|(_, token)| token)
}

pub struct AuthError;

impl From<AuthError> for actix_web::Error {
    fn from(_other: AuthError) -> Self {
        HttpResponse::Unauthorized().into()
    }
}

pub enum StatusError {
    Auth,
    InternalServerError,
}

impl From<StatusError> for actix_web::Error {
    fn from(other: StatusError) -> Self {
        match other {
            StatusError::Auth => HttpResponse::Unauthorized().into(),
            StatusError::InternalServerError => HttpResponse::InternalServerError().into(),
        }
    }
}

impl FromRequest for FirebaseUser {
    type Error = FirebaseError;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let settings: &Data<Settings> = req.app_data().unwrap();
        let settings = settings.clone();
        let jwk_verifier: &Arc<RwLock<JwkVerifier>> = req.app_data().unwrap();
        let jwk_verifier = jwk_verifier.clone();

        // this whole dance is to avoid cloning the headers.
        let token = match bearer_token(req.headers()) {
            Some(token) => token.to_owned(),
            None => return futures::future::err(FirebaseError::MissingBearerToken).boxed(),
        };

        // HACK for testing don't verify external auth, just it's valid.
        if settings.local_no_auth {
            return futures::future::ready(
                try_insecure_decode(&token)
                    .map(|id| Self { id })
                    .ok_or_else(|| FirebaseError::InvalidToken),
            )
            .boxed();
        }

        async move {
            // todo: more specific errors.
            let id = jwk_verifier
                .read()
                .await
                .verify(&token)
                .map_err(|_| FirebaseError::InvalidToken)?;

            Ok(Self { id })
        }
        .boxed()
    }
}

fn csrf_header(headers: &HeaderMap) -> Option<&str> {
    headers.get(CSRF_HEADER_NAME)?.to_str().ok()
}

#[repr(transparent)]
pub struct WrapAuthClaimsNoDb(pub AuthClaims);

impl FromRequest for WrapAuthClaimsNoDb {
    type Error = AuthError;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let cookie = req.cookie(JWT_COOKIE_NAME);
        let csrf = csrf_header(req.headers());
        let settings: &Data<Settings> = req.app_data().expect("Settings??");

        let (cookie, csrf) = match (cookie, csrf) {
            (Some(cookie), Some(csrf)) => (cookie, csrf),
            _ => return futures::future::err(AuthError),
        };

        futures::future::ready(
            check_no_db(cookie.value(), csrf, &settings.jwt_decoding_key)
                .map_err(|_| AuthError)
                .and_then(|it| it.ok_or(AuthError))
                .map(Self),
        )
    }
}

#[repr(transparent)]
pub struct WrapAuthClaimsCookieDbNoCsrf(pub AuthClaims);

impl FromRequest for WrapAuthClaimsCookieDbNoCsrf {
    type Error = StatusError;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let db: &Data<PgPool> = req.app_data().unwrap();
        let db = db.as_ref().clone();
        let settings: &Data<Settings> = req.app_data().unwrap();
        let settings = settings.clone();

        let cookie = match req.cookie(JWT_COOKIE_NAME) {
            Some(token) => token.to_owned(),
            None => return futures::future::err(StatusError::Auth).boxed(),
        };

        async move {
            check_no_csrf(&db, &cookie.value(), &settings.jwt_decoding_key)
                .await
                .map_err(|_| StatusError::InternalServerError)?
                .map(Self)
                .ok_or(StatusError::Auth)
        }
        .boxed()
    }
}

pub fn reply_signin_auth(
    user_id: Uuid,
    jwt_encoding_key: &EncodingKey,
    local_insecure: bool,
) -> anyhow::Result<(String, Cookie<'static>)> {
    let csrf: String = thread_rng().sample_iter(&Alphanumeric).take(16).collect();

    let claims = AuthClaims {
        id: user_id,
        csrf: Some(csrf.clone()),
    };

    let jwt = jwt::encode(&jwt::Header::default(), &claims, jwt_encoding_key)?;

    let mut cookie = CookieBuilder::new(JWT_COOKIE_NAME, jwt)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(MAX_SIGNIN_COOKIE_DURATION);

    if !local_insecure {
        cookie = cookie.domain(COOKIE_DOMAIN);
    }

    Ok((csrf, cookie.finish()))
}
