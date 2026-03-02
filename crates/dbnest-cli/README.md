## Quickstart
1) Create a SQLite database
```bash
dbnest up sqlite --path ./dev.sqlite
```

List instances
```bash
dbnest ls
```

2) Define schema
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

3. Generate SQL from schema (plan)

```bash
dbnest plan sqlite --schema ./schema.json
```
or
```bash
dbnest plan sqlite --schema ./schema/
```

4. Apply schema to database (apply)

```bash
dbnest apply --id <INSTANCE_ID> --schema ./schema.json
```
or
```bash
dbnest apply --id <INSTANCE_ID> --schema ./schema/
```

---

## License
MIT