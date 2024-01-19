pub mod test_constants {
    use uuid::{uuid, Uuid};

    pub const COMPANY0_ID: Uuid = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f5");
    pub const COMPANY1_ID: Uuid = uuid!("134d5286-5f55-4637-9b98-223a5820a464");
    pub const COMPANY2_ID: Uuid = uuid!("71fa27d6-6f00-4ad0-8902-778e298aaed2");

    pub const USER0_ID: Uuid = uuid!("35341253-da20-40b6-96d8-ce069b1ba5d4");
    pub const USER1_ID: Uuid = uuid!("0465041f-fe64-461f-9f71-71e3b97ca85f");
    pub const USER2_ID: Uuid = uuid!("ac9bf689-a713-4b66-a3d0-41faaf0f8d0c");
    pub const USER3_ID: Uuid = uuid!("51a01dbf-dcd5-43a0-809c-94ed8e61d420");

    pub const EVENT0_ID: Uuid = uuid!("b71fd7ce-c891-410a-9bb4-70fc5c7748f8");
    pub const EVENT1_ID: Uuid = uuid!("3f152d12-0bbd-429a-a9c5-28967d6370cc");

    pub const TIMESHEET0_ID: Uuid = uuid!("d47e8141-a77e-4d55-a2d5-4a77de24b6d0");
    pub const TIMESHEET1_ID: Uuid = uuid!("0f0f0ff5-0073-47cc-bd1f-540a04fee9ea");
    pub const TIMESHEET2_ID: Uuid = uuid!("c51e77aa-bd80-42c7-8b8a-003f018328f6");
    pub const TIMESHEET3_ID: Uuid = uuid!("8446b2ba-8223-4388-be5f-9efdfc4ea265");
    pub const TIMESHEET4_ID: Uuid = uuid!("a19a0ac6-3bd2-4ebd-bc8d-ec111ec9f705");
    pub const TIMESHEET5_ID: Uuid = uuid!("ced9f31c-8662-4812-9005-b8ae85d3b951");

    pub const EVENT_STAFF0_ID: Uuid = uuid!("9281b570-4d02-4096-9136-338a613c71cd");
    pub const EVENT_STAFF1_ID: Uuid = uuid!("a96d1d99-93b5-469b-ac62-654b0cf7ebd3");
    pub const EVENT_STAFF2_ID: Uuid = uuid!("aa7f3d0e-ab48-473b-ac69-b84cb74f34f7");

    pub const TASK0_ID: Uuid = uuid!("7ae0c017-fe31-4aac-b767-100d18a8877b");
    pub const TASK1_ID: Uuid = uuid!("bd9b422d-33c1-42a2-88bf-a56ce6cc55a6");

    pub const COMMENT0_ID: Uuid = uuid!("0d6cec6a-4fe8-4e44-bf68-e33de0ed121b");
    pub const COMMENT1_ID: Uuid = uuid!("daac23ec-fb36-434a-823b-49716ed2002c");
}

#[cfg(test)]
mod calculate_wage_tests {
    use std::sync::Arc;

    use organization::common::DELTA;
    use organization::models::EmploymentContract;

    use chrono::NaiveDate;
    use organization::common::DbResult;
    use organization::repositories::timesheet::timesheet_repo::TimesheetRepository;
    use organization::templates::timesheet::{DetailedWage, TimesheetWageDetailed};
    use organization::utils::calculate_wage::calculate_timesheet_wage;

    use sqlx::PgPool;

    use crate::test_constants::{
        COMPANY1_ID, COMPANY2_ID, TIMESHEET1_ID, TIMESHEET2_ID, TIMESHEET3_ID, USER1_ID, USER3_ID,
    };

    fn check_finished_detailed_wage_result(timesheet_wage_detailed: &TimesheetWageDetailed) {
        let total_wage: &DetailedWage = &timesheet_wage_detailed.total_wage;

        assert!(timesheet_wage_detailed.error_option.is_none());

        assert!(total_wage.tax_base >= total_wage.net_wage);
        assert!(
            total_wage.tax_base
                - (total_wage.net_wage
                    + total_wage.employee_social_insurance
                    + total_wage.employee_social_insurance)
                < DELTA
        );
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn calculate_wage(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let timesheet_repo = TimesheetRepository::new(arc_pool);
        {
            let user_id = USER1_ID;
            let company_id = COMPANY1_ID;
            let date_from = NaiveDate::from_ymd_opt(1969, 07, 28).unwrap();
            let date_to = NaiveDate::from_ymd_opt(1969, 08, 18).unwrap();
            let main_timesheet_id = TIMESHEET1_ID;
            //
            // Get timesheets of an employee who participated at several events
            // within this time period.
            //
            let timesheets_extended = timesheet_repo
                .read_all_with_date_from_to_per_employment_extended_db(
                    user_id, company_id, date_from, date_to,
                )
                .await
                .expect("Should succeed");

            assert_eq!(timesheets_extended.timesheets.len(), 2);
            assert_eq!(timesheets_extended.date_to_wage_presets.len(), 2);

            let timesheet_wage_detailed =
                calculate_timesheet_wage(false, &timesheets_extended, main_timesheet_id)
                    .expect("Should succeed");

            assert_eq!(timesheet_wage_detailed.month_to_detailed_wage.len(), 1);
            check_finished_detailed_wage_result(&timesheet_wage_detailed);
        }

        {
            let user_id = USER1_ID;
            let company_id = COMPANY1_ID;
            let date_from = NaiveDate::from_ymd_opt(1969, 07, 28).unwrap();
            let date_to = NaiveDate::from_ymd_opt(1969, 08, 18).unwrap();
            let main_timesheet_id = TIMESHEET3_ID;
            //
            // Get timesheets of an employee who participated at several events
            // within this time period.
            //
            let timesheets_extended = timesheet_repo
                .read_all_with_date_from_to_per_employment_extended_db(
                    user_id, company_id, date_from, date_to,
                )
                .await
                .expect("Should succeed");

            let timesheet_wage_detailed =
                calculate_timesheet_wage(false, &timesheets_extended, main_timesheet_id)
                    .expect("Should succeed");

            assert_eq!(timesheet_wage_detailed.month_to_detailed_wage.len(), 2);

            check_finished_detailed_wage_result(&timesheet_wage_detailed);
        }

        //
        // Rewrite `hourly_wage` to a value below required minimum
        // and expect error.
        //
        {
            let user_id = USER3_ID;
            let company_id = COMPANY2_ID;
            let date_from = NaiveDate::from_ymd_opt(2024, 01, 01).unwrap();
            let date_to = date_from;
            let main_timesheet_id = TIMESHEET2_ID;

            let mut timesheets_extended = timesheet_repo
                .read_all_with_date_from_to_per_employment_extended_db(
                    user_id, company_id, date_from, date_to,
                )
                .await
                .expect("Should succeed");

            assert_eq!(timesheets_extended.employment_type, EmploymentContract::Dpp);

            timesheets_extended.hourly_wage = 90.0;

            let timesheet_wage_detailed =
                calculate_timesheet_wage(false, &timesheets_extended, main_timesheet_id)
                    .expect("Should succeed");

            assert!(timesheet_wage_detailed.error_option.is_some());
        }

        Ok(())
    }
}
