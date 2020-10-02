use super::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Insertable, Debug)]
pub struct User {
    pub id: String,
    pub lastname: String,
    pub firstname: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub lastname: String,
    pub firstname: String,
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

    pub fn get_by_id(
        connection: &MysqlConnection,
        user_id: String,
    ) -> Result<Self, diesel::result::Error> {
        use crate::db::schema::users::dsl::*;
        users.find(user_id).get_result::<User>(connection)
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
