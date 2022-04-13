use holosite::domain::blog_posts::BlogPostID;
use holosite::domain::comments::{CommentID, NewComment};
use holosite::domain::users::UserID;
use holosite::services::insert_new_comment;
use holosite::startup::Pool;
use uuid::Uuid;

pub struct TestComment {
    pub contents: String,
}

impl TestComment {
    pub fn generate() -> Self {
        Self {
            contents: Uuid::new_v4().to_string(),
        }
    }

    pub fn register_internally(
        &self,
        pool: &Pool,
        post_id: &BlogPostID,
        author_id: &UserID,
    ) -> CommentID {
        let new_comment = NewComment {
            author_id,
            post_id,
            parent_id: None,
            contents: &self.contents,
        };
        insert_new_comment(&pool, &new_comment)
            .expect("Failed to insert comment.html")
            .id
    }

    pub fn register_response_internally(
        &self,
        pool: &Pool,
        post_id: &BlogPostID,
        author_id: &UserID,
        parent_id: &CommentID,
    ) -> CommentID {
        let new_comment = NewComment {
            author_id,
            post_id,
            parent_id: Some(parent_id),
            contents: &self.contents,
        };
        insert_new_comment(&pool, &new_comment)
            .expect("Failed to insert comment.html")
            .id
    }
}
