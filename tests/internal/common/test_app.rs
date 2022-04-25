use crate::common::{get_test_config, init_tracing, TestDB};
use holosite::domain::blog_posts::BlogPostID;
use holosite::domain::comments::CommentID;
use holosite::domain::projects::ProjectID;
use holosite::startup::Application;
use holosite::Pool;
use reqwest::Response;

pub struct TestApp {
    address: String,
    db: TestDB,
    api_client: reqwest::Client,
}

impl TestApp {
    pub async fn spawn() -> TestApp {
        init_tracing();
        let config = get_test_config();
        let db = TestDB::new(&config.database_uri);
        let app = Application::build_with_pool(config.clone(), db.pool().clone())
            .await
            .expect("Failed to build application");

        let address = format!("http://localhost:{}", app.port());

        let _ = tokio::spawn(app.run_until_stopped());

        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .cookie_store(true)
            .build()
            .unwrap();

        TestApp {
            address,
            db,
            api_client: client,
        }
    }

    pub fn pool(&self) -> &Pool {
        self.db.pool()
    }

    pub async fn post_logout(&self) -> Response {
        self.api_client
            .get(format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_health_check(&self) -> Response {
        self.get_page("/health_check").await
    }

    pub async fn get_login_page(&self) -> Response {
        self.get_page("/login").await
    }

    pub async fn get_login_page_html(&self) -> String {
        self.get_login_page().await.text().await.unwrap()
    }

    pub async fn get_registration_page(&self) -> Response {
        self.get_page("/registration").await
    }

    pub async fn get_registration_page_html(&self) -> String {
        self.get_registration_page().await.text().await.unwrap()
    }

    pub async fn post_login<Body>(&self, body: &Body) -> Response
    where
        Body: serde::Serialize,
    {
        self.post("/login", body).await
    }

    pub async fn post_registration<Body>(&self, body: &Body) -> Response
    where
        Body: serde::Serialize,
    {
        self.post("/registration", body).await
    }

    pub async fn post_change_password(&self, body: &impl serde::Serialize) -> Response {
        self.post("/account/change_password", body).await
    }

    pub async fn post_change_name(&self, body: &impl serde::Serialize) -> Response {
        self.post("/account/change_name", body).await
    }

    pub async fn post_create_blog_post(&self, body: &impl serde::Serialize) -> Response {
        self.post("/blog_posts/create", body).await
    }

    pub async fn post_edit_blog_post(
        &self,
        body: &impl serde::Serialize,
        id: &BlogPostID,
    ) -> Response {
        self.post(format!("/blog_posts/{}/edit", id.as_ref()).as_str(), body)
            .await
    }

    pub async fn post_create_comment(
        &self,
        body: &impl serde::Serialize,
        id: &BlogPostID,
    ) -> Response {
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
    ) -> Response {
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

    pub async fn post_delete_comment(
        &self,
        post_id: &BlogPostID,
        comment_id: &CommentID,
    ) -> Response {
        self.api_client
            .get(
                format!(
                    "{}/blog_posts/{}/comments/{}/delete",
                    &self.address,
                    post_id.as_ref(),
                    comment_id.as_ref()
                )
                .as_str(),
            )
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_create_project(&self, body: &impl serde::Serialize) -> Response {
        self.post("/projects/create", body).await
    }

    pub async fn post_edit_project(
        &self,
        body: &impl serde::Serialize,
        project_id: &ProjectID,
    ) -> Response {
        self.post(
            format!("/projects/{}/edit", project_id.as_ref()).as_str(),
            body,
        )
        .await
    }

    pub async fn get_account_page_html(&self) -> String {
        self.get_page("/account/home").await.text().await.unwrap()
    }

    pub async fn get_create_blog_post_page(&self) -> Response {
        self.get_page("/blog_posts/create").await
    }

    pub async fn get_all_blog_posts_page(&self) -> Response {
        self.get_page("/blog_posts/all").await
    }

    pub async fn get_all_blog_posts_page_html(&self) -> String {
        self.get_all_blog_posts_page().await.text().await.unwrap()
    }

    pub async fn get_edit_blog_post_page(&self, id: &str) -> Response {
        self.get_page(format!("/blog_posts/{}/edit", id).as_str())
            .await
    }

    pub async fn get_edit_blog_post_page_html(&self, id: &str) -> String {
        self.get_edit_blog_post_page(id).await.text().await.unwrap()
    }

    pub async fn get_view_blog_post_page(&self, id: &str) -> Response {
        self.get_page(format!("/blog_posts/{}/view", id).as_str())
            .await
    }

    pub async fn get_view_blog_post_page_html(&self, id: &str) -> String {
        self.get_view_blog_post_page(id).await.text().await.unwrap()
    }

    pub async fn get_view_project_page(&self, id: &str) -> Response {
        self.get_page(format!("/projects/{}/view", id).as_str())
            .await
    }

    pub async fn get_view_project_page_html(&self, id: &str) -> String {
        self.get_view_blog_post_page(id).await.text().await.unwrap()
    }

    pub async fn get_all_projects_page(&self) -> Response {
        self.get_page("/projects/all").await
    }

    pub async fn get_all_projects_page_html(&self) -> String {
        self.get_all_projects_page().await.text().await.unwrap()
    }

    pub async fn get_edit_project_page(&self, id: &str) -> Response {
        self.get_page(format!("/blog_posts/{}/edit", id).as_str())
            .await
    }

    pub async fn get_edit_project_page_html(&self, id: &str) -> String {
        self.get_edit_project_page(id).await.text().await.unwrap()
    }

    pub async fn get_page(&self, rel_address: &str) -> Response {
        self.api_client
            .get(format!("{}{}", &self.address, rel_address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post<Body>(&self, rel_addr: &str, body: &Body) -> Response
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
