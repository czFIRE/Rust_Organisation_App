use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::Type, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type, PartialEq)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum UserStatus {
    Available,
    Unavailable,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "staff_level", rename_all = "lowercase")]
pub enum StaffLevel {
    Basic,
    Organizer,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "acceptance_status", rename_all = "snake_case")]
pub enum AcceptanceStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "task_priority", rename_all = "lowercase")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "association", rename_all = "lowercase")]
pub enum Association {
    Sponsor,
    Organizer,
    Media,
    Other,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "employee_level", rename_all = "lowercase")]
pub enum EmployeeLevel {
    Basic,
    Manager,
    CompanyAdministrator,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "employee_contract", rename_all = "lowercase")]
pub enum EmployeeContract {
    Dpp,
    Dpc,
    Hpp,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "approval_status", rename_all = "lowercase")]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    NotRequested,
}

/////////////////////////////
