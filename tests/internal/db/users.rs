use crate::common::{TestDB, TestUser};
use claim::{assert_err, assert_ok, assert_some};
use holosite::domain::users::{HashedUserPassword, NewUser, UpdateUser, UserName, UserPassword};
use holosite::services::{
    get_user_by_id, get_user_by_name, insert_new_user, update_user, UserError,
};
use secrecy::Secret;

#[test]
fn test_add_new_user_works() {
    let db = TestDB::spawn();
    let test_user = TestUser::generate();
    let res = insert_new_user(
        db.pool(),
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
    let db = TestDB::spawn();
    let test_user = TestUser::generate();
    test_user.register_internally(db.pool());

    let res = get_user_by_name(db.pool(), &test_user.name);
    assert_ok!(&res);
    let res = res.unwrap();
    assert_some!(&res);
    let res = res.unwrap();
    assert_eq!(res.name, test_user.name);
}

#[test]
fn test_add_new_user_and_get_it_by_id_works() {
    let db = TestDB::spawn();
    let test_user = TestUser::generate();
    let id = test_user.register_internally(db.pool());

    let res = get_user_by_id(db.pool(), &id);
    assert_ok!(&res);
    let res = res.unwrap();
    assert_some!(&res);
    let res = res.unwrap();
    assert_eq!(res.name, test_user.name);
}

#[test]
fn update_user_name_works() {
    let db = TestDB::spawn();
    let test_user = TestUser::generate();
    let id = test_user.register_internally(db.pool());

    let initial = get_user_by_id(db.pool(), &id).unwrap().unwrap();
    let new_name = UserName::generate_random();
    let changeset = UpdateUser {
        id: &id,

        name: Some(&new_name),
        email: None,
        password: None,
        is_banned: None,
    };
    let res = update_user(db.pool(), &changeset);
    assert_ok!(res);

    let user = get_user_by_id(db.pool(), &id).unwrap().unwrap();
    assert_ne!(initial, user);
    assert_eq!(user.name, new_name);
}

#[test]
fn update_user_password_works() {
    let db = TestDB::spawn();
    let test_user = TestUser::generate();
    let id = test_user.register_internally(db.pool());

    let initial = get_user_by_id(db.pool(), &id).unwrap().unwrap();
    let new_password = UserPassword::parse(Secret::new("!1Aaaaaa".to_string())).unwrap();
    let hashed_password = HashedUserPassword::parse(&new_password, &initial.password_salt);

    let changeset = UpdateUser {
        id: &id,

        name: None,
        email: None,
        password: Some(&hashed_password),
        is_banned: None,
    };
    let res = update_user(db.pool(), &changeset);
    assert_ok!(res);

    let user = get_user_by_id(db.pool(), &id).unwrap().unwrap();
    assert_ne!(initial, user);
    assert_eq!(user.name, initial.name);
    assert_eq!(user.password, hashed_password);
}

#[test]
fn cant_create_user_with_same_name_and_get_correct_error_kind() {
    let db = TestDB::spawn();
    let test_user = TestUser::generate();
    test_user.register_internally(db.pool());

    let res = insert_new_user(
        db.pool(),
        &NewUser {
            name: test_user.name.clone(),
            password: test_user.password,
        },
    );
    assert_err!(&res);
    let res = res.unwrap_err();
    match res {
        UserError::TakenName => {}
        _ => panic!("Incorrect error type: got {:?}", res),
    };
}

#[test]
fn cant_update_user_with_same_name_and_get_correct_error_kind() {
    let db = TestDB::spawn();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(db.pool());
    let other_user = TestUser::generate();
    other_user.register_internally(db.pool());

    let res = update_user(
        db.pool(),
        &UpdateUser {
            id: &user_id,
            name: Some(&other_user.name),
            email: None,
            password: None,
            is_banned: None,
        },
    );
    assert_err!(&res);
    let res = res.unwrap_err();
    match res {
        UserError::TakenName => {}
        _ => panic!("Incorrect error type: got {:?}", res),
    };
}
