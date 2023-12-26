use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    User,
    Admin,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "user_status")]
pub enum UserStatus {
    Available,
    Unavailable,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "gender")]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "staff_level")]
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
#[sqlx(type_name = "task_priority")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "association")]
pub enum Association {
    Sponsor,
    Organizer,
    Media,
    Other,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "employee_level")]
pub enum EmployeeLevel {
    Basic,
    Manager,
    CompanyAdministrator,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "employee_contract")]
pub enum EmployeeContract {
    Dpp,
    Dpc,
    Hpp,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "approval_status")]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    NotRequested,
}

/////////////////////////////
