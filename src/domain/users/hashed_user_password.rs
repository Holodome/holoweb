use crate::domain::users::{UserPassword, UserPasswordSalt};
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sqlite::Sqlite;
use secrecy::{ExposeSecret, Secret};
use sha3::Digest;
use std::io::Write;

#[derive(Debug, Clone, diesel::AsExpression)]
#[sql_type = "diesel::sql_types::Text"]
pub struct HashedUserPassword {
    s: Secret<String>,
}

impl HashedUserPassword {
    pub fn parse(password: &UserPassword, salt: &UserPasswordSalt) -> Self {
        let new_password_string =
            password.as_ref().expose_secret().to_owned() + salt.as_ref().expose_secret();
        let password_hash = sha3::Sha3_256::digest(new_password_string.as_bytes());
        let password_hash = format!("{:x}", password_hash);
        Self {
            s: Secret::new(password_hash),
        }
    }
}

impl PartialEq<HashedUserPassword> for HashedUserPassword {
    fn eq(&self, other: &HashedUserPassword) -> bool {
        self.as_ref()
            .expose_secret()
            .eq(other.as_ref().expose_secret())
    }
}

impl diesel::Queryable<diesel::sql_types::Text, Sqlite> for HashedUserPassword {
    type Row = <String as diesel::Queryable<diesel::sql_types::Text, Sqlite>>::Row;

    fn build(row: Self::Row) -> Self {
        HashedUserPassword {
            s: Secret::new(row),
        }
    }
}

impl FromSql<diesel::sql_types::Text, Sqlite> for HashedUserPassword {
    fn from_sql(
        bytes: Option<&<Sqlite as Backend>::RawValue>,
    ) -> diesel::deserialize::Result<Self> {
        <String as FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(bytes)
            .map(|s| HashedUserPassword { s: Secret::new(s) })
    }
}

impl ToSql<diesel::sql_types::Text, Sqlite> for HashedUserPassword {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> diesel::serialize::Result {
        <String as ToSql<diesel::sql_types::Text, Sqlite>>::to_sql(self.s.expose_secret(), out)
    }
}

impl AsRef<Secret<String>> for HashedUserPassword {
    fn as_ref(&self) -> &Secret<String> {
        &self.s
    }
}
