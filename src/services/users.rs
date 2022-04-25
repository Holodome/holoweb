use crate::domain::time::DateTime;
use crate::domain::users::{
    Credentials, HashedUserPassword, NewUser, UpdateUser, User, UserEmail, UserID, UserName,
    UserPasswordSalt, UserRole,
};
use crate::schema::users::dsl::*;
use crate::services::get_stored_credentials;
use crate::Pool;
use diesel::result::{DatabaseErrorKind, Error};
use diesel::{insert_into, update, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use std::fmt::Formatter;
use uuid::Uuid;

pub fn get_user_by_id(pool: &Pool, user_id: &UserID) -> Result<Option<User>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(users
        .filter(id.eq(user_id))
        .first::<User>(&conn)
        .optional()?)
}

pub fn get_user_by_name(pool: &Pool, user_name: &UserName) -> Result<Option<User>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(users
        .filter(name.eq(user_name))
        .first::<User>(&conn)
        .optional()?)
}

#[derive(thiserror::Error)]
pub enum UserError {
    #[error("Name is already taken")]
    TakenName,
    #[error("Email is already taken")]
    TakenEmail,
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for UserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use crate::utils::error_chain_fmt;

        error_chain_fmt(self, f)
    }
}

pub fn insert_new_user(pool: &Pool, new_user: &NewUser) -> Result<User, UserError> {
    let conn = pool
        .get()
        .map_err(|e| UserError::UnexpectedError(e.into()))?;
    let salt = UserPasswordSalt::generate_random();
    let hashed_password = HashedUserPassword::parse(&new_user.password, &salt);

    let user = User {
        id: UserID::generate_random(),
        name: new_user.name.clone(),
        email: UserEmail::parse(format!("{}@email.com", Uuid::new_v4())).expect("Oh no"), // TODO
        password: hashed_password,
        password_salt: salt,
        created_at: DateTime::now(),
        is_banned: false,
        role: UserRole::User,
    };

    insert_into(users)
        .values(&user)
        .execute(&conn)
        .map_err(get_user_error_from_database_error)?;

    Ok(user)
}

pub fn update_user(pool: &Pool, changeset: &UpdateUser) -> Result<(), UserError> {
    let conn = pool
        .get()
        .map_err(|e| UserError::UnexpectedError(e.into()))?;
    update(users.filter(id.eq(&changeset.id)))
        .set(changeset)
        .execute(&conn)
        .map_err(get_user_error_from_database_error)?;
    Ok(())
}

fn get_user_error_from_database_error(e: Error) -> UserError {
    match e {
        Error::DatabaseError(DatabaseErrorKind::UniqueViolation, ref data) => {
            let msg = data.message();
            if msg.contains("name") {
                UserError::TakenName
            } else if msg.contains("email") {
                UserError::TakenEmail
            } else {
                UserError::UnexpectedError(e.into())
            }
        }
        _ => UserError::UnexpectedError(e.into()),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub fn validate_credentials(credentials: Credentials, pool: &Pool) -> Result<UserID, AuthError> {
    if let Some(stored) = get_stored_credentials(credentials.name, pool)? {
        let hashed_password = HashedUserPassword::parse(&credentials.password, &stored.salt);
        if hashed_password == stored.password {
            Ok(stored.user_id)
        } else {
            Err(AuthError::InvalidCredentials(anyhow::anyhow!(
                "Passwords don't match"
            )))
        }
    } else {
        Err(AuthError::InvalidCredentials(anyhow::anyhow!(
            "No user with such name found"
        )))
    }
}
