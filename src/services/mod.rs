mod blog_posts;
mod comments;
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

fn get_current_time_str() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        .to_string()
}

pub use blog_posts::*;
pub use credentials::*;
pub use password::*;
pub use users::*;
