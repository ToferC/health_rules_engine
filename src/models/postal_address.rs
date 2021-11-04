use serde::{Deserialize, Serialize};
use uuid::Uuid;

use async_graphql::guard::Guard;
use crate::common_utils::{is_analyst, AssociatedGuardAnalyst};


#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct PostalAddress {
    #[graphql(
        guard(AssociatedGuardAnalyst()),
        visible = "is_analyst",
    )]
    pub id: Uuid,
    #[graphql(
        guard(AssociatedGuardAnalyst()),
        visible = "is_analyst",
    )]
    pub street_address: String,
    pub address_locality_id: Uuid,
    pub address_region: String,
    pub address_country_id: Uuid,
    #[graphql(
        guard(AssociatedGuardAnalyst()),
        visible = "is_analyst",
    )]
    pub postal_code: String,
    #[graphql(
        guard(AssociatedGuardAnalyst()),
        visible = "is_analyst",
    )]
    pub lattitude: f64,
    #[graphql(
        guard(AssociatedGuardAnalyst()),
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

