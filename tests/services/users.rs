use claim::{assert_ok, assert_some_eq};
use holosite::domain::users::NewUser;
use holosite::services::{get_user_by_id, get_user_by_name, insert_new_user};
use secrecy::Secret;

#[test]
fn insert_user_works() {
    let pool = crate::helpers::get_new_connection();
    let conn = pool.get().expect("Failed to get connection");
    let name = "SeriousName";
    let password = "Str0ngPassword";
    let new_user = NewUser::parse(name.to_string(), Secret::new(password.to_string()))
        .expect("Failed to create NewUser");

    let res = insert_new_user(&conn, &new_user);
    assert_ok!(&res);
    assert_eq!(res.as_ref().unwrap().name.as_ref(), name);
}

#[test]
fn insert_and_get_user_by_name_and_id_works() {
    let pool = crate::helpers::get_new_connection();
    let conn = pool.get().expect("Failed to get connection");
    let name = "SeriousName";
    let password = "Str0ngPassword";
    let new_user = NewUser::parse(name.to_string(), Secret::new(password.to_string()))
        .expect("Failed to create NewUser");

    let res = insert_new_user(&conn, &new_user);
    assert_ok!(&res);
    let insert_user = res.unwrap();

    let by_id = get_user_by_id(&conn, &insert_user.id);
    assert_ok!(&by_id);
    let by_id = by_id.unwrap();
    assert_some_eq!(by_id, insert_user);

    let by_name = get_user_by_name(&conn, &insert_user.name);
    assert_ok!(&by_name);
    let by_name = by_name.unwrap();
    assert_some_eq!(by_name, insert_user);
}
