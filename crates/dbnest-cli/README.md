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
