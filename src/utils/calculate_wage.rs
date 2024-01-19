//
// todo: Some code parts was written in a rush, a bit of refactoring needed.
//

use crate::{
    repositories::wage_preset::models::{WagePreset},
};

use crate::templates::timesheet::{
    DetailedWage,
    TimesheetWageDetailed,
};

use crate::repositories::timesheet::models::{
    Workday, TimesheetWithEvent, TimesheetsWithWorkdaysExtended
};
use crate::models::EmploymentContract;
use crate::utils::year_and_month::YearAndMonth;

use uuid::Uuid;

use std::collections::HashMap;

/// Info about workdays aggregated to months (belonging to a same timesheet).
#[derive(Debug, Clone)]
pub struct WorkdaysInfo {
    workdays: Vec<Workday>,
    total_hours: f32,
    // Note: This is called `zaklad dane` in Czech.
    tax_base: f32,
}

impl WorkdaysInfo {
    fn compute_total_hours(&mut self) {
        for workday in self.workdays.iter() {
            self.total_hours += workday.total_hours;
        }
    }
    fn compute_tax_base(&mut self,
                        hourly_wage: f32) {
        self.tax_base = self.total_hours * hourly_wage;
    }
}

#[derive(Debug, Clone)]
pub struct TimesheetWithClassifiedWorkdays {
    pub timesheet: TimesheetWithEvent,
    pub date_to_workdays_info: HashMap<YearAndMonth, WorkdaysInfo>,
}

//
/// A reduced wage preset which has some values precomputed.
///
#[derive(Debug)]
struct WagePresetOptimized {
    name: String,
    currency: String,
    monthly_employee_no_tax_limit: f32,
    monthly_employer_no_tax_limit: f32,
    //
    // Precomputed factors.
    //
    health_insurance_employee_tax_multiplicand: f32,
    social_insurance_employee_tax_multiplicand: f32,
    health_insurance_employer_tax_multiplicand: f32,
    social_insurance_employer_tax_multiplicand: f32,
    min_hourly_wage: f32,
    min_monthly_hpp_salary: f32,
}

impl WagePresetOptimized {
    // Construct a new instance using `WagePreset`.
    fn new_wage_preset(wage_preset: &WagePreset,
                       monthly_employee_no_tax_limit: f32,
                       monthly_employer_no_tax_limit: f32)
                       -> Self {

        WagePresetOptimized {
            name: wage_preset.name.clone(),
            currency: wage_preset.currency.clone(),
            monthly_employee_no_tax_limit: monthly_employee_no_tax_limit,
            monthly_employer_no_tax_limit: monthly_employer_no_tax_limit,
            health_insurance_employee_tax_multiplicand:
            wage_preset.health_insurance_employee_tax_pct / 100.0,
            social_insurance_employee_tax_multiplicand:
            wage_preset.social_insurance_employee_tax_pct / 100.0,
            health_insurance_employer_tax_multiplicand:
            wage_preset.health_insurance_employer_tax_pct / 100.0,
            social_insurance_employer_tax_multiplicand:
            wage_preset.social_insurance_employer_tax_pct / 100.0,
            min_hourly_wage: wage_preset.min_hourly_wage,
            min_monthly_hpp_salary: wage_preset.min_monthly_hpp_salary,
        }
    }
}

//
// todo later: ATM we don't implement the pink_paper discount
//             is limited per month.
//
fn compute_monthly_dpp_or_dpc_wage(
    pink_paper_signed: bool,
    wanted_workdays_info: &WorkdaysInfo,
    preset: &WagePresetOptimized,
    related_workdays_tax_base: f32)
    -> DetailedWage {

    let mut monthly_wage = DetailedWage::default();

    // Employee's monthly tax base across **all** timesheets.
    let monthly_total_tax_base
        = wanted_workdays_info.tax_base
        + related_workdays_tax_base;

    monthly_wage.worked_hours = wanted_workdays_info.total_hours;

    if monthly_total_tax_base >= preset.monthly_employee_no_tax_limit {
        monthly_wage.employee_health_insurance
            = wanted_workdays_info.tax_base
            * preset.health_insurance_employee_tax_multiplicand;

        monthly_wage.employee_social_insurance
            = wanted_workdays_info.tax_base
            * preset.social_insurance_employee_tax_multiplicand;
    }

    if monthly_total_tax_base >= preset.monthly_employer_no_tax_limit {
        monthly_wage.employer_health_insurance
            = wanted_workdays_info.tax_base
            * preset.health_insurance_employer_tax_multiplicand;

        monthly_wage.employer_social_insurance
            = wanted_workdays_info.tax_base
            * preset.social_insurance_employer_tax_multiplicand;
    }

    monthly_wage.tax_base = wanted_workdays_info.tax_base;

    // Initialize the `net wage`.
    monthly_wage.net_wage = monthly_wage.tax_base;

    if !pink_paper_signed {
        monthly_wage.net_wage
            -= monthly_wage.employee_health_insurance
            + monthly_wage.employee_social_insurance;
    }

    monthly_wage
}

