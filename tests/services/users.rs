use claim::assert_ok;
use holosite::domain::{NewUser, UserName};
use holosite::services::insert_new_user;
use secrecy::{ExposeSecret, Secret};

#[test]
fn insert_user_works() {
    let pool = crate::helpers::get_new_connection();
    let conn = pool.get().expect("Failed to get connection");
    let name = "SeriousName";
    let password = "Str0ngPassword";
    let new_user = NewUser::parse(name.to_string(), Secret::new(password.to_string()))
        .expect("Failed to create NewUser");

    let res = insert_new_user(&conn, new_user);
    assert_ok!(&res);
    assert_eq!(res.as_ref().unwrap().name.as_ref(), name);
    assert_eq!(
        res.as_ref().unwrap().password.as_ref().expose_secret(),
        password
    );
}
