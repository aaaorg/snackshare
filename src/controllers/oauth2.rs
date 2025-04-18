use crate::models::{o_auth2_sessions, users, users::EntraIDUserProfile};
use crate::views::auth::LoginResponse;
use axum::extract::Query;
use axum::Extension;
use axum_session::{Session, SessionNullPool};
use loco_oauth2::controllers::middleware::{OAuth2CookieUser, OAuth2PrivateCookieJar};
use loco_oauth2::controllers::oauth2::{callback, callback_jwt, get_authorization_url, AuthParams};
use loco_oauth2::OAuth2ClientStore;
use loco_rs::prelude::*;

/// The authorization URL for the `OAuth2` flow
/// This will redirect the user to the `OAuth2` provider's login page
/// and then to the callback URL

/// # Arguments
/// * `session` - The axum session
/// * `oauth_store` - The `OAuth2ClientStore` extension
/// # Returns
/// The HTML response with the link to the `OAuth2` provider's login page
/// # Errors
/// `loco_rs::errors::Error` - When the `OAuth2` client cannot be retrieved
pub async fn entraid_authorization_url(
    session: Session<SessionNullPool>,
    Extension(oauth2_store): Extension<OAuth2ClientStore>,
) -> Result<String> {
    // Get the `google` Authorization Code Grant client from the `OAuth2ClientStore`
    let mut client = oauth2_store
        .get_authorization_code_client("entraid")
        .await
        .map_err(|e| {
            tracing::error!("Error getting client: {:?}", e);
            Error::InternalServerError
        })?;
    // Get the authorization URL and save the csrf token in the session
    let auth_url = get_authorization_url(session, &mut client).await;
    drop(client);
    Ok(auth_url)
}

/// The callback URL for the `OAuth2` flow
/// This will exchange the code for a token and then get the user profile
/// then upsert the user and the session and set the token in a short live
/// cookie Lastly, it will redirect the user to the protected URL
/// # Arguments
/// * `ctx` - The application context
/// * `session` - The axum session
/// * `params` - The query parameters
/// * `jar` - The oauth2 private cookie jar
/// * `oauth_store` - The `OAuth2ClientStore` extension
/// # Returns
/// The response with the short live cookie and the redirect to the protected
/// URL
/// # Errors
/// * `loco_rs::errors::Error`
pub async fn entraid_callback_cookie(
    State(ctx): State<AppContext>,
    session: Session<SessionNullPool>,
    Query(params): Query<AuthParams>,
    // Extract the private cookie jar from the request
    jar: OAuth2PrivateCookieJar,
    Extension(oauth2_store): Extension<OAuth2ClientStore>,
) -> Result<impl IntoResponse> {
    println!("OAuth2 callback initiated successfully. Proceeding with token exchange and user session setup.");
    let mut client = oauth2_store
        .get_authorization_code_client("entraid")
        .await
        .map_err(|e| {
            tracing::error!("Error getting client: {:?}", e);
            Error::InternalServerError
        })?;
    // This function will validate the state from the callback. Then it will exchange the code for a token and then get the user profile. After that, the function will upsert the user and the session and set the token in a short live cookie and save the cookie in the private cookie jar. Lastly, the function will create a response with the short live cookie and the redirect to the protected URL
    let response = callback::<
        EntraIDUserProfile,
        users::Model,
        o_auth2_sessions::Model,
        SessionNullPool,
    >(ctx, session, params, jar, &mut client)
    .await?;
    drop(client);
    Ok(response)
}

/// The callback URL for the `OAuth2` flow
/// This will exchange the code for a token and then get the user profile
/// then upsert the user and the session and set the token in a short live
/// cookie Lastly, it will redirect the user to the protected URL
/// # Generics
/// * `T` - The user profile, should implement `DeserializeOwned` and `Send`
/// * `U` - The user model, should implement `OAuth2UserTrait` and `ModelTrait`
/// * `V` - The session model, should implement `OAuth2SessionsTrait` and `ModelTrait`
/// * `W` - The database pool
/// # Arguments
/// * `ctx` - The application context
/// * `session` - The axum session
/// * `params` - The query parameters
/// * `oauth2_store` - The `OAuth2ClientStore` extension
/// # Return
/// * `Result<impl IntoResponse>` - The response with the jwt token
/// # Errors
/// * `loco_rs::errors::Error`
pub async fn entraid_callback_jwt(
    State(ctx): State<AppContext>,
    session: Session<SessionNullPool>,
    Query(params): Query<AuthParams>,
    Extension(oauth2_store): Extension<OAuth2ClientStore>,
) -> Result<impl IntoResponse> {
    let mut client = oauth2_store
        .get_authorization_code_client("entraid")
        .await
        .map_err(|e| {
            tracing::error!("Error getting client: {:?}", e);
            Error::InternalServerError
        })?;
    // Get JWT secret from the config
    let jwt_secret = ctx.config.get_jwt_config()?;
    let user = callback_jwt::<
        EntraIDUserProfile,
        users::Model,
        o_auth2_sessions::Model,
        SessionNullPool,
    >(&ctx, session, params, &mut client)
    .await?;
    drop(client);
    let token = user
        .generate_jwt(&jwt_secret.secret, jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;
    // Return jwt token
    Ok(token)
}

async fn protected(
    State(ctx): State<AppContext>,
    // Extract the user from the Cookie via middleware
    user: OAuth2CookieUser<EntraIDUserProfile, users::Model, o_auth2_sessions::Model>,
) -> Result<Response> {
    let user: &users::Model = user.as_ref();
    let jwt_secret = ctx.config.get_jwt_config()?;
    // Generate a JWT token
    let token = user
        .generate_jwt(&jwt_secret.secret, jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;
    // Return the user and the token in JSON format
    format::json(LoginResponse::new(user, &token))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/oauth2")
        .add("/entraid", get(entraid_authorization_url))
        // Route for the Cookie callback
        .add("/entraid/callback/cookie", get(entraid_callback_cookie))
        .add("/protected", get(protected))
}
