use crate::api::assert_is_redirect_to_resource;
use crate::common::test_app::TestApp;
use holosite::domain::users::{NewUser, UserID, UserName, UserPassword};
use holosite::services::insert_new_user;
use holosite::Pool;
use secrecy::ExposeSecret;
use secrecy::Secret;

pub struct TestUser {
    pub name: UserName,
    pub password: UserPassword,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            name: UserName::generate_random(),
            password: UserPassword::parse(Secret::new("!1Aapass".to_string())).expect("OOps"),
        }
    }

    pub fn register_internally(&self, pool: &Pool) -> UserID {
        let new_user = NewUser {
            name: self.name.clone(),
            password: self.password.clone(),
        };
        insert_new_user(&pool, &new_user)
            .expect("Failed to insert new user")
            .id
    }

    pub async fn login(&self, app: &TestApp) {
        let response = app
            .post_login(&serde_json::json!({
                "name": self.name.as_ref(),
                "password": self.password.as_ref().expose_secret()
            }))
            .await;
        assert_is_redirect_to_resource(&response, "/blog_posts/all");
    }
}
