use holosite::domain::blog_posts::{BlogPostID, BlogPostVisibility, NewBlogPost};
use holosite::domain::users::UserID;
use holosite::services::insert_new_blog_post;
use holosite::Pool;
use uuid::Uuid;

pub struct TestBlogPost {
    pub title: String,
    pub brief: String,
    pub contents: String,
    pub visibility: BlogPostVisibility,
}

impl TestBlogPost {
    pub fn generate() -> Self {
        Self {
            title: Uuid::new_v4().to_string(),
            brief: Uuid::new_v4().to_string(),
            contents: Uuid::new_v4().to_string(),
            visibility: BlogPostVisibility::All,
        }
    }

    pub fn generate_authenticated() -> Self {
        let mut result = Self::generate();
        result.visibility = BlogPostVisibility::Authenticated;
        result
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "title": self.title.clone(),
            "brief": self.brief.clone(),
            "contents": self.contents.clone()
        })
    }

    pub fn register_internally(&self, pool: &Pool, author_id: &UserID) -> BlogPostID {
        let new_blog_post = NewBlogPost {
            title: self.title.as_str(),
            brief: self.brief.as_str(),
            contents: self.contents.as_str(),
            author_id,
            visibility: self.visibility.clone(),
        };
        insert_new_blog_post(pool, &new_blog_post)
            .expect("Failed to insert blog post")
            .id
    }
}
