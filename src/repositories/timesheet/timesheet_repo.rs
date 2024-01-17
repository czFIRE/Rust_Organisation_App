use crate::common::DbResult;
use crate::models::ApprovalStatus;
use crate::repositories::timesheet::models::{
    TimesheetCreateData, TimesheetReadAllData, TimesheetStructureData, TimesheetUpdateData,
    TimesheetWithEvent, TimesheetWithWorkdays, Workday, TimesheetsWithWorkdaysExtended,
};

use crate::repositories::wage_preset::{
    models::WagePreset,
    wage_preset_repo,
};

use crate::repositories::employment::employment_repo;

use std::collections::HashMap;
use chrono::{Duration, NaiveDate, Datelike, Months};
use sqlx::postgres::PgPool;
use sqlx::{Postgres, Transaction};
use std::ops::DerefMut;
use std::sync::Arc;
use uuid::Uuid;

/// Reads workdays of a specific timesheet that match a requested date range.
async fn read_some_timesheet_workdays_db_using_tx(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    timesheet_id: Uuid,
    date_from: NaiveDate,
    date_to: NaiveDate)
    -> DbResult<Vec<Workday>> {

    sqlx::query_as!(
        Workday,
        r#"
        SELECT timesheet_id,
               date,
               total_hours,
               comment AS "comment?",
               is_editable,
               created_at,
               edited_at
        FROM workday
        WHERE timesheet_id = $1
          AND date >= $2
          AND date <= $3
        ORDER BY date;
        "#,
        timesheet_id,
        date_from,
        date_to,
    )
    .fetch_all(tx.deref_mut())
        .await
}

/// Reads all workdays of a specific timesheet.
async fn read_all_timesheet_workdays_db_using_tx(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    timesheet: &TimesheetWithEvent)
    -> DbResult<Vec<Workday>> {

    //
    // Pass timesheet's start and end date respectively
    // in order to get all its workdays.
    //
    read_some_timesheet_workdays_db_using_tx(
        tx,
        timesheet.id,
        timesheet.start_date,
        timesheet.end_date,
    ).await
}

///
/// Reads all timesheets of specific employee (along with their workdays)
/// that at least partially fall into a requested date range.
///
pub async fn read_all_with_date_from_to_per_employment_db_using_tx(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: Uuid,
    company_id: Uuid,
    date_from: NaiveDate,
    date_to: NaiveDate,
    omit_workdays_outside_of_date_range: bool,
) -> DbResult<Vec<TimesheetWithWorkdays>> {

    if date_from > date_to {
        //
        // todo later: Return a meaningful error type, sqlx::error::Error
        //             does not seem to contain one.
        //
        return Err(sqlx::Error::RowNotFound);
    }

    // Get all timesheets which match a required date range.
    let timesheets = sqlx::query_as!(
        TimesheetWithEvent,
        r#"
            SELECT timesheet.id,
                   timesheet.start_date,
                   timesheet.end_date,
                   total_hours,
                   is_editable,
                   status AS "approval_status!: ApprovalStatus",
                   manager_note AS "manager_note?",
                   user_id,
                   company_id,
                   event_id,
                   event.avatar_url AS event_avatar_url,
                   event.name AS event_name,
                   timesheet.created_at,
                   timesheet.edited_at
            FROM timesheet
             JOIN event ON timesheet.event_id = event.id
            WHERE user_id = $1
              AND company_id = $2
              AND timesheet.start_date <= $3
              AND timesheet.end_date >= $4
              AND timesheet.deleted_at IS NULL;
            "#,
        user_id,
        company_id,
        date_to,
        date_from,
    )
        .fetch_all(tx.deref_mut())
        .await?;

    let mut timesheets_with_workdays = vec![];

    if timesheets.is_empty() {
        return Ok(timesheets_with_workdays);
    }

    // Iterate each timesheet and attach its workdays.
    for timesheet in timesheets.iter() {
        let workdays = match omit_workdays_outside_of_date_range {
            true => {
                read_some_timesheet_workdays_db_using_tx(
                    tx, timesheet.id,
                    date_from, date_to)
                    .await?

            },
            false => {
                read_all_timesheet_workdays_db_using_tx(
                    tx, &timesheet.clone())
                    .await?
            }
        };

        timesheets_with_workdays.push(
            TimesheetWithWorkdays{
                timesheet: timesheet.clone(),
                workdays,
            }
        );
    }

    Ok(timesheets_with_workdays)
}

