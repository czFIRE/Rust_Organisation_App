use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::Type, PartialEq, Clone)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRole::Admin => write!(f, "Admin"),
            UserRole::User => write!(f, "User"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum UserStatus {
    Available,
    Unavailable,
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserStatus::Available => write!(f, "Available"),
            UserStatus::Unavailable => write!(f, "Unavailable"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type, PartialEq, Clone)]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gender::Female => write!(f, "Female"),
            Gender::Male => write!(f, "Male"),
            Gender::Other => write!(f, "Other"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type, PartialEq, Clone)]
#[sqlx(type_name = "event_role", rename_all = "lowercase")]
pub enum EventRole {
    Staff,
    Organizer,
}

impl fmt::Display for EventRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventRole::Organizer => write!(f, "Event Organizer"),
            EventRole::Staff => write!(f, "Staff"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type, PartialEq, Clone)]
#[sqlx(type_name = "acceptance_status", rename_all = "snake_case")]
pub enum AcceptanceStatus {
    Pending,
    Accepted,
    Rejected,
}

impl fmt::Display for AcceptanceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AcceptanceStatus::Accepted => write!(f, "Accepted"),
            AcceptanceStatus::Rejected => write!(f, "Rejected"),
            AcceptanceStatus::Pending => write!(f, "Pending"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type, PartialEq, Clone)]
#[sqlx(type_name = "task_priority", rename_all = "lowercase")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

impl fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskPriority::High => write!(f, "High"),
            TaskPriority::Medium => write!(f, "Medium"),
            TaskPriority::Low => write!(f, "Low"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "association", rename_all = "lowercase")]
pub enum Association {
    Sponsor,
    Organizer,
    Media,
    Other,
}

impl fmt::Display for Association {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Association::Sponsor => write!(f, "Sponsor"),
            Association::Organizer => write!(f, "Organizer"),
            Association::Media => write!(f, "Media"),
            _ => write!(f, "Other"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "employee_level", rename_all = "lowercase")]
pub enum EmployeeLevel {
    Basic,
    Manager,
    CompanyAdministrator,
}

impl fmt::Display for EmployeeLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmployeeLevel::Basic => write!(f, "Basic"),
            EmployeeLevel::Manager => write!(f, "Manager"),
            EmployeeLevel::CompanyAdministrator => write!(f, "Company Administrator"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "employment_contract", rename_all = "lowercase")]
pub enum EmploymentContract {
    Dpp,
    Dpc,
    Hpp,
}

impl fmt::Display for EmploymentContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmploymentContract::Dpp => write!(f, "Dpp"),
            EmploymentContract::Dpc => write!(f, "Dpc"),
            EmploymentContract::Hpp => write!(f, "Hpp"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "approval_status", rename_all = "snake_case")]
pub enum ApprovalStatus {
    Pending,
    Accepted,
    Rejected,
    NotRequested,
}

impl fmt::Display for ApprovalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApprovalStatus::Accepted => write!(f, "Accepted"),
            ApprovalStatus::Rejected => write!(f, "Rejected"),
            ApprovalStatus::Pending => write!(f, "Pending"),
            ApprovalStatus::NotRequested => write!(f, "Not Requested"),
        }
    }
}
