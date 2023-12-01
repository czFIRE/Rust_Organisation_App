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
- EmployeeLevel { 'Basic', 'Manager' } -> Denotes whether the employee has managerial responsibilities / privileges within a company, or not.
- StaffLevel { 'Basic', 'Organizer' } -> Denotes whether the employee has additional (organizer) privileges within a given event.
- UserStatus { 'OK', 'Sick', 'Vacation' } -> Denotes the availability of a user in terms of employment and task opportunities.