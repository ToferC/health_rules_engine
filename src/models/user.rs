// Modelled off https://github.com/clifinger/canduma/blob/master/src/user

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use diesel::{self, ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::{schema::*};
use crate::common_utils::{is_admin, RoleGuard, Role as AuthRole};
use crate::models::hash_password;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInstance {
    id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject, Queryable, AsChangeset)]
pub struct User {
    #[graphql(
        guard = "RoleGuard::new(AuthRole::Admin)",
        visible = "is_admin",
    )]
    pub id: Uuid,
    #[graphql(skip)]
    pub hash: String,

    #[graphql(
        guard = "RoleGuard::new(AuthRole::Admin)",
        visible = "is_admin",
    )]
    pub email: String,
    pub role: String,

    #[graphql(
        guard = "RoleGuard::new(AuthRole::Admin)",
        visible = "is_admin",
    )]
    pub name: String,
    pub access_level: String, // AccessLevelEnum
    pub created_at: NaiveDateTime,
    #[graphql(
        guard = "RoleGuard::new(AuthRole::Admin)",
        visible = "is_admin",
    )]
    /// Access Level: Admin
    pub access_key: String,

    #[graphql(
        guard = "RoleGuard::new(AuthRole::Admin)",
        visible = "is_admin",
    )]
    /// Access Level: Admin
    pub approved_by_user_uid: Option<Uuid>,
}

impl User {

    pub fn get_by_id(id: &Uuid, conn: &PgConnection) -> FieldResult<Self> {
        let user = users::table
            .filter(users::id.eq(id))
            .get_result(conn)?;

        Ok(user)
    }

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

    pub fn update(&self, conn: &PgConnection) -> FieldResult<Self> {
        let user = diesel::update(users::table)
            .filter(users::id.eq(&self.id))
            .set(self)
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
    /// Role in system: USER, OPERATOR, ANALYST, ADMIN
    pub role: String,
}

#[derive(Debug, Deserialize, Serialize, InputObject)]
/// Input Struct to create a new user. Only accessible by Administrators.
pub struct UserUpdate {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    /// Role in system: USER, OPERATOR, ANALYST, ADMIN
    pub role: Option<String>,
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