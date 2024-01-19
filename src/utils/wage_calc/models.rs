use std::collections::HashMap;

use chrono::{Datelike, NaiveDate};
use serde::Deserialize;

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Deserialize, Debug, Clone)]
pub struct YearAndMonth {
    year: u16,
    month: u8,
}

impl From<NaiveDate> for YearAndMonth {
    fn from(date: NaiveDate) -> Self {
        YearAndMonth {
            year: date.year() as u16,
            month: date.month() as u8,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DetailedWage {
    // A tax value which is used for computing employee's `net wage` and such.
    pub tax_base: f32,
    //
    // A final wage an employee is supposed to be given.
    //
    pub net_wage: f32,

    // Number of worked hours per timesheet or a whole month.
    pub worked_hours: f32,

    // Note: In `wage_currency` units.
    pub employee_social_insurance: f32,
    pub employee_health_insurance: f32,
    pub employer_social_insurance: f32,
    pub employer_health_insurance: f32,
}

impl Default for DetailedWage {
    fn default() -> DetailedWage {
        DetailedWage {
            tax_base: 0.0,
            net_wage: 0.0,
            worked_hours: 0.0,
            employee_social_insurance: 0.0,
            employee_health_insurance: 0.0,
            employer_social_insurance: 0.0,
            employer_health_insurance: 0.0,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TimesheetWageDetailed {
    // A total wage data for selected timesheet's work.
    pub total_wage: DetailedWage,

    pub wage_currency: String,
    pub hourly_wage: f32,

    //
    // A wage employee is supposed to get for selected event's work,
    // divided into months.
    //
    pub month_to_detailed_wage: HashMap<YearAndMonth, DetailedWage>,

    // Note: Empty value means a wage computation went well and data are valid.
    pub error_option: Option<String>,
}

impl Default for TimesheetWageDetailed {
    fn default() -> TimesheetWageDetailed {
        TimesheetWageDetailed {
            total_wage: DetailedWage::default(),
            wage_currency: "".to_string(),
            hourly_wage: 0.0,
            month_to_detailed_wage: HashMap::new(),
            error_option: None,
        }
    }
}
