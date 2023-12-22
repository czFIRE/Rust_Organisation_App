use askama::Template;
use sqlx::types::uuid;
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::models::{StaffLevel, AcceptanceStatus};

use super::{company::CompanyLiteTemplate, user::UserLiteTemplate};

#[derive(Template)]
#[template(path = "staff/staff.html")]
pub struct StaffTemplate {
    pub id: Uuid,
    pub user: UserLiteTemplate,
    pub company: CompanyLiteTemplate,
    pub event_id: Uuid,
    pub role: StaffLevel,
    pub status: AcceptanceStatus,
    pub decided_by: UserLiteTemplate,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime
}

#[derive(Template)]
#[template(path = "staff/task-staff.html")]
pub struct TaskStaffTemplate {
    pub id: Uuid,
    pub user: StaffTemplate,
    pub status: AcceptanceStatus,
    pub decided_by: UserLiteTemplate
}