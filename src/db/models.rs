use diesel::MysqlConnection;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserList(pub Vec<User>);

impl UserList {
    pub fn list(connection: &MysqlConnection) -> Self {
        use crate::db::schema::users::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let result = users
            .limit(10)
            .load::<User>(connection)
            .expect("Error loading users");

        UserList(result)
    }
}
