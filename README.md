# tech-week-api

REST API for the Week Tech event at Unicesumar — built with Rust, Axum and SQLite, with a focus on secure-by-default design.

## Stack

- **Rust** — memory safety and ownership guarantees without a garbage collector
- **Axum + Tokio** — async web framework; handles concurrent I/O without blocking threads
- **SQLx** — compile-time verified SQL queries, no ORM
- **SQLite** — sufficient for a single-event workload
- **Argon2id** — memory-hard password hashing; resistant to GPU brute-force attacks
- **JWT (HS256)** — stateless authentication; token verified on every protected request via a custom Axum extractor
- **tower-http** — CORS middleware

## Endpoints

| Method | Route | Auth |
|--------|-------|------|
| POST | /registrations | — |
| POST | /projects | — |
| POST | /checkin | — |
| POST | /admin/login | — |
| GET | /registrations | JWT |
| GET | /projects | JWT |
| DELETE | /registrations/{ra} | JWT |
| DELETE | /checkin/{ra} | JWT |

## Running locally

```bash
cargo run
```
