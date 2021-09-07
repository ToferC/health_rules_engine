use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessLog {
    uuid: String,
    person_uid: String,
    voyage_uid: String,
    access_key: String,
    rationale: AccessRationale,
    granularity: Granularity,
    requested_on: NaiveDateTime,
    decision_on: NaiveDateTime,
    approval_type: ApprovalType,
    approved: bool,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AccessRationale {
    Automated,
    Administrative,
    Research,
    PublicHealth,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Granularity {
    Aggregated,
    Anonymized,
    Identifiable,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ApprovalType {
    LegalRequirement,
    PersonPreApproved,
    PersonResponse,
    LegalGuardianResponse,
    LegalGuardianPreApproved,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AccessLevel {
    Adminstrator,
    Analyst,
    Employee,
    Researcher,
    Open,

}

