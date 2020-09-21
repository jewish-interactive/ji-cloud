use crate::db::{self, user::register};
use crate::extractor::{
    reply_signin_auth, FirebaseUser, WrapAuthClaimsCookieDbNoCsrf, WrapAuthClaimsNoDb,
};
use actix_web::{
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use core::settings::RuntimeSettings;
use jsonwebtoken as jwt;
use shared::{
    api::endpoints::{
        user::{Profile, Register, Signin, SingleSignOn},
        ApiEndpoint,
    },
    domain::auth::{
        AuthClaims, RegisterRequest, RegisterSuccess, SigninSuccess, SingleSignOnSuccess,
    },
    domain::user::OtherUser,
    error::auth::RegisterError,
    error::user::NoSuchUserError,
    error::InternalServerError,
};
use sqlx::PgPool;

async fn user_by_name(
    db: Data<PgPool>,
    username: Path<String>,
) -> Result<Option<OtherUser>, InternalServerError> {
    Ok(db::user::by_name(db.as_ref(), &username.into_inner()).await?)
}

async fn handle_signin_credentials(
    settings: Data<RuntimeSettings>,
    db: Data<PgPool>,
    user: FirebaseUser,
) -> actix_web::Result<HttpResponse> {
    let user_id = db::user::firebase_to_id(&db, &user.id)
        .await
        .map_err(|_| HttpResponse::InternalServerError())?
        .ok_or_else(|| HttpResponse::UnprocessableEntity().json(NoSuchUserError {}))?;

    let (csrf, cookie) =
        reply_signin_auth(user_id, &settings.jwt_encoding_key, settings.is_local())
            .map_err(|_| HttpResponse::InternalServerError())?;

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(SigninSuccess { csrf }))
}

async fn validate_register_req(req: &RegisterRequest) -> Result<(), RegisterError> {
    // todo: decide if we should check for an _empty_ email?
    if req.username.is_empty() {
        return Err(RegisterError::EmptyDisplayName);
    }

    Ok(())
}

async fn handle_register(
    settings: Data<RuntimeSettings>,
    db: Data<PgPool>,
    user: FirebaseUser,
    req: Json<RegisterRequest>,
) -> actix_web::Result<HttpResponse, RegisterError> {
    validate_register_req(&req).await?;

    let id = register(db.as_ref(), &user.id, &req).await?;

    let (csrf, cookie) = reply_signin_auth(id, &settings.jwt_encoding_key, settings.is_local())?;

    Ok(HttpResponse::Created()
        .cookie(cookie)
        .json(RegisterSuccess::Signin(csrf)))
}

async fn handle_get_profile(
    db: Data<PgPool>,
    claims: WrapAuthClaimsNoDb,
) -> actix_web::Result<Json<<Profile as ApiEndpoint>::Res>> {
    // todo: figure out how to do `<Profile as ApiEndpoint>::Err`

    db::user::profile(db.as_ref(), claims.0.id)
        .await
        .map_err(|_| HttpResponse::InternalServerError())?
        .map(Json)
        .ok_or(HttpResponse::NotFound().json(NoSuchUserError {}).into())
}

async fn handle_authorize(
    settings: Data<RuntimeSettings>,
    auth: WrapAuthClaimsCookieDbNoCsrf,
) -> actix_web::Result<Json<<SingleSignOn as ApiEndpoint>::Res>> {
    let claims = AuthClaims {
        id: auth.0.id,
        csrf: None,
    };

    let jwt = jwt::encode(&jwt::Header::default(), &claims, &settings.jwt_encoding_key)
        .map_err(|_| HttpResponse::InternalServerError())?;

    Ok(Json(SingleSignOnSuccess { jwt }))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        Profile::PATH,
        Profile::METHOD.route().to(handle_get_profile),
    )
    .route(
        SingleSignOn::PATH,
        SingleSignOn::METHOD.route().to(handle_authorize),
    )
    .route(Register::PATH, Register::METHOD.route().to(handle_register))
    .route(
        Signin::PATH,
        Signin::METHOD.route().to(handle_signin_credentials),
    );
}
