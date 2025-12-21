# taskflow
A real-time task and workflow backend built with Rust.

## Database Migrations

This project uses SQLx for database migrations.

Migrations are applied **manually** and are not run automatically at application startup.

### Running migrations locally

```bash
sqlx migrate run
