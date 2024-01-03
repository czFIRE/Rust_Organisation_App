use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::AcceptanceStatus;

#[derive(Deserialize)]
pub struct NewAssignedStaffData {
    staff_id: Uuid,
}

#[derive(Deserialize)]
pub struct AssignedStaffData {
    status: AcceptanceStatus,
    decided_by: Uuid,
}

#[get("/task/{task_id}/staff")]
pub async fn get_all_assigned_staff(_task_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[get("/task/{task_id}/staff/{staff_id}")]
pub async fn get_assigned_staff(
    _task_id: web::Path<String>,
    _staff_id: web::Path<String>,
) -> HttpResponse {
    todo!()
}

#[post("/task/{task_id}/staff")]
pub async fn create_assigned_staff(
    _task_id: web::Path<String>,
    _new_task_staff: web::Form<NewAssignedStaffData>,
) -> HttpResponse {
    todo!()
}

#[patch("/task/{task_id}/staff/{staff_id}")]
pub async fn update_assigned_staff(
    _task_id: web::Path<String>,
    _staff_id: web::Path<String>,
    _task_staff_data: web::Form<AssignedStaffData>,
) -> HttpResponse {
    todo!()
}

#[delete("/task/{task_id}/staff")]
pub async fn delete_not_accepted_assigned_staff(_task_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[delete("task/{task_id}/staff/{staff_id}")]
pub async fn delete_assigned_staff(
    _task_id: web::Path<String>,
    _staff_id: web::Path<String>,
) -> HttpResponse {
    todo!()
}
