use serde::{Deserialize, Serialize};
use uuid::Uuid;

use async_graphql::guard::Guard;
use crate::common_utils::{is_analyst, AnalystGuard};


#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct PostalAddress {
    #[graphql(
        guard(AnalystGuard()),
        visible = "is_analyst",
    )]
    pub id: Uuid,
    #[graphql(
        guard(AnalystGuard()),
        visible = "is_analyst",
    )]
    pub street_address: String,
    pub address_locality_id: Uuid,
    pub address_region: String,
    pub address_country_id: Uuid,
    #[graphql(
        guard(AnalystGuard()),
        visible = "is_analyst",
    )]
    pub postal_code: String,
    #[graphql(
        guard(AnalystGuard()),
        visible = "is_analyst",
    )]
    pub lattitude: f64,
    #[graphql(
        guard(AnalystGuard()),
        visible = "is_analyst",
    )]
    pub longitude: f64,
    pub additional_info: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
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

