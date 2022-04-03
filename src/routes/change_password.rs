use crate::domain::credentials::Credentials;
use crate::domain::users::{UserName, UserPassword};
use crate::middleware::reject_anonymous_users;
use crate::services::{validate_credentials, AuthError};
use crate::startup::Pool;
use crate::utils::{e500, see_other};
use actix_web::{route, web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use actix_web_lab::middleware::from_fn;
use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    repeat_new_password: Secret<String>,
}

#[route(
    "/change_password",
    method = "POST",
    wrap = "from_fn(reject_anonymous_users)"
)]
#[tracing::instrument(skip(form, pool))]
pub async fn change_password(
    form: web::Form<FormData>,
    pool: web::Data<Pool>,
    user_name: web::ReqData<UserName>,
) -> actix_web::Result<HttpResponse> {
    let user_name = user_name.into_inner();
    if form.new_password.expose_secret() != form.repeat_new_password.expose_secret() {
        FlashMessage::error("Repeat password doesn't match old password").send();
        return Ok(see_other("/password"));
    }

    let credentials = Credentials {
        name: user_name.clone(),
        password: UserPassword::parse(form.current_password.clone()).map_err(e500)?,
    };
    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("Current password is incorrect").send();
                Ok(see_other("/change_password"))
            }
            AuthError::UnexpectedError(_) => Err(e500(e)),
        };
    }

    let new_password = UserPassword::parse(form.new_password.clone()).map_err(e500)?;
    crate::services::change_password(&pool, &user_name, &new_password)
        .await
        .map_err(e500)?;
    FlashMessage::info("Your password has been changed").send();
    Ok(see_other("/change_password"))
}
