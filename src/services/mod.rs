mod users;
mod authentication;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;

type Connection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub use users::*;
pub use authentication::*;