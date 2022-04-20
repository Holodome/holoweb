use holosite::domain::projects::{NewProject, ProjectID, ProjectVisibility};
use holosite::domain::users::UserID;
use holosite::services::insert_new_project;
use holosite::Pool;
use uuid::Uuid;

pub struct TestProject {
    pub title: String,
    pub brief: String,
}

impl TestProject {
    pub fn generate() -> Self {
        Self {
            title: Uuid::new_v4().to_string(),
            brief: Uuid::new_v4().to_string(),
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "title": self.title.clone(),
            "brief": self.brief.clone(),
        })
    }

    pub fn register_internally(&self, pool: &Pool, author_id: &UserID) -> ProjectID {
        let new_project = NewProject {
            title: self.title.as_str(),
            brief: self.brief.as_str(),
            author_id,
            visibility: ProjectVisibility::All,
        };
        insert_new_project(pool, &new_project)
            .expect("Failed to insert blog post")
            .id
    }
}
