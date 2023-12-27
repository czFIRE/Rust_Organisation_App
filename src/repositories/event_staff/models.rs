use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    models::{AcceptanceStatus, Gender, StaffLevel, UserRole, UserStatus},
    repositories::{company::models::Company, user::models::User},
};

#[derive(Debug)]
pub struct NewStaff {
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub event_id: Uuid,
    pub role: StaffLevel,
}

#[derive(Debug, FromRow)]
pub struct Staff {
    pub id: Uuid,
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub event_id: Uuid,
    pub role: StaffLevel,
    pub status: AcceptanceStatus,
    pub decided_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, FromRow)]
pub struct StaffExtended {
    pub user: User,
    pub company: Company,
    pub event_id: Uuid,
    pub role: StaffLevel,
    pub status: AcceptanceStatus,
    pub decided_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct StaffData {
    pub role: Option<StaffLevel>,
    pub status: Option<AcceptanceStatus>,
    pub decided_by: Option<Uuid>,
}

#[derive(Debug)]
pub struct StaffFilter {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

////////////////////////////////////////////

// TODO needs to be kept the same as in user/models.rs => User
// TODO needs to be kept the same as in company/models.rs => Company
// TODO needs to be kept the same as in event/models.rs => Event
#[derive(Debug, FromRow)]
pub struct StaffUserCompanyFlattened {
    pub staff_id: Uuid,
    pub staff_user_id: Uuid,
    pub staff_company_id: Uuid,
    pub staff_event_id: Uuid,
    pub staff_role: StaffLevel,
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
}

impl From<StaffUserCompanyFlattened> for StaffExtended {
    fn from(value: StaffUserCompanyFlattened) -> Self {
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

        Self {
            user: tmp_user,
            company: tmp_company,
            event_id: value.staff_event_id,
            role: value.staff_role,
            status: value.staff_status,
            decided_by: value.staff_decided_by,
            created_at: value.staff_created_at,
            edited_at: value.staff_edited_at,
            deleted_at: value.staff_deleted_at,
        }
    }
}
