use crate::helpers::{get_test_config, TRACING};
use holosite::domain::blog_posts::BlogPostID;
use holosite::domain::comments::CommentID;
use holosite::startup::{Application, Pool};
use once_cell::sync::Lazy;

pub struct TestApp {
    pub address: String,
    pub pool: Pool,
    pub api_client: reqwest::Client,
}

impl TestApp {
    pub async fn spawn() -> TestApp {
        Lazy::force(&TRACING);

        let config = get_test_config();
        let app = Application::build(config.clone())
            .await
            .expect("Failed to build application");

        let pool = app.pool.clone();
        let address = format!("http://localhost:{}", app.port());

        let _ = tokio::spawn(app.run_until_stopped());

        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .cookie_store(true)
            .build()
            .unwrap();

        TestApp {
            address,
            pool,
            api_client: client,
        }
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.api_client
            .get(format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_login_page(&self) -> reqwest::Response {
        self.get_page("/login").await
    }

    pub async fn get_login_page_html(&self) -> String {
        self.get_login_page().await.text().await.unwrap()
    }

    pub async fn get_registration_page(&self) -> reqwest::Response {
        self.get_page("/registration").await
    }

    pub async fn get_registration_page_html(&self) -> String {
        self.get_registration_page().await.text().await.unwrap()
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.post("/login", body).await
    }

    pub async fn post_registration<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.post("/registration", body).await
    }

    pub async fn post_change_password(&self, body: &impl serde::Serialize) -> reqwest::Response {
        self.post("/account/change_password", body).await
    }

    pub async fn post_change_name(&self, body: &impl serde::Serialize) -> reqwest::Response {
        self.post("/account/change_name", body).await
    }

    pub async fn post_create_blog_post(&self, body: &impl serde::Serialize) -> reqwest::Response {
        self.post("/blog_posts/create", body).await
    }

    pub async fn post_edit_blog_post(
        &self,
        body: &impl serde::Serialize,
        id: &BlogPostID,
    ) -> reqwest::Response {
        self.post(format!("/blog_posts/{}/edit", id.as_ref()).as_str(), body)
            .await
    }

    pub async fn post_create_comment(
        &self,
        body: &impl serde::Serialize,
        id: &BlogPostID,
    ) -> reqwest::Response {
        self.post(
            format!("/blog_posts/{}/comments/create", id.as_ref().as_str()).as_str(),
            body,
        )
        .await
    }

    pub async fn post_edit_comment(
        &self,
        body: &impl serde::Serialize,
        post_id: &BlogPostID,
        comment_id: &CommentID,
    ) -> reqwest::Response {
        self.post(
            format!(
                "/blog_posts/{}/comments/{}/edit",
                post_id.as_ref(),
                comment_id.as_ref()
            )
            .as_str(),
            body,
        )
        .await
    }

    pub async fn get_home_page(&self) -> reqwest::Response {
        self.get_page("/").await
    }

    pub async fn get_home_page_html(&self) -> String {
        self.get_home_page().await.text().await.unwrap()
    }

    pub async fn get_account_page_html(&self) -> String {
        self.get_page("/account/home").await.text().await.unwrap()
    }

    pub async fn get_change_password(&self) -> reqwest::Response {
        self.get_page("/account/change_password").await
    }

    pub async fn get_change_password_page_html(&self) -> String {
        self.get_change_password().await.text().await.unwrap()
    }

    pub async fn get_change_name(&self) -> reqwest::Response {
        self.get_page("/account/change_name").await
    }

    pub async fn get_change_name_page_html(&self) -> String {
        self.get_change_name().await.text().await.unwrap()
    }

    pub async fn get_create_blog_post_page(&self) -> reqwest::Response {
        self.get_page("/blog_posts/create").await
    }

    pub async fn get_all_blog_posts_page(&self) -> reqwest::Response {
        self.get_page("/blog_posts/all").await
    }

    pub async fn get_all_blog_posts_page_html(&self) -> String {
        self.get_all_blog_posts_page().await.text().await.unwrap()
    }

    pub async fn get_edit_blog_post_page(&self, id: &str) -> reqwest::Response {
        self.get_page(format!("/blog_posts/{}/edit", id).as_str())
            .await
    }

    pub async fn get_edit_blog_post_page_html(&self, id: &str) -> String {
        self.get_edit_blog_post_page(id).await.text().await.unwrap()
    }

    pub async fn get_view_blog_post_page(&self, id: &str) -> reqwest::Response {
        self.get_page(format!("/blog_posts/{}/view", id).as_str())
            .await
    }

    pub async fn get_view_blog_post_page_html(&self, id: &str) -> String {
        self.get_view_blog_post_page(id).await.text().await.unwrap()
    }

    async fn get_page(&self, rel_address: &str) -> reqwest::Response {
        self.api_client
            .get(format!("{}{}", &self.address, rel_address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    async fn post<Body>(&self, rel_addr: &str, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(format!("{}{}", &self.address, rel_addr))
            .form(body)
            .send()
            .await
            .expect("Failed to execute request")
    }
}
