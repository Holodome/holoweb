mod credentials;
mod password;
mod users;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;

type Connection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub use credentials::*;
pub use password::*;
pub use users::*;
