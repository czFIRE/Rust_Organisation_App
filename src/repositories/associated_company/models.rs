use chrono::NaiveDate;
use sqlx::{types::chrono::NaiveDateTime, FromRow};
use uuid::Uuid;

use crate::{
    models::Association,
    repositories::{
        company::{self, models::Company},
        event::models::Event,
    },
};

#[derive(Debug, FromRow, Clone)]
pub struct AssociatedCompany {
    pub company_id: Uuid,
    pub event_id: Uuid,
    pub association_type: Association,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone)]
pub struct NewAssociatedCompany {
    pub company_id: Uuid,
    pub event_id: Uuid,
    pub association_type: Association,
}

#[derive(Debug, Clone)]
pub struct AssociatedCompanyData {
    pub association_type: Option<Association>,
}

#[derive(Debug, Clone)]
pub struct AssociatedCompanyExtented {
    pub company: Company,
    pub event: Event,
    pub association_type: Association,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, FromRow)]
pub struct AssociatedCompanyFlattened {
    pub company_id: Uuid,
    pub company_name: String,
    pub company_description: Option<String>,
    pub company_phone: String,
    pub company_email: String,
    pub company_avatar_path: Option<String>,
    pub company_website: Option<String>,
    pub company_crn: String,
    pub company_vatin: String,
    pub company_created_at: NaiveDateTime,
    pub company_edited_at: NaiveDateTime,
    pub company_deleted_at: Option<NaiveDateTime>,

    pub event_id: Uuid,
    pub event_name: String,
    pub event_description: Option<String>,
    pub event_website: Option<String>,
    pub event_accepts_staff: bool,
    pub event_start_date: NaiveDate,
    pub event_end_date: NaiveDate,
    pub event_avatar_path: Option<String>,
    pub event_created_at: NaiveDateTime,
    pub event_edited_at: NaiveDateTime,
    pub event_deleted_at: Option<NaiveDateTime>,

    pub association_type: Association,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone)]
pub struct AssociatedCompanyFilter {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl From<AssociatedCompanyFlattened> for AssociatedCompanyExtented {
    fn from(value: AssociatedCompanyFlattened) -> Self {
        let company = Company {
            id: value.company_id,
            name: value.company_name,
            description: value.company_description,
            phone: value.company_phone,
            email: value.company_email,
            avatar_path: value.company_avatar_path,
            website: value.company_website,
            crn: value.company_crn,
            vatin: value.company_vatin,
            created_at: value.company_created_at,
            edited_at: value.company_edited_at,
            deleted_at: value.company_deleted_at,
        };

        let event = Event {
            id: value.event_id,
            name: value.event_name,
            description: value.event_description,
            website: value.event_website,
            accepts_staff: value.event_accepts_staff,
            start_date: value.event_start_date,
            end_date: value.event_end_date,
            avatar_path: value.event_avatar_path,
            created_at: value.event_created_at,
            edited_at: value.event_edited_at,
            deleted_at: value.event_deleted_at,
        };

        Self {
            company,
            event,
            association_type: value.association_type,
            created_at: value.created_at,
            edited_at: value.edited_at,
            deleted_at: value.deleted_at,
        }
    }
}
