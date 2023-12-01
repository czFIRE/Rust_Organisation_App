ERD.puml contains the entity relationship diagram of the database design for the Organisation App.

## Considerations
- Recording of time worked may require more detail or better security. If we keep it as vague as it is currently, we might be able to store it in this state, but anything more could potentially prove to be sensitive private information. (Do we want to handle that in a school assignment?)

- The different types of users (event organizer, temporary, full-time) are recorded in the user_level attribute of User.

- Do we also want an administrator to be a user, or a separate entity with different responsibilites? (E.G. An Administrator can't do the same things either of the normal users can). If and administrator is to be a user, then the cardinality of User - Employment must be changed from 1:1 to 1:{0,1}.

## Enums
Currently, the enums in the model are as follows:
- UserLevel { 'User', 'Admin' }
- TaskPriority { 'Low', 'Medium', 'High' }
- AssociationType { 'Sponsor', 'Organizer', 'Other' }
- EmploymentType { 'DPP', 'DPC', 'HPP' }
- EmployeeLevel { 'Basic', 'Manager' }
- StaffLevel { 'Basic', 'Organizer' }
- UserStatus { 'OK', 'Sick', 'Vacation' }