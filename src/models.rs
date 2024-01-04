use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    User,
    Admin,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Available,
    Unavailable,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaffLevel {
    Basic,
    Organizer,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcceptanceStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Association {
    Sponsor,
    Organizer,
    Media,
    Other,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmployeeLevel {
    Basic,
    Manager,
    CompanyAdministrator,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmployeeContract {
    Dpp,
    Dpc,
    Hpp,
}

#[derive(Debug, Serialize, Clone, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    NotRequested,
    Pending,
    Accepted,
    Rejected,
}
