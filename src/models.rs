use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    User,
    Admin,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Available,
    Unavailable,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Serialize, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaffLevel {
    Basic,
    Organizer,
}

#[derive(Serialize, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcceptanceStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Association {
    Sponsor,
    Organizer,
    Media,
    Other,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmployeeLevel {
    Basic,
    Manager,
    CompanyAdministrator,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmployeeContract {
    Dpp,
    Dpc,
    Hpp,
}
