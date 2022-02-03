# Rocket, Diesel, PostgreSQL and Docker Compose

A web server built with Rocket and Diesel using PostgreSQL. It provides CRUD operations on TODOs.

Note that Rocket v0.5 is used. There some differences between Rocket v0.4 and Rocket v0.5 such as `crate rocket_contrib` is integrated into `crate rocket`. This repo fully uses Rocket v0.5 features.

## How to Run

```shell
docker-compose run
```

## How to Test

### 0. TODO has following fields.

Field | Type
--- | ---
id | Integer
creator_name | String(100)
title | String(256)
description | String(1000)
created_at | Timestamp with Timezone

### 1. Get all TODOs.

```shell
GET /todos
```

### 2. Get all TODOs ordered by `created_at` in ascending or descending.

```shell
GET /todos?order={asc,desc}
```

### 3. Get TODO by id.

```shell
GET /todos/{id}
```

### 4. Create new TODO.

```shell
POST /todos/
```

Key | Value
--- | ---
creator_name | String(100)
title | String(256)
description | String(1000)

### 5. Update TODO by id.

```shell
PUT /todos/{id}
```

Key | Value
--- | ---
creator_name | String(100)
title | String(256)
description | String(1000)

### 6. Delete TODO by id.

```shell
DELETE /todos/{id}
```

## License

This project is licensed under the MIT License.
See [LICENSE](LICENSE) for details.

