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
        new_user: NewUser,
    ) -> Result<Self, diesel::result::Error> {
        let user = User {
            id: Uuid::new_v4().to_string(),
            lastname: new_user.lastname,
            firstname: new_user.firstname,
        };

        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)?;

        Ok(user)
    }

    pub fn get_by_id(
        connection: &MysqlConnection,
        user_id: String,
    ) -> Result<Self, diesel::result::Error> {
        use crate::db::schema::users::dsl::*;
        users.find(user_id).get_result::<User>(connection)
    }

    pub fn delete(
        connection: &MysqlConnection,
        user_id: String,
    ) -> Result<usize, diesel::result::Error> {
        use crate::db::schema::users::dsl::*;

        let num_deleted = diesel::delete(users.filter(id.eq(user_id))).execute(connection)?;
        Ok(num_deleted)
    }

    pub fn update(
        connection: &MysqlConnection,
        user_id: String,
        new_user: NewUser,
    ) -> Result<Self, diesel::result::Error> {
        use crate::db::schema::users::dsl::*;

        diesel::update(users.find(&user_id))
            .set((
                lastname.eq(&new_user.lastname),
                firstname.eq(&new_user.firstname),
            ))
            .execute(connection)?;

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
