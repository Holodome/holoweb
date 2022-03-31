mod users;
mod authentication;
mod credentials;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;

type Connection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub use users::*;
pub use authentication::*;
pub use credentials::*;