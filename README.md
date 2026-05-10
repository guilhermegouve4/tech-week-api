# week-tech-api

REST API for the Week Tech event — built with Rust, Axum and SQLite.

## Stack

Rust · Axum · SQLx · SQLite · Railway

## Endpoints

| Method | Route              | Auth |
|--------|--------------------|------|
| POST   | /registrations     | —    |
| POST   | /projects          | —    |
| POST   | /checkin           | —    |
| POST   | /admin/login       | —    |
| GET    | /registrations     | JWT  |
| GET    | /projects          | JWT  |

## Schema

**registrations** — name, student_registration, course_name, course_period, coffee_break, checked_in

**projects** — id, submitter_name, submitter_registration, project_name, description

## Como rodar localmente

```bash
cargo run
```

## Decisões técnicas

- **Axum + Tokio** — framework web assíncrono; async porque a API lida com múltiplas requisições de I/O concorrentes
- **SQLx sem ORM** — SQL escrito na mão para controle total das queries
- **JWT** — autenticação stateless para rotas admin; token retornado no login, enviado via `Authorization: Bearer`
- **Argon2** — hash de senha do admin; escolhido sobre SHA-512 por ser intencionalmente lento (resistente a brute force)
- **SQLite** — suficiente para um evento pontual; sem necessidade de infraestrutura de banco externa
- **404 no checkin** — RA não encontrado retorna 404; `None` (Rust) mapeado para HTTP antes de responder ao frontend

## Deploy

Railway — variáveis de ambiente configuradas no painel do projeto.
