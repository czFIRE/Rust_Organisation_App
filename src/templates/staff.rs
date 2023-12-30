use askama::Template;
use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::types::uuid;
use uuid::Uuid;

use crate::models::{AcceptanceStatus, StaffLevel};

use super::{company::CompanyLiteTemplate, user::UserLiteTemplate};

#[derive(Template, Deserialize, Debug)]
#[template(path = "event/staff/staff.html")]
pub struct StaffTemplate {
    pub id: Uuid,
    pub user: UserLiteTemplate,
    pub company: CompanyLiteTemplate,
    pub event_id: Uuid,
    pub role: StaffLevel,
    pub status: AcceptanceStatus,
    pub decided_by: UserLiteTemplate,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

#[derive(Template, Deserialize)]
#[template(path = "event/staff/all-staff.html")]
pub struct AllStaffTemplate {
    pub staff: Vec<StaffTemplate>,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "event/staff/task-staff.html")]
pub struct TaskStaffTemplate {
    pub id: Uuid,
    pub user: StaffTemplate,
    pub status: AcceptanceStatus,
    pub decided_by: UserLiteTemplate,
}

#[derive(Template, Deserialize)]
#[template(path = "event/staff/all-task-staff.html")]
pub struct AllStaffTaskTemplate {
    pub staff: Vec<TaskStaffTemplate>,
}