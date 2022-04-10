use crate::helpers::{get_db_connection, TestUser};
use claim::{assert_ok, assert_some};
use holosite::domain::users::hashed_user_password::HashedUserPassword;
use holosite::domain::users::{NewUser, UpdateUser, UserName, UserPassword};
use holosite::services::{get_user_by_id, get_user_by_name, insert_new_user, update_user};
use secrecy::Secret;

#[test]
fn test_add_new_user_works() {
    let pool = get_db_connection();
    let test_user = TestUser::generate();
    let res = insert_new_user(
        &pool,
        &NewUser {
            name: test_user.name.clone(),
            password: test_user.password.clone(),
        },
    );
    assert_ok!(&res);
    let res = res.unwrap();
    assert_eq!(&res.name, &test_user.name);
}

#[test]
fn test_add_new_user_and_get_it_by_name_works() {
    let pool = get_db_connection();
    let test_user = TestUser::generate();
    test_user.register_internally(&pool);

    let res = get_user_by_name(&pool, &test_user.name);
    assert_ok!(&res);
    let res = res.unwrap();
    assert_some!(&res);
    let res = res.unwrap();
    assert_eq!(res.name, test_user.name);
}

#[test]
fn test_add_new_user_and_get_it_by_id_works() {
    let pool = get_db_connection();
    let test_user = TestUser::generate();
    let id = test_user.register_internally(&pool);

    let res = get_user_by_id(&pool, &id);
    assert_ok!(&res);
    let res = res.unwrap();
    assert_some!(&res);
    let res = res.unwrap();
    assert_eq!(res.name, test_user.name);
}

#[test]
fn update_user_name_works() {
    let pool = get_db_connection();
    let test_user = TestUser::generate();
    let id = test_user.register_internally(&pool);

    let initial = get_user_by_id(&pool, &id).unwrap().unwrap();
    let new_name = UserName::generate_random();
    let changeset = UpdateUser {
        id: &id,

        name: Some(&new_name),
        email: None,
        password: None,
        is_banned: None,
    };
    let res = update_user(&pool, &changeset);
    assert_ok!(res);

    let user = get_user_by_id(&pool, &id).unwrap().unwrap();
    assert_ne!(initial, user);
    assert_eq!(user.name, new_name);
}

#[test]
fn update_user_password_works() {
    let pool = get_db_connection();
    let test_user = TestUser::generate();
    let id = test_user.register_internally(&pool);

    let initial = get_user_by_id(&pool, &id).unwrap().unwrap();
    let new_password = UserPassword::parse(Secret::new("!1Aaaaaa".to_string())).unwrap();
    let hashed_password = HashedUserPassword::parse(&new_password, &initial.password_salt);

    let changeset = UpdateUser {
        id: &id,

        name: None,
        email: None,
        password: Some(&hashed_password),
        is_banned: None,
    };
    let res = update_user(&pool, &changeset);
    assert_ok!(res);

    let user = get_user_by_id(&pool, &id).unwrap().unwrap();
    assert_ne!(initial, user);
    assert_eq!(user.name, initial.name);
    assert_eq!(user.password, hashed_password);
}