fn compute_tax_base_of_workdays(
    related_timesheets: &Vec<TimesheetWithClassifiedWorkdays>,
    year_month: &YearAndMonth) -> f32 {

    let mut total_tax_base = 0.0;
    for sheet in related_timesheets.iter() {
        if let Some(workdays_info) = sheet.date_to_workdays_info.get(year_month) {
            total_tax_base += workdays_info.tax_base;
        }
    }
    total_tax_base
}

fn compute_dpp_or_dpc_wage(
    pink_paper_signed: bool,
    wanted_timesheet: &TimesheetWithClassifiedWorkdays,
    date_to_wage_presets: &HashMap<YearAndMonth, Option<WagePreset>>,
    hourly_wage: f32,
    employment_type: EmploymentContract,
    related_timesheets: &Vec<TimesheetWithClassifiedWorkdays>)
    -> Result<TimesheetWageDetailed, String> {

    let mut total_wage_output: TimesheetWageDetailed
        = TimesheetWageDetailed::default();

    // Go through each month of `wanted timesheet`
    for (year_month, wanted_workdays_info) in &wanted_timesheet.date_to_workdays_info {

        let wage_preset = date_to_wage_presets.get(year_month).unwrap().clone().unwrap();
        let monthly_employee_no_tax_limit;
        let monthly_employer_no_tax_limit;

        match employment_type {
            EmploymentContract::Dpp => {

                if hourly_wage < wage_preset.min_hourly_wage {
                    return Err(
                        "The hourly_wage of DPP agreement is below a required minimum."
                            .to_string());
                }
                monthly_employee_no_tax_limit
                    = wage_preset.monthly_dpp_employee_no_tax_limit;
                monthly_employer_no_tax_limit
                    = wage_preset.monthly_dpp_employer_no_tax_limit
            }
            EmploymentContract::Dpc => {

                monthly_employee_no_tax_limit
                    = wage_preset.monthly_dpc_employee_no_tax_limit;
                monthly_employer_no_tax_limit
                    = wage_preset.monthly_dpc_employer_no_tax_limit
            },
            EmploymentContract::Hpp => unreachable!("Bug in code.")
        }

        let wage_preset_optimized
            = WagePresetOptimized::new_wage_preset(
                &wage_preset,
                monthly_employee_no_tax_limit,
                monthly_employer_no_tax_limit,
            );

        let related_workdays_tax_base
            = compute_tax_base_of_workdays(&related_timesheets, year_month);


        let monthly_wage_output
            = compute_monthly_dpp_or_dpc_wage(
                pink_paper_signed,
                &wanted_workdays_info,
                &wage_preset_optimized,
                related_workdays_tax_base
            );

        total_wage_output.total_wage.tax_base += monthly_wage_output.tax_base;
        total_wage_output.total_wage.net_wage += monthly_wage_output.net_wage;
        total_wage_output.total_wage.worked_hours += monthly_wage_output.worked_hours;
        total_wage_output.total_wage.employee_social_insurance
            += monthly_wage_output.employee_social_insurance;
        total_wage_output.total_wage.employee_health_insurance
            += monthly_wage_output.employee_health_insurance;
        total_wage_output.total_wage.employer_social_insurance
            += monthly_wage_output.employer_social_insurance;
        total_wage_output.total_wage.employer_health_insurance
            += monthly_wage_output.employer_health_insurance;

        total_wage_output.month_to_detailed_wage.insert(
            year_month.clone(), monthly_wage_output);
    }

    Ok(total_wage_output)
}

//
// Divides workdays into equivalence classes where all elems have a same
// year and month.
//
fn classify_workdays(workdays: &Vec<Workday>)
                     -> HashMap::<YearAndMonth, WorkdaysInfo> {
    let mut date_to_workdays_info = HashMap::<YearAndMonth, WorkdaysInfo>::new();

    for workday in workdays.iter() {
        let year_month: YearAndMonth = workday.date.into();

        if let Some(workdays_info) = date_to_workdays_info.get_mut(&year_month) {
            workdays_info.workdays.push(workday.clone());
        } else {
            let workdays_info = WorkdaysInfo {
                workdays: vec![ workday.clone() ],
                // Note: Gets computed later.
                total_hours: 0.0,
                tax_base: 0.0,
            };
            date_to_workdays_info.insert(year_month, workdays_info);
        }
    }

    date_to_workdays_info
}

