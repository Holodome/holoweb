mod blog_post;
mod new_blog_post;
mod update_blog_post;

pub type BlogPostID = ResourceID;

use crate::domain::resource_id::ResourceID;
pub use blog_post::*;
pub use new_blog_post::*;
pub use update_blog_post::*;
