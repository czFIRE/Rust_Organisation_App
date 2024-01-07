use chrono::NaiveDate;
use sqlx::{types::chrono::NaiveDateTime, FromRow};
use uuid::Uuid;

use crate::{
    models::{AcceptanceStatus, EventRole, Gender, UserRole, UserStatus},
    repositories::{
        company::models::Company, event_staff::models::StaffExtended, user::models::User,
    },
};

#[derive(Debug, Clone)]
pub struct NewAssignedStaff {
    pub task_id: Uuid,
    pub staff_id: Uuid,
}

#[derive(Debug, FromRow)]
pub struct AssignedStaff {
    pub task_id: Uuid,
    pub staff_id: Uuid,
    pub status: AcceptanceStatus,
    pub decided_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct AssignedStaffExtended {
    pub task_id: Uuid,
    pub staff: StaffExtended,
    pub status: AcceptanceStatus,
    pub decided_by: Option<User>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

// TODO - remove this option if not needed
#[derive(Debug, Clone)]
pub struct AssignedStaffData {
    pub status: Option<AcceptanceStatus>,
    pub decided_by: Option<Uuid>,
}

#[derive(Debug)]
pub struct AssignedStaffFilter {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

//////////////////////////////////////

// TODO needs to be kept the same as in user/models.rs => User
// TODO needs to be kept the same as in company/models.rs => Company
// TODO needs to be kept the same as in event/models.rs => Event
// TODO needs to be kept the same as in assigned_staff/models.rs => AssignedStaff

#[derive(Debug, FromRow)]
pub struct AssignedStaffStaffUserCompanyFlattened {
    pub assigned_staff_task_id: Uuid,
    pub assigned_staff_id: Uuid,
    pub assigned_staff_status: AcceptanceStatus,
    pub assigned_staff_decided_by: Option<Uuid>,
    pub assigned_staff_created_at: NaiveDateTime,
    pub assigned_staff_edited_at: NaiveDateTime,
    pub assigned_staff_deleted_at: Option<NaiveDateTime>,

    pub staff_id: Uuid,
    pub staff_user_id: Uuid,
    pub staff_company_id: Uuid,
    pub staff_event_id: Uuid,
    pub staff_role: EventRole,
    pub staff_status: AcceptanceStatus,
    pub staff_decided_by: Option<Uuid>,
    pub staff_created_at: NaiveDateTime,
    pub staff_edited_at: NaiveDateTime,
    pub staff_deleted_at: Option<NaiveDateTime>,

    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub user_birth: NaiveDate,
    pub user_avatar_url: Option<String>, // TODO: Now is the same as in INIT.SQL but do we want this?
    pub user_gender: Gender,
    pub user_role: UserRole,
    pub user_status: UserStatus,
    pub user_created_at: NaiveDateTime,
    pub user_edited_at: NaiveDateTime,
    pub user_deleted_at: Option<NaiveDateTime>,

    pub company_id: Uuid,
    pub company_name: String,
    pub company_description: Option<String>,
    pub company_phone: String,
    pub company_email: String,
    pub company_avatar_url: Option<String>,
    pub company_website: Option<String>,
    pub company_crn: String,
    pub company_vatin: String,
    pub company_created_at: NaiveDateTime,
    pub company_edited_at: NaiveDateTime,
    pub company_deleted_at: Option<NaiveDateTime>,

    pub decided_by_user_id: Option<Uuid>,
    pub decided_by_user_name: Option<String>,
    pub decided_by_user_email: Option<String>,
    pub decided_by_user_birth: Option<NaiveDate>,
    pub decided_by_user_avatar_url: Option<String>, // TODO: Now is the same as in INIT.SQL but do we want this?
    pub decided_by_user_gender: Option<Gender>,
    pub decided_by_user_role: Option<UserRole>,
    pub decided_by_user_status: Option<UserStatus>,
    pub decided_by_user_created_at: Option<NaiveDateTime>,
    pub decided_by_user_edited_at: Option<NaiveDateTime>,
    pub decided_by_user_deleted_at: Option<NaiveDateTime>,
}

impl From<AssignedStaffStaffUserCompanyFlattened> for AssignedStaffExtended {
    fn from(value: AssignedStaffStaffUserCompanyFlattened) -> Self {
        let tmp_user = User {
            id: value.user_id,
            name: value.user_name,
            email: value.user_email,
            birth: value.user_birth,
            avatar_url: value.user_avatar_url,
            gender: value.user_gender,
            role: value.user_role,
            status: value.user_status,
            created_at: value.user_created_at,
            edited_at: value.user_edited_at,
            deleted_at: value.user_deleted_at,
        };

        let tmp_company = Company {
            id: value.company_id,
            name: value.company_name,
            description: value.company_description,
            phone: value.company_phone,
            email: value.company_email,
            avatar_url: value.company_avatar_url,
            website: value.company_website,
            crn: value.company_crn,
            vatin: value.company_vatin,
            created_at: value.company_created_at,
            edited_at: value.company_edited_at,
            deleted_at: value.company_deleted_at,
        };

        let tmp_event_staff = StaffExtended {
            user: tmp_user,
            company: tmp_company,
            event_id: value.staff_event_id,
            role: value.staff_role,
            status: value.staff_status,
            decided_by: value.staff_decided_by,
            created_at: value.staff_created_at,
            edited_at: value.staff_edited_at,
            deleted_at: value.staff_deleted_at,
        };

        let decided_by_user: Option<User> = match value.decided_by_user_id {
            None => None,
            Some(_) => Some(User {
                id: value.decided_by_user_id.unwrap(),
                name: value.decided_by_user_name.unwrap(),
                email: value.decided_by_user_email.unwrap(),
                birth: value.decided_by_user_birth.unwrap(),
                avatar_url: value.decided_by_user_avatar_url,
                gender: value.decided_by_user_gender.unwrap(),
                role: value.decided_by_user_role.unwrap(),
                status: value.decided_by_user_status.unwrap(),
                created_at: value.decided_by_user_created_at.unwrap(),
                edited_at: value.decided_by_user_edited_at.unwrap(),
                deleted_at: value.decided_by_user_deleted_at,
            }),
        };

        Self {
            task_id: value.assigned_staff_task_id,
            staff: tmp_event_staff,
            status: value.assigned_staff_status,
            decided_by: decided_by_user,
            created_at: value.assigned_staff_created_at,
            edited_at: value.assigned_staff_edited_at,
            deleted_at: value.assigned_staff_deleted_at,
        }
    }
}
