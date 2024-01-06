# Database

The database can be controlled using a CLI utility _psql_:

```bash
# export variables from the config file into the shell
source ../.env

# connect to the database (a password prompt follows)
psql -U ${POSTGRES_USER} -h localhost -p 5432 -d pv281

# execute the SQL script
\i data/data.sql
```
