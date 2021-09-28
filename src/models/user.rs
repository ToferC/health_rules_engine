use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInstance {
    id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    id: String,
    user_instance_uid: String,
    email: String,
    access_level: String, // AccessLevelEnum
    created_on: NaiveDateTime,
    access_key: String,
    approved_by_user_uid: String,
}