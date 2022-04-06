mod blog_posts;
mod credentials;
mod password;
mod users;

#[derive(serde::Deserialize, Clone)]
pub struct Page {
    pub number: usize,
    pub size: usize,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            number: 0,
            size: 10,
        }
    }
}

pub use blog_posts::*;
pub use credentials::*;
pub use password::*;
pub use users::*;
