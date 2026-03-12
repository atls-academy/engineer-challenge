## Что это за проект

Модуль аутентификации для продукта Orbitto в рамках Advanced Engineer Challenge:

- **Регистрация пользователя**
- **Авторизация и выдача JWT-токенов (access + refresh)**
- **Восстановление пароля по e‑mail (reset‑token)**

Сервис работает по **gRPC** и использует **PostgreSQL** в Docker Compose.

---

## Стек и почему так

- **Язык: Go**
  - Простая сборка в один бинарник, удобно для gRPC и микросервисов.
  - Хорошая экосистема для сетевых сервисов, миграций, PostgreSQL, JWT.
- **gRPC (+ protobuf)** – строгая схема контракта, типобезопасность, удобная генерация кода.
- **PostgreSQL** – стабильная, привычная реляционная БД, хорошо подходит для хранения пользователей.
- **golang-migrate** – простой и надёжный инструмент для миграций через код/CLI.
- **Docker Compose** – минимальный, но воспроизводимый стенд для локального запуска (IaC).

Альтернативы, которые рассматривал:

- **REST вместо gRPC** – проще дебажить через браузер, но в задании явно просят gRPC/GraphQL, поэтому выбрал gRPC.

---

## Как запустить

### Вариант 1: Docker Compose (рекомендуемый)

Требования:

- Docker
- Docker Compose

Команды из корня проекта:

```bash
docker compose build
docker compose up
```

Что при этом поднимается:

- `db` – контейнер с PostgreSQL `15-alpine`
- `auth-service` – gRPC-сервис аутентификации на порту `5555`

При старте сервис:

1. Подключается к PostgreSQL.
2. Прогоняет миграции из `./migrations`.
3. Запускает gRPC‑сервер на `:5555`.

Ожидаемые строки:

- `Running database migrations...`
- `Migrations applied successfully or already up to date`
- `gRPC Server listening on :5555`

### Вариант 2: Локальный запуск без Docker

Требования:

- Go `1.22`+
- Локальный PostgreSQL

1. Настроить БД (по умолчанию):

   - host: `localhost`
   - port: `5432`
   - user: `postgres`
   - password: `postgres`
   - database: `postgres`

2. Создать `.env` (есть пример `.env.example`) или экспортировать переменные в shell:

3. Запустить сервис:

```bash
go run ./cmd/auth-service/main.go
```

---

## gRPC API

Прото‑контракт: `api/proto/identity.proto`

Сервис:

- `identity.IdentityService`

Методы:

- `Register(RegisterRequest) returns (RegisterResponse)`
- `Login(LoginRequest) returns (LoginResponse)`
- `ForgotPassword(ForgotPasswordRequest) returns (ForgotPasswordResponse)`
- `ResetPassword(ResetPasswordRequest) returns (ResetPasswordResponse)`

### Примеры запросов (Postman / grpcurl)

**Register**

```json
{
  "email": "test@example.com",
  "password": "StrongP@ssword123!",
  "display_name": "Test User"
}
```

**Login**

```json
{
  "email": "test@example.com",
  "password": "StrongP@ssword123!"
}
```

**ForgotPassword**

```json
{
  "email": "test@example.com"
}
```

Токен сброса попадает в лог сервиса в виде ссылки:

`[EMAIL STUB] To: test@example.com | Reset Link: http://localhost:5555/reset?token=...`

**ResetPassword**

```json
{
  "token": "СКОПИРОВАННЫЙ_ТОКЕН_ТУТА",
  "new_password": "NewStrongP@ss123!"
}
```

---

## Тесты

Запуск тестов:

```bash
go test ./...
```

Что покрыто:

- `internal/domain/user_test.go`
  - Проверка сложности пароля (`ValidatePassword`).
  - Проверка статуса пользователя (`IsPro`).
- `internal/app/auth_service_test.go`
  - Фейковый in‑memory `UserRepository` для unit‑тестов.
  - Проверка, что `Login` на несуществующий e‑mail возвращает `ErrInvalidCredentials`.
  - Проверка, что `Register` не даёт зарегистрировать пользователя с существующим e‑mail.

---