#[derive(Clone)]
pub struct TimesheetRepository {
    pub pool: Arc<PgPool>,
}

impl TimesheetRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    // CRUD
    async fn create_workdays(
        &self,
        mut tx: Transaction<'_, Postgres>,
        timesheet_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> DbResult<Vec<Workday>> {
        let mut workdays = vec![];
        let mut workdate = start_date;
        while workdate <= end_date {
            let workday = sqlx::query_as!(
                Workday,
                r#"
                INSERT INTO workday (timesheet_id,
                                     date,
                                     is_editable)
                VALUES ($1, $2, $3)
                RETURNING timesheet_id,
                          date,
                          total_hours,
                          comment,
                          is_editable,
                          created_at,
                          edited_at;
                "#,
                timesheet_id,
                workdate,
                true
            )
            .fetch_one(tx.deref_mut())
            .await?;
            workdays.push(workday);
            workdate += Duration::days(1);
        }
        tx.commit().await?;
        Ok(workdays)
    }

    pub async fn create(&self, data: TimesheetCreateData) -> DbResult<TimesheetWithWorkdays> {
        let mut tx = self.pool.begin().await?;

        let timesheet_structure = sqlx::query_as!(
            TimesheetStructureData,
            r#"
            INSERT INTO timesheet (start_date, end_date, user_id, company_id, event_id) 
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, start_date, end_date;
            "#,
            data.start_date,
            data.end_date,
            data.user_id,
            data.company_id,
            data.event_id
        )
        .fetch_one(tx.deref_mut())
        .await?;

        // ToDo: This can probably be done better, but we need the event there as well.
        let timesheet = sqlx::query_as!(
            TimesheetWithEvent,
            r#"
            SELECT timesheet.id, 
                   timesheet.start_date, 
                   timesheet.end_date, 
                   total_hours, 
                   is_editable, 
                   status AS "approval_status!:ApprovalStatus", 
                   manager_note AS "manager_note?", 
                   user_id, 
                   company_id,
                   event_id,
                   event.avatar_url AS event_avatar_url,
                   event.name AS event_name,
                   timesheet.created_at, 
                   timesheet.edited_at
            FROM timesheet 
            JOIN event ON timesheet.event_id = event.id
            WHERE timesheet.id = $1 
              AND timesheet.deleted_at IS NULL;
            "#,
            timesheet_structure.id
        )
        .fetch_one(tx.deref_mut())
        .await?;

        let workdays = self
            .create_workdays(
                tx,
                timesheet_structure.id,
                timesheet_structure.start_date,
                timesheet_structure.end_date,
            )
            .await?;

        // Tx commit in create_workdays

        let result = TimesheetWithWorkdays {
            timesheet,
            workdays,
        };

        Ok(result)
    }

    pub async fn _read_one(&self, timesheet_id: Uuid) -> DbResult<TimesheetWithWorkdays> {
        let mut tx = self.pool.begin().await?;

        let timesheet = sqlx::query_as!(
            TimesheetWithEvent,
            r#"
            SELECT timesheet.id, 
                   timesheet.start_date, 
                   timesheet.end_date, 
                   total_hours, 
                   is_editable, 
                   status AS "approval_status!: ApprovalStatus", 
                   manager_note AS "manager_note?", 
                   user_id, 
                   company_id,
                   event_id,
                   event.avatar_url AS event_avatar_url,
                   event.name AS event_name,
                   timesheet.created_at, 
                   timesheet.edited_at
            FROM timesheet 
            JOIN event ON timesheet.event_id = event.id
            WHERE timesheet.id = $1 
              AND timesheet.deleted_at IS NULL;
            "#,
            timesheet_id
        )
        .fetch_optional(tx.deref_mut())
        .await?;

        if timesheet.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        let sheet_clone = timesheet.clone().unwrap();

        let workdays = read_all_timesheet_workdays_db_using_tx(
            &mut tx, &sheet_clone).await?;

        let result = TimesheetWithWorkdays {
            timesheet: timesheet.expect("Should be valid here."),
            workdays,
        };

        tx.commit().await?;

        Ok(result)
    }

    // Warning!! The tx will be commited, use this as the last call in a transaction.
    async fn _read_one_tx(
        &self,
        timesheet_id: Uuid,
        mut tx: Transaction<'_, Postgres>,
    ) -> DbResult<TimesheetWithEvent> {
        let timesheet = sqlx::query_as!(
            TimesheetWithEvent,
            r#"
            SELECT timesheet.id, 
                   timesheet.start_date, 
                   timesheet.end_date, 
                   total_hours, 
                   is_editable, 
                   status AS "approval_status!: ApprovalStatus", 
                   manager_note AS "manager_note?", 
                   user_id, 
                   company_id,
                   event_id,
                   event.avatar_url AS event_avatar_url,
                   event.name AS event_name,
                   timesheet.created_at,
                   timesheet.edited_at
            FROM timesheet 
            JOIN event ON timesheet.event_id = event.id
            WHERE timesheet.id = $1 
              AND timesheet.deleted_at IS NULL;
            "#,
            timesheet_id
        )
        .fetch_optional(tx.deref_mut())
        .await?;

        if timesheet.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        tx.commit().await?;

        Ok(timesheet.expect("Should be valid here."))
    }

    pub async fn _read_all(&self, data: TimesheetReadAllData) -> DbResult<Vec<TimesheetWithEvent>> {
        // This is for redis.

        self._read_all_db(data).await
    }

    async fn _read_all_db(&self, data: TimesheetReadAllData) -> DbResult<Vec<TimesheetWithEvent>> {
        let executor = self.pool.as_ref();

        let timesheets = sqlx::query_as!(
            TimesheetWithEvent,
            r#"
            SELECT timesheet.id, 
                   timesheet.start_date, 
                   timesheet.end_date, 
                   total_hours, 
                   is_editable, 
                   status AS "approval_status!: ApprovalStatus", 
                   manager_note AS "manager_note?", 
                   user_id, 
                   company_id,
                   event_id,
                   event.avatar_url AS event_avatar_url,
                   event.name AS event_name,
                   timesheet.created_at, 
                   timesheet.edited_at
            FROM timesheet 
             JOIN event ON timesheet.event_id = event.id
            WHERE timesheet.deleted_at IS NULL
            LIMIT $1 OFFSET $2;
            "#,
            data.limit,
            data.offset
        )
        .fetch_all(executor)
        .await?;

        Ok(timesheets)
    }

    pub async fn read_all_per_employment(
        &self,
        user_id: Uuid,
        company_id: Uuid,
        data: TimesheetReadAllData,
    ) -> DbResult<Vec<TimesheetWithEvent>> {
        // For Redis

        self.read_all_per_employment_db(user_id, company_id, data)
            .await
    }

    async fn read_all_per_employment_db(
        &self,
        user_id: Uuid,
        company_id: Uuid,
        data: TimesheetReadAllData,
    ) -> DbResult<Vec<TimesheetWithEvent>> {
        let executor = self.pool.as_ref();

        let timesheets = sqlx::query_as!(
            TimesheetWithEvent,
            r#"
            SELECT timesheet.id, 
                   timesheet.start_date, 
                   timesheet.end_date, 
                   total_hours, 
                   is_editable, 
                   status AS "approval_status!: ApprovalStatus", 
                   manager_note AS "manager_note?", 
                   user_id, 
                   company_id,
                   event_id,
                   event.avatar_url AS event_avatar_url,
                   event.name AS event_name,
                   timesheet.created_at, 
                   timesheet.edited_at 
            FROM timesheet 
             JOIN event ON timesheet.event_id = event.id
            WHERE user_id = $1
              AND company_id = $2
              AND timesheet.deleted_at IS NULL
            LIMIT $3 OFFSET $4;
            "#,
            user_id,
            company_id,
            data.limit,
            data.offset
        )
        .fetch_all(executor)
        .await?;

        Ok(timesheets)
    }

    fn _is_data_empty(data: TimesheetUpdateData) -> bool {
        data.start_date.is_none()
            && data.end_date.is_none()
            && data.total_hours.is_none()
            && data.is_editable.is_none()
            && data.status.is_none()
            && data.manager_note.is_none()
            && data.workdays.is_none()
    }

    pub async fn update(
        &self,
        timesheet_id: Uuid,
        data: TimesheetUpdateData,
    ) -> DbResult<TimesheetWithWorkdays> {
        let mut tx = self.pool.begin().await?;

        if Self::_is_data_empty(data.clone()) {
            return Err(sqlx::Error::TypeNotFound {
                type_name: "User error.".to_string(),
            });
        }

        sqlx::query!(
            r#"
            UPDATE timesheet
            SET start_date = COALESCE($1, start_date),
                end_date = COALESCE($2, end_date),
                total_hours = COALESCE($3, total_hours),
                is_editable = COALESCE($4, is_editable),
                status = COALESCE($5, status),
                manager_note = COALESCE($6, manager_note),
                edited_at = NOW()
            WHERE id = $7
              AND deleted_at IS NULL
            "#,
            data.start_date,
            data.end_date,
            data.total_hours,
            data.is_editable,
            data.status as Option<ApprovalStatus>,
            data.manager_note,
            timesheet_id
        )
        .execute(tx.deref_mut())
        .await?;

        // This should likely be in a separate function.
        if data.workdays.is_some() {
            for workday in data.workdays.expect("Should be some!").into_iter() {
                sqlx::query!(
                    r#"
                    UPDATE workday
                    SET total_hours = COALESCE($1, total_hours),
                        comment = COALESCE($2, comment),
                        is_editable = COALESCE($3, is_editable),
                        edited_at = NOW()
                    WHERE timesheet_id = $4
                      AND date = $5
                      AND deleted_at IS NULL;"#,
                    workday.total_hours,
                    workday.comment,
                    workday.is_editable,
                    workday.timesheet_id,
                    workday.date
                )
                .execute(tx.deref_mut())
                .await?;
            }
        }

        /* You may think this is redundant, BUT
         *  since not all workdays may be edited,
         *  it's better to just retrieve all of them after the change.
         * Also we don't get event data from the update.
         * But if we get the time, we should think about improving this.
         */

        let workdays = sqlx::query_as!(
            Workday,
            r#"
            SELECT timesheet_id,
                    date, 
                    total_hours, 
                    comment AS "comment?", 
                    is_editable,
                    created_at,
                    edited_at 
            FROM workday 
            WHERE timesheet_id = $1
            ORDER BY date;
            "#,
            timesheet_id
        )
        .fetch_all(tx.deref_mut())
        .await?;

        let timesheet = self._read_one_tx(timesheet_id, tx).await?;

        let result = TimesheetWithWorkdays {
            timesheet,
            workdays,
        };

        Ok(result)
    }

    pub async fn _delete(&self, timesheet_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let timesheet = sqlx::query_as!(
            TimesheetStructureData,
            r#"UPDATE timesheet 
            SET edited_at = NOW(), 
            deleted_at = NOW() 
            WHERE id = $1
              AND deleted_at IS NULL
            RETURNING id,
                      start_date,
                      end_date;"#,
            timesheet_id
        )
        .fetch_optional(executor)
        .await?;

        if timesheet.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        /* We don't need to check if workdays are already
         * deleted because we do so for the timesheet.
         */
        sqlx::query!(
            "UPDATE workday
             SET edited_at = NOW(),
                 deleted_at = NOW()
             WHERE timesheet_id = $1;",
            timesheet_id
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn reset_timesheet(&self, timesheet_id: Uuid) -> DbResult<TimesheetWithWorkdays> {
        let mut tx = self.pool.begin().await?;

        let timesheet = sqlx::query_as!(
            TimesheetStructureData,
            r#"UPDATE timesheet 
            SET total_hours = 0,
                edited_at = NOW()
            WHERE id = $1
                  AND deleted_at IS NULL
            RETURNING id,
                      start_date,
                      end_date;"#,
            timesheet_id
        )
        .fetch_optional(tx.deref_mut())
        .await?;

        if timesheet.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        let workdays = sqlx::query_as!(
            Workday,
            r#"UPDATE workday
            SET total_hours = 0,
                comment = NULL,
                edited_at = NOW()
            WHERE timesheet_id = $1
              AND deleted_at IS NULL
            RETURNING timesheet_id,
                      date,
                      total_hours,
                      comment,
                      is_editable,
                      created_at,
                      edited_at;"#,
            timesheet_id
        )
        .fetch_all(tx.deref_mut())
        .await?;

        let timesheet = self._read_one_tx(timesheet_id, tx).await?;

        let edited_data = TimesheetWithWorkdays {
            timesheet,
            workdays,
        };

        Ok(edited_data)
    }

    pub async fn read_all_with_date_from_to_per_employment(
        &self,
        user_id: Uuid,
        company_id: Uuid,
        date_from: NaiveDate,
        date_to: NaiveDate,
        omit_workdays_outside_of_date_range: bool,
    ) -> DbResult<Vec<TimesheetWithWorkdays>> {
        // This is for redis.

        self.read_all_with_date_from_to_per_employment_db(
            user_id, company_id,
            date_from, date_to,
            omit_workdays_outside_of_date_range,
        ).await
    }

    pub async fn read_all_with_date_from_to_per_employment_db(
        &self,
        user_id: Uuid,
        company_id: Uuid,
        date_from: NaiveDate,
        date_to: NaiveDate,
        omit_workdays_outside_of_date_range: bool,
    ) -> DbResult<Vec<TimesheetWithWorkdays>> {
        let mut tx = self.pool.begin().await?;

        let timesheets_with_workdays
            = read_all_with_date_from_to_per_employment_db_using_tx(
                &mut tx,
                user_id,
                company_id,
                date_from,
                date_to,
                omit_workdays_outside_of_date_range)
            .await?;

        tx.commit().await?;

        Ok(timesheets_with_workdays)
    }

    ///
    /// Reads all timesheets (and their workdays) of a specific employee
    /// that intersect with requested date range and extend it with data
    /// needed for wage computation.
    ///
    pub async fn read_all_with_date_from_to_per_employment_extended_db(
        &self,
        user_id: Uuid,
        company_id: Uuid,
        date_from: NaiveDate,
        date_to: NaiveDate,
    ) -> DbResult<TimesheetsWithWorkdaysExtended> {
        let mut tx = self.pool.begin().await?;

        let timesheets_with_workdays: Vec<TimesheetWithWorkdays>
            = self.read_all_with_date_from_to_per_employment_db(
                user_id,
                company_id,
                date_from,
                date_to,
                true,
            ).await?;

        let employment_lite = employment_repo::read_one_lite_db_using_tx(
                &mut tx, user_id, company_id)
            .await?;

        let mut date_to_wage_presets = HashMap::<String, Option<WagePreset>>::new();

        //
        // Go through each timesheet and compute which wage presets it requires.
        //
        for timesheet in timesheets_with_workdays.iter() {
            let (date_from, date_to)
                = match timesheet.workdays.is_empty() {
                    true => (timesheet.timesheet.start_date,
                             timesheet.timesheet.end_date),
                    false => (timesheet.workdays[0].date,
                              timesheet.workdays.last().unwrap().date)
            };

            //
            // Start from a `date_from`, but reset the day of a month.
            //
            // Note: This cannot fail as 1st day always available.
            //
            let mut cur_date = date_from.with_day(1).unwrap();

            while cur_date <= date_to {
                let yyyy_mm = cur_date.format("%Y-%m").to_string();
                if date_to_wage_presets.contains_key(&yyyy_mm) {
                    continue;
                }
                //
                // todo later: Try to find a preset in `date_to_wage_presets`
                //             first as its faster than seeking it DB.
                //
                let preset_optional
                    = wage_preset_repo::read_optional_matching_date_db_using_tx(
                        &mut tx, &cur_date)
                    .await?;

                date_to_wage_presets.insert(yyyy_mm.clone(), preset_optional);

                if let Some(cur_date_incremented)
                    = cur_date.checked_add_months(Months::new(1)) {
                        cur_date = cur_date_incremented;
                } else {
                    //
                    // Note: This can return None only when `cur_date` > the
                    //        always-valid `end_date` in which case we need
                    //        to break the loop.
                    //
                    break;
                }
            }
        }

        tx.commit().await?;

        Ok(TimesheetsWithWorkdaysExtended {
            timesheets: timesheets_with_workdays,
            hourly_wage: employment_lite.hourly_wage,
            employment_type: employment_lite.employment_type,
            date_to_wage_presets,
        })
    }
}