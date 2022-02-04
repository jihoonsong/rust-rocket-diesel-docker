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

For example, if you use curl command:

```shell
curl -X GET http://0.0.0.0:8000/todos
```

### 2. Get all TODOs ordered by `created_at` in ascending or descending.

```shell
GET /todos?order={asc,desc}
```

For example, if you use curl command:

```shell
curl -X GET http://0.0.0.0:8000/todos?order=asc
```

### 3. Get TODO by id.

```shell
GET /todos/{id}
```

For example, if you use curl command:

```shell
curl -X GET http://0.0.0.0:8000/todos/1
```

### 4. Create new TODO.

```shell
POST /todos/
```

You need to send JSON data like below.

Key | Value
--- | ---
creator_name | String(100)
title | String(256)
description | String(1000)

For example, if you use curl command:

```shell
curl -X POST \
     -H "Content-Type: application/json" \
     -d '{"creator_name":"JihoonSong", "title": "Meeting at 3PM", "description": "Weekly scrum"}' \
     http://0.0.0.0:8000/todos
```

### 5. Update TODO by id.

```shell
PUT /todos/{id}
```

You need to send JSON data like below.


Key | Value
--- | ---
creator_name | String(100)
title | String(256)
description | String(1000)

For example, if you use curl command:

```shell
curl -X PUT \
     -H "Content-Type: application/json" \
     -d '{"creator_name":"JihoonSong", "title": "Lunch at 1PM",  "description": "With J"}' \
     http://0.0.0.0:8000/todos/1
```

### 6. Delete TODO by id.

```shell
DELETE /todos/{id}
```

For example, if you use curl command:

```shell
curl -X DELETE http://0.0.0.0:8000/todos/1
```

## License

This project is licensed under the MIT License.
See [LICENSE](LICENSE) for details.

