# database

## psql

The database can be controled using a CLI utility *psql*:
```
    # export variables from the config file into the shell.
    source ../.env

    # connect into database (a password prompt follows)
    psql -U ${POSTGRES_USER} -h localhost -p 5432 -d pv281

    # execute the SQL script
    \i data/data.sql
```
