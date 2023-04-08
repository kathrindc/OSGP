use crate::{database::establish_connection, schema::users};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Identifiable, Queryable, Serialize, AsChangeset)]
pub struct User {
    pub id: i32,
    pub role: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct UserTrim {
    pub role: String,
    pub email: String,
    pub password: String,
}

fn hash_password(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let bytes = password.as_bytes();

    Argon2::default()
        .hash_password(bytes, &salt)
        .expect("Error while hashing password")
        .to_string()
}

impl User {
    pub fn load_all(page: u32) -> Vec<User> {
        let connection = &mut establish_connection();
        let page_size: i64 = 20;
        let offset = (page - 1) as i64 * page_size;

        users::table
            .offset(offset)
            .limit(page_size)
            .load::<User>(connection)
            .expect("Database error while loading users")
    }

    pub fn load_by_id(id: i32) -> Option<User> {
        let connection = &mut establish_connection();

        users::table
            .find(id)
            .first::<User>(connection)
            .optional()
            .expect("Database error while finding user")
    }

    pub fn load_by_logon(email: String, password: String) -> Option<User> {
        let connection = &mut establish_connection();

        users::table
            .filter(users::dsl::email.eq(email))
            .first::<User>(connection)
            .optional()
            .expect("Database error while finding user")
            .and_then(|user| match user.verify_password(password) {
                true => Some(user),
                false => None,
            })
    }

    pub fn count() -> i64 {
        let connection = &mut establish_connection();

        users::table
            .count()
            .get_result(connection)
            .expect("Database error while counting users")
    }

    pub fn update(mut self, trim: UserTrim) -> User {
        let connection = &mut establish_connection();

        self.role = trim.role;
        self.email = trim.email;

        self.save_changes::<User>(connection)
            .expect("Database error while updating user")
    }

    pub fn update_password(mut self, password: String) {
        let connection = &mut establish_connection();

        self.password = hash_password(password);

        self.save_changes::<User>(connection)
            .expect("Database error while updating user");
    }

    pub fn verify_password(&self, other: String) -> bool {
        let hash = PasswordHash::new(&self.password).expect("Unable to read password hash");
        let arg = Argon2::default();
        let bytes = other.as_bytes();

        arg.verify_password(bytes, &hash).is_ok()
    }

    pub fn delete(id: i32) -> bool {
        let connection = &mut establish_connection();

        match diesel::delete(users::table.find(id))
            .execute(connection)
            .expect("Database error while deleting user")
        {
            1 => true,
            _ => false,
        }
    }
}

impl UserTrim {
    pub fn store(mut self) -> User {
        let connection = &mut establish_connection();

        self.password = hash_password(self.password);

        diesel::insert_into(users::dsl::users)
            .values(self)
            .get_result::<User>(connection)
            .expect("Database error while creating user")
    }
}
