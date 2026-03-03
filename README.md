# dbnest

Cozy local databases in seconds.

`dbnest` is a cross-platform CLI for provisioning local databases and initializing schema from a simple JSON (file or folder layout).

## Current features (v0.1.0)

- SQLite embedded provisioning (no external dependencies)
- PostgreSQL server docker instance (docker required)
- Instance registry (list / remove)
- Schema workflow:
  - `plan` generates SQL from schema
  - `apply` executes the schema against the database
- Schema input formats:
  - Single JSON file (`schema.json`)
  - Directory layout (`schema/<table>/columns.json`, optional `indexes.json`)

Planned next:

- MySQL via Docker

---

## Install

### From crates.io (recommended)

````bash
cargo install dbnest
````

### From source
````bash
cargo install --path crates/dbnest-cli
````

---

## Quickstart

1. Create a database

SQLite:

```bash
dbnest up sqlite --path ./dev.sqlite
```

PostgreSQL:

```bash
dbnest up postgres --user dev --password dev --db appdb
```

List instances

```bash
dbnest ls
```

2. Define schema
   Create a schema file or directory layout

###### schema.json:

```bash
{
  "tables": [
    {
      "name": "users",
      "columns": [
        { "name": "id", "type": "uuid", "primary_key": true },
        { "name": "email", "type": "string", "unique": true, "nullable": false },
        { "name": "created_at", "type": "timestamp", "default": "now" }
      ],
      "indexes": [
        { "name": "idx_users_email", "columns": ["email"], "unique": true }
      ]
    }
  ]
}
```

Or a directory layout:

```bash
schema/
  users/
    columns.json
    indexes.json
```

with `columns.json` and `indexes.json` containing the respective table schema.

```bash
[
  { "name": "id", "type": "uuid", "primary_key": true },
  {
    "name": "email",
    "type": "string",
    "unique": true,
    "nullable": false
  },
  { "name": "created_at", "type": "timestamp", "default": "now" }
]
```

3. Generate SQL from schema (plan)

SQLite:

```bash
dbnest plan sqlite --schema ./schema.json               # from single json schema
dbnest plan sqlite --schema ./schema/                   # from directory based schema
```

Postgres:

```bash
dbnest plan postgres --schema ./examples/schema.json    # from single json schema
dbnest plan postgres --schema ./schema/                 # from directory based schema
```

4. Apply schema to database (apply)

```bash
dbnest apply --id <INSTANCE_ID> --schema ./schema.json
# or
dbnest apply --id <INSTANCE_ID> --schema ./schema/
```

---

## License

MIT
