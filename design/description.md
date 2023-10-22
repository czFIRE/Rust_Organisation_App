# Description relevant to the design of the application
The application must support organization of concerts and music events. These are the basic block of the app.

Part-time worker timesheet calculations must be included - as such, a part-time worker must be able to enter their timesheet (an accounting of hours worked in a given month) into the application, and the application must be able to calculate the pay the worker should receive.

Each concert has associated companies, those companies have employees that have responsibilities in a concert. Employees can be full-time and part-time. (?)

## Problem Domain Model (PDM) notes
- Currently, 'staff' is an umbrella term for any worker related to the event.

- The term 'employee' is currently interpreted as 'full time worker'.

- The term 'temporary position' is currently interpreted as 'part-time worker'.

- The difference between Staff and EventOrganizer is the level of rights assigned to each user type.

- The term 'company' refers to associated companies.

## Considerations
- Users have different rights levels. Regular staff may have less rights than event organizers.

- Do event organizers represent companies? If so, they should have access to PartTimeWorker timesheets through events.

- What relevance should associated companies (Company in PDM) have?

- Is there any difference between 'Temporary Positions' and 'Part-Time Workers'? If so, what?

- What is the difference between 'employee' and 'staff'? The project description mentions both separately in one sentence.