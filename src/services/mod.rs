mod authentication;
mod credentials;
mod users;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;

type Connection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub use authentication::*;
pub use credentials::*;
pub use users::*;
