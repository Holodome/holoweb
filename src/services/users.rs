use crate::domain::users::hashed_user_password::HashedUserPassword;
use crate::domain::users::{
    NewUser, UpdateUser, User, UserEmail, UserID, UserName, UserPasswordSalt,
};
use crate::schema::users::dsl::*;
use crate::startup::Pool;
use diesel::{insert_into, update, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
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

pub fn insert_new_user(pool: &Pool, new_user: &NewUser) -> Result<User, anyhow::Error> {
    let conn = pool.get()?;
    let salt = UserPasswordSalt::generate_random();
    let hashed_password = HashedUserPassword::parse(&new_user.password, &salt);

    let user = User {
        id: UserID::generate_random(),
        name: new_user.name.clone(),
        email: UserEmail::parse(format!("{}@email.com", Uuid::new_v4())).expect("Oh no"), // TODO
        password: hashed_password,
        password_salt: salt,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
            .to_string(),
        is_banned: false,
    };

    insert_into(users).values(&user).execute(&conn)?;

    Ok(user)
}

pub fn update_user(pool: &Pool, changeset: UpdateUser) -> Result<(), anyhow::Error> {
    let conn = pool.get()?;
    update(users.filter(id.eq(&changeset.id)))
        .set(&changeset)
        .execute(&conn)?;
    Ok(())
}
