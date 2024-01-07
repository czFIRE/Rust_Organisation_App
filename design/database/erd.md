ERD.puml contains the entity relationship diagram of the database design for the Organisation App.

## Tables

### User
This table contains basic information about users within the system.

#### Attributes
- user_id - Primary key
- name - User's full name
- user_level - The standing of a user within the system's authorizations. See the UserLevel enum.
- user_status - The availability of a user in regards to tasks.
- email - User's email address
- date_of_birth - Speaks for itself.
- avatar_url - path / url to the user's avatar picture.
- sex - speaks for itself

### Company
This table contains basic information about companies registered within the system.

#### Attributes
- company_id - Primary Key
- name - Name of the company
- description
- website
- CRN / VATIN - company registration and tax information
- contact_phone - phone number
- contact_email - email
- avatar_url - path / url to the company's avatar picture.

### Company_address
Stores a compound address for a company.

#### Attributes
- company_id - Primary Key, FK to company
- country - name of country, e.g. Czech Republic
- region - name of region, e.g. Jihomoravská Kraj
- city - name of city, e.g. Brno
- street - name of street, e.g. Botanická
- address_number - the orientation number of an address, e.g. 554/68a
- postal_code - the postal code, e.g. 60200

### Event
This table contains information about events registered within the system.

#### Attributes
- event_id - Primary Key
- name - Name of the event
- description
- website
- accepts_staff - this boolean value denotes whether users (employees of associated companies) may register for possible work on the event.
- work_start - denotes the start of work on the event
- work_end - denotes the end of work on the event
- avatar_url - path / url to the event's avatar

### Timesheet
This table contains information about timesheets that users may access within the system.
In our system, timesheets are related to events, not whole months.

#### Attributes
- timesheet_id - Primary Key
- user_id - Foreign Key, keeps track of who the timesheet belongs to
- company_id - Foreign Key, keeps track of what company the timesheet is asociated with.
- (user_id, company_id) - Identify a particular employment relationship
- event_id - The event this timesheet is bound to.
- start_date - The start date of the timesheet. Corresponds to the start date of the work on an event.
- end_date - The end date of the timesheets. Has same correspondence as start_date.
- worked_hours - A float value representing the aggregated sum of worked_hours for all workdays (workday table)
- is_editable - This attribute represents two things. A timesheet is editable when a user can enter data into it. It can become non-editable when the timesheet is turned in. So we model both the editability and the status of the timesheet. The sheet becomes editable if any issue is detected by the responsible party and a correction is needed.
- manager_note - a note from a responsible person
  
### Workday
This table contains information about workdays that belong to timesheets.

#### Attributes
- (timesheet_id, work_date) - Primary Key, timesheet_id is a FK to timesheet
- worked_hours - worked hours for a given day
- commentary - user-added commentary just for some clarification for a manager that might be checking the timesheet
- is_editable - ensuring that editability of a timesheet propagates to days

### Employment
This table contains information about employments of users for companies. Hence, it also marks relationships between users and 
companies in the systems.

#### Attributes
- (user_id, company_id) - Primary Key, user_id is FK to user, company_id is FK to company
- manager_id - Hierarchical Foreign Key, points to a user above ours in the hierarchy. Upper management / company administrators has this set to null
- employment_type - the contract type, see the EmploymentType enum.
- hourly_rate - the hourly rate
- employee_level - employee level, see the EmployeeLevel enum.
- description - the work description of the employment
- start_date - start date of the contracted employment
- end_date - end_date of the contracted employment, if the end_date is not known, 9999-12-31 should be used.

### Associated_company
This table holds information about the relationships between events and companies.

#### Attributes
- (event_id, company_id) - Primary Key, event_id is FK to event, company_id is FK to company
- association_type - denotes the type of relationship a company has to an event, see the AssociationType enum.
  
### Event_staff
This table contains information about staff for events. It connects an employee from an organization to a relevant event and tells us about their position within the event.

#### Attributes
- staff_id - Primary Key
- (user_id, company_id) - FK to employment, user_id is FK to user, company_id is FK to company
- event_id - FK to event
- staff_level - denotes the competency of a given staff member within an event. See the EventRole enum for more.
- acceptance_status - this column is used to track whether given staff has been accepted or rejected for an event. Or whether no decisions have been made yet. Used for registration of users for events they want to work on.
- decided_by - FK to event_staff. The default is null, this attribute is populated when acceptance_status changes to accepted or rejected and contains the ID of the organizer that made the decision.

### Assigned_staff
Represents a relationship between event staff and tasks.

#### Attributes
- (task_id, staff_id) - Primary key, task_id is FK to task, staff_id is FK to staff
- assignment_status - this column behaves differently to event_staff acceptance_status, but considers acceptance for a single task instead. Within an event, staff are able to register for tasks that interest them, and organizers once again reject or accept staff registrations for tasks.
- decided_by - the exact same as in event_staff.

### Task
Contains information about tasks that are to be carried out during the work on event organization.

#### Attributes
- task_id - Primary Key
- event_id - FK to event, identifies the event this task belongs to
- creator_id - FK to event_staff, identifies the event organizer this task was created by.
- title - the title of the task
- descritpion - the description of the task
- date_accomplished - starts off as null, used to track when a task is finished.
- priority - priority of the task, TaskPriority enum
- accepts_staff - this has similar semantics as the event attribute of accepts_staff. Toggles the ability of event staff to register for a task.

### Comment
This table contains comments from users. These commenst are either for events (where only those involved in an event can see them) or tasks (similar).

#### Attributes
- comment_id - Primary Key
- event_id / task_id - FK to the relevant table (only one, never both, never neither)
- author_id - FK to user, the author of the comment.
- content - the content of the comment


## Enums
We use several enum types, their description can be found in the puml file.