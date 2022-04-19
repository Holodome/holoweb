mod blog_posts;
mod comments;
mod credentials;
mod projects;
mod users;

fn get_current_time_str() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        .to_string()
}

pub use blog_posts::*;
pub use comments::*;
pub use credentials::*;
pub use projects::*;
pub use users::*;
