# Echo

A server for the Echo secure messenger.

## Usage

This program is configured using environmental variables, which can either be passed in via the command line or using a `.env` file in the project directory.

The following variables can be set:

- `IP_ADDRESS` specifies the IP address to host on
- `PORT_NUMBER` specifies the port number to host on
- `DATABASE_URL` specifies the URL of the Postgres database to connect to (e.g. `postgres://USER:PASS@localhost:5432/DATABASE`)
- `MAX_DB_CONNECTIONS` specifies the number of concurrent connections the database can use
- `CREATE_DATABASE` can be set to 1 to set up tables for a new database
- `DROP_DATABASE` can be set to 1 to drop all tables in a database
