use crate::db::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Insertable, Debug)]
pub struct User {
    pub id: String,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub lastname: String,
    pub firstname: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub lastname: String,
    pub firstname: String,
    pub email: String,
    pub token: String,
    pub expires_at: String,
}

#[derive(Serialize, Deserialize)]
// TODO: Supprimer et mettre dans impl User
pub struct UserList(pub Vec<User>);

impl User {
    pub fn login(
        connection: &MysqlConnection,
        user_login: Login,
    ) -> Result<Self, diesel::result::Error> {
        use crate::db::schema::users::dsl::*;

        let hashed_password = format!("{:x}", Sha512::digest(&user_login.password.as_bytes()));
        let user = users
            .filter(email.eq(&user_login.email))
            .filter(password.eq(&hashed_password))
            .get_result::<User>(connection);
        user
    }

    pub fn create(
        connection: &MysqlConnection,
        new_user: NewUser,
    ) -> Result<Self, diesel::result::Error> {
        let user = User {
            id: Uuid::new_v4().to_string(),
            lastname: new_user.lastname,
            firstname: new_user.firstname,
            email: new_user.email,
            password: format!("{:x}", Sha512::digest(&new_user.password.as_bytes())),
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

// TODO: Supprimer et mettre dans impl User
impl UserList {
    pub fn list(connection: &MysqlConnection) -> Result<Self, diesel::result::Error> {
        use crate::db::schema::users::dsl::*;

        let result = users.limit(10).load::<User>(connection)?;

        Ok(UserList(result))
    }
}
