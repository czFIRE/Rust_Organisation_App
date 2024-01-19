use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;

#[derive(Debug, FromRow, Clone)]
pub struct WagePreset {
    pub name: String,
    pub valid_from: NaiveDate,
    pub valid_to: Option<NaiveDate>,
    pub currency: String,
    pub description: String,
    pub monthly_dpp_employee_no_tax_limit: f32,
    pub monthly_dpp_employer_no_tax_limit: f32,
    pub monthly_dpc_employee_no_tax_limit: f32,
    pub monthly_dpc_employer_no_tax_limit: f32,
    pub health_insurance_employee_tax_pct: f32,
    pub social_insurance_employee_tax_pct: f32,
    pub health_insurance_employer_tax_pct: f32,
    pub social_insurance_employer_tax_pct: f32,
    pub min_hourly_wage: f32,
    pub min_monthly_hpp_salary: f32,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
