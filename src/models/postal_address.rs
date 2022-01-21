use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common_utils::{is_analyst, RoleGuard, Role};


#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
/// Object referring to a geographic location
pub struct PostalAddress {
    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    pub id: Uuid,
    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    pub street_address: String,
    pub address_locality_id: Uuid,
    pub address_region: String,
    pub address_country_id: Uuid,
    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    pub postal_code: String,
    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    pub lattitude: f64,
    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    pub longitude: f64,
    pub additional_info: Option<String>,
}

#[derive(Debug, Clone, Deserialize, InputObject)]
/// Data input object for PostalAddress
pub struct SlimAddress {
    pub street_address: String,
    pub address_locality_id: Uuid,
    pub address_region: String,
    pub address_country_id: Uuid,
    pub postal_code: String,
    pub additional_info: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
/// Insertable version of PostalAddress
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

impl NewPostalAddress {
    pub fn from(slim_address: SlimAddress) -> Self {
        NewPostalAddress {
            street_address: slim_address.street_address.to_owned(),
            address_locality_id: slim_address.address_locality_id,
            address_region: slim_address.address_region.to_owned(),
            address_country_id: slim_address.address_country_id,
            postal_code: slim_address.postal_code.to_owned(),
            lattitude: 0.00, // default
            longitude: 0.00, // default
            additional_info: slim_address.additional_info,
        }
    }
}

