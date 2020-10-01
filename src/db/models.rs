use super::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Insertable, Debug)]
pub struct User {
    pub id: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub firstname: String,
    pub lastname: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserList(pub Vec<User>);

impl User {
    pub fn create(
        connection: &MysqlConnection,
        lastname: String,
        firstname: String,
    ) -> Result<Self, diesel::result::Error> {
        let new_user = User {
            id: Uuid::new_v4().to_string(),
            lastname: lastname,
            firstname: firstname,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(connection)?;

        Ok(new_user)
    }
}

impl UserList {
    pub fn list(connection: &MysqlConnection) -> Self {
        use crate::db::schema::users::dsl::*;

        let result = users
            .limit(10)
            .load::<User>(connection)
            .expect("Error loading users");

        UserList(result)
    }
}
