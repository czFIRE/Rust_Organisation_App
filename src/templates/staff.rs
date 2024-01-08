use crate::repositories::{event_staff::models::StaffLite, user::models::User};
use askama::Template;
use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::types::uuid;
use uuid::Uuid;

use crate::{
    models::{AcceptanceStatus, EventRole},
    repositories::{
        assigned_staff::models::AssignedStaffExtended, event_staff::models::StaffExtended,
    },
};

use super::{company::CompanyLiteTemplate, user::UserLiteTemplate};

#[derive(Template, Deserialize, Debug)]
#[template(path = "event/staff/staff.html")]
pub struct StaffTemplate {
    pub id: Uuid,
    pub user: UserLiteTemplate,
    pub company: CompanyLiteTemplate,
    pub event_id: Uuid,
    pub role: EventRole,
    pub status: AcceptanceStatus,
    pub decided_by: Option<Uuid>,
    pub decided_by_user: Option<UserLiteTemplate>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

impl From<StaffExtended> for StaffTemplate {
    fn from(staff: StaffExtended) -> Self {
        let user = UserLiteTemplate {
            id: staff.user.id,
            name: staff.user.name,
            status: staff.user.status,
            age: chrono::offset::Local::now()
                .naive_local()
                .date()
                .years_since(staff.user.birth)
                .expect("Should be valid"),
            gender: staff.user.gender,
            avatar_url: staff.user.avatar_url,
        };

        let company = CompanyLiteTemplate {
            id: staff.company.id,
            name: staff.company.name,
            avatar_url: staff.company.avatar_url,
        };

        let decided_by: Option<UserLiteTemplate>;
        if staff.decided_by.is_some() {
            let decider = staff.decided_by_user.unwrap();
            decided_by = Some(decider.into());
        } else {
            decided_by = None;
        }

        StaffTemplate {
            id: staff.id,
            user,
            company,
            event_id: staff.event_id,
            role: staff.role,
            status: staff.status,
            decided_by: staff.decided_by,
            decided_by_user: decided_by,
            created_at: staff.created_at,
            edited_at: staff.edited_at,
        }
    }
}

#[derive(Template, Deserialize)]
#[template(path = "event/staff/all-staff.html")]
pub struct AllStaffTemplate {
    pub staff: Vec<StaffTemplate>,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "event/staff/task-staff.html")]
pub struct AssignedStaffTemplate {
    pub task_id: Uuid,
    pub staff: StaffLite,
    pub status: AcceptanceStatus,
    pub decided_by: Option<Uuid>,
    pub decided_by_user: Option<User>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

impl From<AssignedStaffExtended> for AssignedStaffTemplate {
    fn from(value: AssignedStaffExtended) -> Self {
        AssignedStaffTemplate {
            task_id: value.task_id,
            staff: value.staff,
            status: value.status,
            decided_by: value.decided_by,
            decided_by_user: value.decided_by_user,
            created_at: value.created_at,
            edited_at: value.edited_at,
        }
    }
}

#[derive(Template, Deserialize)]
#[template(path = "event/staff/all-task-staff.html")]
pub struct AllAssignedStaffTemplate {
    pub staff: Vec<AssignedStaffTemplate>,
}
