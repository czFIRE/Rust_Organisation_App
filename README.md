# Organization app

### Contents
- [Official Assignment](#official-assignment)
- [Project Deployment](#project-deployment)
- [Authors](#authors)
- [Work Distribution](#work-distribution)

## Official Assignment

A web application with several features that would facilitate the organization
of music events and concerts. It would be an application with login (several
different levels of rights), adding, removing, and editing employees,
individual concerts, associated companies, staff, and temporary positions...

The main function would be for part-time workers. It would be a logging of hours
worked (the part-time worker would choose a given month, a given event, and a
given day the event took place and log how much he worked in that position) then
the application would calculate the price he should get (including taxes).

The application would include a frontend (the website itself) and a backend that
will take care of all the logic and database, possibly the actual deployment to
the domain and server. Optional extensions were the ability to add, remove, and
the ability to view event schedules where instructions would be listed for
employees.

## Project Deployment
**Contact Person** - Matej Vavrek (Discord: .swiftfeather)

For the sake of convenience, we have prepared a default setup in .env.docker. 
This example should, obviously, never used in production as it uses credentials that are default and well known, but for the purposes of a demo, you may use them to start up the application.

To start the project up, you must have docker on your machine.
The command to start the project is:
```sh
docker compose up --build
```

The docker compilation can be quite lengthy, so it may be wise to enjoy a 5 minute side activity while you are waiting for the web container to start up.
The web container may complain that if failed to connect to the database, but eventually, it should connect. It may take 3 - 4 tries.

### If Docker Fails
Should the web container somehow fail, feel free to turn it off and ignore it. The application should still run if you keep postgres and keycloak running and do ``cargo run`` instead.
Of course, in such a case, you need to manually apply migrations using ``cargo sqlx migrate run``. This should also apply the seeding script so you have data to interact with.


### Available Users
We have prepared several user accounts you can use to explore Orchestrate.

**Dave Null** - Our most privileged user. Dave should be able to access the Administration Panel, manage Employments for AMD, manage AMD itself and also manage the Woodstock event.

Login: dave@null.com 

Password: davenull

**Anna Smeth** - A regular user. She should have an editable timesheet in her employment. You should be able to confirm Anna's timesheet as Dave Null, since he is her superior in the company hierarchy.

Login: a.smeth@seznam.cz
Password: annasmeth

**James Bean** - Regular user with a regular employment. Can be registered to Woodstock and confirmed/denied by Dave Null.

Login: jamesbean176@snailmail.com
Password: beans

## Authors 
* Bc. Petr Kadlec - czFire
* Slavomír Vlček - s_vlc
* Bc. Matej Vavrek - Swiftfeather
* Michal Šoltis - jumpman23

## Work Distribution
### Database Design - ERD
The database design was one of the first steps we took towards achieving our
results.

The database was modelled as an ERD diagram in plant UML, and can be found in
/design. This should be an up-to-date version.

Most of it was performed by **Slavomír** and **Matej**. Matej provided the base for the
ERD, and Slavomír did a lot of work on further elaboration, ensuring the
database model was consistent and logical in its design.

### Event Storming
We also tried to implement event storming, which we did not finish. Still, the work in progress workflows were useful in developing the frontend, database and api designs that were done simultaneously.
You can find the event storming figma whiteboard we used [here](https://www.figma.com/file/wkLYbNLmuZt3n8VkvsprZH/Event-organization-app?type=whiteboard&node-id=0-1).

The main organizer of Event Storming was **Petr**.

### Frontend Design 
The first designs of the frontend side of the project were
done in figma. These were done by **Matej**. During frontend development, some of
the designs have changed significantly, so currently, this design is somewhat outdated.

You can find the early designs
[here](https://www.figma.com/file/TdkpVqSw8VvE8rMivkN2xl/Orchestrate---Rust-App?type=design&node-id=74%3A1472&mode=design&t=zWLYTI86JcVkAUde-1).

### API Design
The first draft of the API design was done by Matej in Swagger. You can find it
in /design.  This is a highly outdated version, as towards the end of development,
many endpoints changed or were removed and there was no time to update the swagger documentation.

### Containerization
Most of the containerization work was performed by Michal and Petr. Michal
provided the orchestration for the website, pgadmin and postgres containers, and
Petr added onto the orchestration with Keycloak and its accompanying database.

### Database SQL
Work on the Database migration SQL was done by Michal, Matej and Slavomir, but mostly by
Slavomir who ensured the SQL was consistent with the ERD after Michal's initial implementation.
Slavomir also added important constraints and wrote most of the database seeding script.

### Testing
To ensure at least a decent level of correctness for the operations in our system, we wrote a large number of tests. These tests do not cover all of the functionality, but we tried our best to cover as much as we could.


All in all, we have written around 133 tests. These test are separated into three categories.

Repository Tests - 58 tests testing the functionality of database access repositories.

API Tests - 74 tests testing the functionality of the REST API.

Wage Calculation Test - 1 test testing the functionality of the wage calculation functionality.

### Database Repository Pattern
For database access, we had chosen SQLX as everyone worked with it due to the
iteration. Most of the work on the repository pattern was done by Petr, who
implemented access for all tables except for Timesheet and Wage Preset.

Petr also implemented most of the tests for the repositories.

The timesheet repository was first developed by Michal, but he later decided that
he did not want to do this task and Matej picked it up after him. Matej also defined 
the tests for the timesheet repository. Later, Slavomir added some additional operations, as well.

The Wage Preset repository was developed Slavomir, who also defined tests for this repository.

### Wage Calculation
Wage calculation was developed by Slavomir, who created everything necessary for
this component, including the Wage Preset table, the Wage Preset repository and
tests for this functionality.

### File Upload
As part of our website, we have decided to allow users to upload simple .jpg
images for their profiles, their companies and their events.  This component was
developed by Matej.

The component itself is rather simple and performs basic checks for file size
and file format. In a proper production environment, these files would likely be
stored elsewhere, such as a separate storage server.

### REST API
The majority of the REST API was developed by Matej. We used actix web to handle
requests and actix_web_files to handle serving of static files for the frontend.

### Frontend
Frontend was developed by Matej. The implementation was done in HTMX and we used
Tailwind CSS for styling.  The frontend is likely the second weakest part of the
application, as not everything is implemented as nicely as we would like. This
was due to time constraints because of the failure to provide Auth on time.

### Auth
The final (and, sadly, least) developed component is Auth. It was first assigned to Michal, but he
was did not implement this feature and stopped working on it towards the very end of the project, 
leaving others to handle it. 
It was then picked up by Petr and Matej, who tried to implement this feature for the project deadline.

For the implementation, we chose to go with Keycloak. We used the actix-web-keycloak-middleware
crate to help with the checking of bearer tokens.
