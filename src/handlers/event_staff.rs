use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::{AcceptanceStatus, StaffLevel};

#[derive(Deserialize)]
pub struct NewEventStaffData {
    user_id: Uuid,
    company_id: Uuid,
    role: StaffLevel,
}

#[derive(Deserialize)]
pub struct EventStaffData {
    role: Option<StaffLevel>,
    status: Option<AcceptanceStatus>,
    decided_by: Option<Uuid>,
}

#[get("/event/{event_id}/staff")]
pub async fn get_all_event_staff(_event_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[get("/event/{event_id}/staff/{staff_id}")]
pub async fn get_event_staff(_event_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[post("/event/{event_id}/staff")]
pub async fn create_event_staff(
    _event_id: web::Path<String>,
    _new_event_staff: web::Form<NewEventStaffData>,
) -> HttpResponse {
    todo!()
}

#[patch("/event/{event_id}/staff/{staff_id}")]
pub async fn update_event_staff(
    _event_id: web::Path<String>,
    _staff_id: web::Path<String>,
    _event_data: web::Form<EventStaffData>,
) -> HttpResponse {
    todo!()
}

#[delete("/event/{event_id}/staff")]
pub async fn delete_all_rejected_event_staff(
    _event_id: web::Path<String>,
    _staff_id: web::Path<String>,
) -> HttpResponse {
    todo!()
}

#[delete("/event/{event_id}/staff/{staff_id}")]
pub async fn delete_event_staff(
    _event_id: web::Path<String>,
    _staff_id: web::Path<String>,
) -> HttpResponse {
    todo!()
}
