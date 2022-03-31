#[derive(Debug, derive_more::Display)]
pub struct UserID {
    s: String,
}

impl diesel::Queryable<diesel::sql_types::Text, diesel::sqlite::Sqlite> for UserID {
    type Row = <String as diesel::Queryable<diesel::sql_types::Text, diesel::sqlite::Sqlite>>::Row;

    fn build(row: Self::Row) -> Self {
        UserID { s: row }
    }
}