pub fn calculate_hpp_or_dpp_or_dpc_wage(
    pink_paper_signed: bool,
    wanted_timesheet: &TimesheetWithClassifiedWorkdays,
    date_to_wage_presets: &HashMap<YearAndMonth, Option<WagePreset>>,
    hourly_wage: f32,
    employment_type: EmploymentContract,
    related_timesheets: &Vec<TimesheetWithClassifiedWorkdays>)
    -> Result<TimesheetWageDetailed, String> {

    match employment_type {
        EmploymentContract::Dpp => (),
        EmploymentContract::Dpc => (),
        EmploymentContract::Hpp => return Err("Hpp is not supported.".to_string()),
    }

    compute_dpp_or_dpc_wage(
        pink_paper_signed,
        wanted_timesheet,
        date_to_wage_presets,
        hourly_wage,
        employment_type,
        related_timesheets)
}

//
// Get some employee's detailed wage info per a wanted timesheet.
//
// Note: In order to return a correct value, the wage should get computed
//       after an employee submitted all work done in all months event got
//       organized in (this includes other events' work for a same employer).
//
//       A reason behind this is that `tax base` is computed from employee's
//       **total hours per each month** which is not limited to a single event.
//
//
// Note: Both `DPP` and `DPC` agreement needs to contain `hourly_wage`.
//
//       A main difference is that `DPC` has lower `no tax limit`
//       and the `DPP` limits a number of work hours per year.
//
//       There are some other differences that we don't deal with
//       (e.g. a person currently registered at `employment agency`
//       **cannot** sign any `DPP` agreement).
//
pub fn calculate_timesheet_wage(
    pink_paper_signed: bool,
    timesheets_extended: &TimesheetsWithWorkdaysExtended,
    wanted_timesheet_id: Uuid,
) -> Result<TimesheetWageDetailed, String> {

    let mut detailed_wage_output = TimesheetWageDetailed::default();

    // Check all wage presets are valid.
    for preset in timesheets_extended.date_to_wage_presets.values() {
        if preset.is_none() {
            detailed_wage_output.error_option = Some(
                "Some workday was missing a wage preset".to_string());
            return Ok(detailed_wage_output)
        }
    }

    let mut wanted_timesheet: Option<TimesheetWithClassifiedWorkdays> = None;
    //
    // Related timesheets with any event's workdays of wanted employee
    // (thus limited to specific employment only, ) which intersect
    // with months of workdays of `wanted_timesheet`.
    //
    // Note: A point is to first compute all worked_hours
    //       in a specific month (per a single `employment`)
    //       as only then we can start computing taxes.
    //
    let mut related_timesheets = Vec::new();

    //
    // Find one `wanted timesheet` and (up to several) `related timesheets`
    // and aggregate their workdays by individual months.
    //
    for timesheet in timesheets_extended.timesheets.iter() {
        let mut date_to_workdays_info = classify_workdays(&timesheet.workdays);

        // Compute individual `tax_base` values.
        for workdays_info in date_to_workdays_info.values_mut() {
            workdays_info.compute_total_hours();
            workdays_info.compute_tax_base(timesheets_extended.hourly_wage as f32);
        }

        let timesheet_with_classified_workdays
            = TimesheetWithClassifiedWorkdays {
                timesheet: timesheet.timesheet.clone(),
                date_to_workdays_info,
            };

        // If it's a timesheet we compute wage for.
        if timesheet.timesheet.id == wanted_timesheet_id {
            wanted_timesheet = Some(timesheet_with_classified_workdays);
        } else {
            related_timesheets.push(timesheet_with_classified_workdays);
        }
    }

    if wanted_timesheet.is_none() {
        return Err("No timesheet with 'wanted_timesheet_id' was found."
                   .to_string())
    }

    let wage_output_result = calculate_hpp_or_dpp_or_dpc_wage(
        pink_paper_signed,
        &wanted_timesheet.unwrap(),
        &timesheets_extended.date_to_wage_presets,
        timesheets_extended.hourly_wage as f32,
        timesheets_extended.employment_type.clone(),
        &related_timesheets);

    match wage_output_result {
        Err(msg) => detailed_wage_output.error_option = Some(msg),
        Ok(wage_output) => detailed_wage_output = wage_output,
    }

    detailed_wage_output.hourly_wage = timesheets_extended.hourly_wage as f32;
    // We use CZK only for now.
    detailed_wage_output.wage_currency = "CZK".to_string();

    Ok(detailed_wage_output)
}