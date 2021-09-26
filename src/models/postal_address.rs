use chrono::prelude::*;
use juniper::FieldResult;
use reqwest::header::VacantEntry;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use diesel::{self, Insertable, PgConnection, Queryable,
    ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel_derive_enum::DbEnum;
use uuid::Uuid;
use std::collections::HashMap;

use crate::models::{Place, Country};
use crate::GraphQLContext;
use crate::graphql::graphql_translate;
use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject)]
pub struct PostalAddress {
    pub id: Uuid,
    pub street_address: String,
    pub address_locality_id: Uuid,
    pub address_region: String,
    pub address_country_id: Uuid,
    pub postal_code: String,
    pub lattitude: f64,
    pub longitude: f64,
    pub additional_info: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject)]
pub struct NewPostalAddress {
    pub street_address: String,
    pub address_locality_id: Uuid,
    pub address_region: String,
    pub address_country_id: Uuid,
    pub postal_code: String,
    pub lattitude: f64,
    pub longitude: f64,
    pub additional_info: Option<String>,
}

