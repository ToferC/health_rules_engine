// Modelled off https://github.com/clifinger/canduma/blob/master/src/user

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use diesel::{self, ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::{schema::*};
use crate::models::hash_password;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInstance {
    id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject, Queryable)]
pub struct User {
    pub id: Uuid,
    #[graphql(skip)]
    pub hash: String,
    pub email: String,
    pub role: String,
    pub name: String,
    pub access_level: String, // AccessLevelEnum
    pub created_at: NaiveDateTime,
    pub access_key: String,
    pub approved_by_user_uid: Option<Uuid>,
}

impl User {
    pub fn get_by_email(email: &String, conn: &PgConnection) -> FieldResult<Self> {
        let user = users::table
            .filter(users::email.eq(email))
            .get_result(conn)?;

        Ok(user)
    }

    pub fn create(user: InsertableUser, conn: &PgConnection) -> FieldResult<Self> {
        let user = diesel::insert_into(users::table)
            .values(&user)
            .get_result(conn)?;

        Ok(user)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub hash: String,
    pub email: String,
    pub role: String,
    pub name: String,
    pub access_level: String, // AccessLevelEnum
    pub created_at: NaiveDateTime,
    pub access_key: String,
    pub approved_by_user_uid: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, InputObject)]
/// Input Struct to create a new user. Only accessible by Administrators.
pub struct UserData {
    pub name: String,
    pub email: String,
    pub password: String,
    /// Role in system: USER, OPERATOR, ADMIN
    pub role: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct SlimUser {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub access_level: String,
}

#[derive(Shrinkwrap, Clone, Default)]
pub struct LoggedUser(pub Option<SlimUser>);

impl From<SlimUser> for LoggedUser {
    fn from(slim_user: SlimUser) -> Self {
        LoggedUser(Some(slim_user))
    }
}

impl From<UserData> for InsertableUser {
    fn from(user_data: UserData) -> Self {
        let UserData {
            name,
            email,
            password,
            role,
            ..
        } = user_data;
        
        let hash = hash_password(&password)
            .expect("Unable to hash password");

        Self {
            email,
            hash,
            created_at: chrono::Utc::now().naive_utc(),
            name,
            role,
            access_key: "".to_owned(),
            access_level: "detailed".to_owned(),
            approved_by_user_uid: None,
        }
    }
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        let User {
            id,
            email,
            role,
            access_level,
            ..
        } = user;

        Self {
            id,
            email,
            role,
            access_level,
        }
    }
}

#[derive(Debug, Deserialize, InputObject)]
pub struct LoginQuery {
    pub email: String,
    pub password: String,
}

/*
pub fn make_salt() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 128;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
}

pub fn make_hash(password: &str, salt: &str) -> String {
    let hash = argon2i_simple(password, salt);

    String::from_utf8(hash.to_vec()).expect("Unable to conver hash to String")
}

pub fn verify(user: &User, password: &str) -> bool {
    let User { hash, salt, ..} = user;
    
    let computed_hash = make_hash(password, salt);
    computed_hash == hash.to_owned()
}

pub fn has_role(user: &LoggedUser, role: &str) -> core::result::Result<bool, CustomError> {
    match user.0 {
        Some(ref user) if user.role == role => Ok(true),
        _ => Err(CustomError::new(501, "Not Authorized".to_string())),
    }
}
*/