# API Contract — Week Tech

## POST /registrations

```json
{
  "name": "string", // 255 characters max
  "student_registration": "253579732", // string, exactly 9 digits
  "course_name": "string", // 255 characters max
  "course_period": 1, // min 1, max 12
  "coffee_break": false/true
}
```

- `201`
- `400` `{ "error": "missing_fields" }`
- `400` `{ "error": "invalid_student_registration" }`
- `400` `{ "error": "ra_already_registered" }`

---

## POST /projects

```json
{
  "submitter_name": "string", // 255 characters max
  "submitter_registration": "253579732", // string, exactly 9 digits
  "project_name": "string", // 255 characters max
  "description": "string" // 500 characters max
}
```

- `200`
- `400` `{ "error": "missing_fields" }`
- `400` `{ "error": "invalid_ra" }`
- `400` `{ "error": "ra_not_found" }`

---

## POST /checkin

```json
{
  "student_registration": "253579732" // string, exactly 9 digits
}
```

- `200`
- `400` `{ "error": "invalid_student_registration" }`
- `404` `{ "error": "ra_not_found" }`

---

## POST /admin/login

```json
{
  "email": "string", // email type
  "password": "string" // backend validation
}
```

- `200` `{ "token": "..."}`
- `400` `{ "error": "missing_fields" }`
- `401` `{ "error": "invalid_credentials" }`

---

## GET /registrations — protected (JWT)

```json
[
  {
    "name": "string", // 255 characters max
    "student_registration": "253579732", // string, exactly 9 digits
    "course_name": "string", // 255 characters max
    "course_period": 1, // min 1, max 12
    "coffee_break": false,
    "checked_in": false
  }
]
```

---

## GET /projects — protected (JWT)

```json
[
  {
    "id": 1,
    "submitter_name": "string", // 255 characters max
    "submitter_registration": "253579732", // string, exactly 9 digits
    "project_name": "string", // 255 characters max
    "description": "string" // 500 characters max
  }
]
```
