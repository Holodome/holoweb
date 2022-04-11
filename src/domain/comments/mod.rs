mod comment;
mod new_comment;
mod update_comment;

pub type CommentID = ResourceID;

use crate::domain::resource_id::ResourceID;
pub use comment::*;
pub use new_comment::*;
pub use update_comment::*;
