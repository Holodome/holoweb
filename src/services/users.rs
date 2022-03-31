





// #[tracing::instrument("Get user by id", skip(conn, user_id))]
// pub fn get_user_by_id(conn: &Connection, user_id: &str) -> Result<Option<User>, DbError> {
//     Ok(users
//         .filter(id.eq(user_id))
//         .first::<User>(conn)
//         .optional()?)
// }
//
// pub fn get_user_by_name(conn: &Connection, user_name: &str) -> Result<Option<User>, DbError> {
//     Ok(users
//         .filter(name.eq(user_name))
//         .first::<User>(conn)
//         .optional()?)
// }
//
