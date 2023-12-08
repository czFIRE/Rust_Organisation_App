ERD.puml contains the entity relationship diagram of the database design for the Organisation App.

## Considerations
- Recording of time worked may require more detail or better security. If we keep it as vague as it is currently, we might be able to store it in this state, but anything more could potentially prove to be sensitive private information. (Do we want to handle that in a school assignment?)

- The different types of users (event organizer, temporary, full-time) are recorded in the user_level attribute of User.

- Do we also want an administrator to be a user, or a separate entity with different responsibilites? (E.G. An Administrator can't do the same things either of the normal users can). If and administrator is to be a user, then the cardinality of User - Employment must be changed from 1:1 to 1:{0,1}.

## Enums
Currently, the enums in the model are as follows:
- UserLevel { 'User', 'Admin' } -> Denotes the privileges of a user in terms of interactions with the system as a whole.
- TaskPriority { 'Low', 'Medium', 'High' } -> Denotes the priority of a task assigned to event staff.
- AssociationType { 'Sponsor', 'Organizer', 'Other' } -> Denotes the type of association that a company has with an event.
- EmploymentType { 'DPP', 'DPC', 'HPP' } -> Denotes the type of employment an employee may have with a company.
- EmployeeLevel { 'Basic', 'Manager', 'Upper Manager' } -> Denotes whether the employee has managerial responsibilities / privileges within a company, or not. The Upper Manager role represents the overall responsible person (people) that take care of the company in the system.
- StaffLevel { 'Basic', 'Organizer' } -> Denotes whether the employee has additional (organizer) privileges within a given event.
- UserStatus { 'OK', 'Sick', 'Vacation' } -> Denotes the availability of a user in terms of employment and task opportunities.
- AssignmentStatus { 'pending', 'accepted', 'rejected' } -> Denotes the status of a staff request to work on a task.
- AcceptanceStatus { 'pending', 'accepted', 'rejected' } -> Denotes the status of a staff request to work on an event.
  
## Notes (4.12.2023)
*Just a consideration*
Anyone can register and then a higher-privilege user (company manager or admin) can register them to a company.

**DONE**
Event Time Slot: Remove

**DONE**
User: Add birth_date
      Add gender (enum)
      

**Event** Registration - **DONE**:
    - User-based registration - workers (full or part-time) may register to an event, or management may assign them to an event. Once registered, a worker can't unregister from an event.
    - Organizers must confirm/reject event registrations.
    - **Events have two states** - 'accepts workers' / 'doesn't accept workers' which is set by event organizer

**Task** Registration - **DONE**:
    - Staff-based registration - Staff may volunteer for a task, or organizers may assign them to a task. Works like event registration. 
    - Organizers must confirm/reject task volunteers.
    - **Tasks have two states** - 'accepts staff' / 'doesn't accept staff'
    
Assigned Staff - **DONE**:
    - Add decided_by column which contains the ID of the organizer that made the final decision on the staff request to work on a task.
    - Add assignment_status column which contains an enum of three values (Pending, Accepted, Rejected) determining the status of the assignment.

Timesheet - **DONE**:
    - Timesheet is **attached to an event**.
    - start_date, end_date is reduntant with event_timespan_range time_from, time_to, but this is probably okay for now.

File - **DONE**:
    - Reference to a thumbnail (smaller version) and its size (width x height).

**DONE**
Move avatar id below line


Company - **DONE**:
    - ICO, DICO, Address, Contact Telephone Number, Contact Email


Employment - **DONE**:
    - Employee Level <BIGBOSS>

TimeSheet - **DONE**: 
    - Responsible Person ID / Approved By / Approved Column