# Auth Service
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=vwency_engineer-challenge&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=vwency_engineer-challenge) [![Bugs](https://sonarcloud.io/api/project_badges/measure?project=vwency_engineer-challenge&metric=bugs)](https://sonarcloud.io/summary/new_code?id=vwency_engineer-challenge) [![Code Smells](https://sonarcloud.io/api/project_badges/measure?project=vwency_engineer-challenge&metric=code_smells)](https://sonarcloud.io/summary/new_code?id=vwency_engineer-challenge) ![License](https://img.shields.io/github/license/vwency/engineer-challenge)

## Description 
Проект реализует функции восстановление пароля, регистрация, авторизации, максимально приближенные к prod-ready решениям. С кэшированием в valkey(open source форк redis)
 
## Architecture

**DDD**  
- Фокус на доменной логике  (entities, port/in ports/out)
- Улучшенная поддерживаемость  
- Чёткое разделение бизнес-слоёв

**DI**  
- Слабая связанность компонентов  
- Упрощённое тестирование  
- Гибкость замены реализаций

**CQRS**  
- Разделение операций чтения и записи  
- Оптимизация I/O  
- Улучшенная масштабируемость  

**ADR References:**  
- [Cookie-based Session Authentication](./docs/adr/0001-cookie-session.md)  
- [Valkey Cache for Session Profiles](./docs/adr/0003-valkey-cache.md)  

## Tech stack
1. **REST**, поскольку поддерживает в запросе `Set-Cookies`, статус коды http.
2. **Yarn berry** большое сообщество, кастомизация.  
3. **NX** время сборки, уменьшение времени на CI.  
4. **Rust** строгая типипизация, гарантия доставки, гибкость в архитектуре.
5. **Valkey**  Поддержка — Valkey поддерживается крупными компаниями: AWS, Google, Oracle, Ericsson. Redis Ltd. — единственный вендор Redis OSS.


## Trade-offs
1. Дублирование стилей/tsx. (скорость прототипирования), рефакторинг перед подготовкой к prod-ready.
2. Redux. (скорость прототипирования + архитектура, возможен пересмотр при разработке.  
3. Webpack (HMR, hot-reload)  как альтернатива рассматривался turbopack(нету HMR)  
4. Нет подтверждения пароля по почте при регистрации. (время отладки), рефакторинг во время разработки, сразу.  
5. Нет полноценного IaC(время), при enterprise подготовки к prod.  
6. Не использовал jwt поскольку сервис 1, нет экосистемы сервисов; сессия шарится cross-domain если cookie-based и делаем запрос с другого домена с `credentials: include`. При подготовки в prod, или масштабировании.
7. Сервис уже является bounded context
Auth-сервис сам по себе — это один BC в рамках большей системы. Дробить BC внутри сервиса — это over-engineering, если нет реальных причин.

### Continue
1. GitOps — чтение новых helm релизов и их применение.
2. Coverage тесты в CI, codecov, SonarQube.  
3. Нагрузочные тесты на GetCurrentUserQuery, Commands

Схема command запроса:
Схема command запроса:
```mermaid
flowchart LR
    Client[HTTP Client]
    Client -->|UserIp X-Forwarded-For| RateLimit{RateLimiter}
    RateLimit -->|Exceeded| RestError[REST Error 429]
    RateLimit -->|OK| TryFrom

    subgraph Validation
        TryFrom -->|Email + Password VO| LoginCommand
        TryFrom -->|Err| RestError
    end

    subgraph Application
        LoginCommand --> LoginCommandHandler
    end

    subgraph Initiate
        LoginCommandHandler -->|initiate_login cookie| AuthenticationPort
        AuthenticationPort --> KratosAuthenticationAdapter
        KratosAuthenticationAdapter -->|whoami| Kratos
        Kratos -->|SessionStatus| KratosAuthenticationAdapter
        KratosAuthenticationAdapter -->|fetch_flow| Kratos
        Kratos -->|flow_id + csrf_token| KratosAuthenticationAdapter
    end

    subgraph Complete
        LoginCommandHandler -->|complete_login credentials| AuthenticationPort
        AuthenticationPort --> KratosAuthenticationAdapter
        KratosAuthenticationAdapter -->|build| LoginPayload[LoginPayload Infra Model]
        LoginPayload -->|POST flow| Kratos
        Kratos -->|SessionCookie| KratosAuthenticationAdapter
    end

    KratosAuthenticationAdapter -->|SessionCookie| LoginCommandHandler
    LoginCommandHandler -->|session_token| Client
    Client -->|Set-Cookie| RestResponse[POST /auth/login Response]
```

Реализация кэша redis для запрос Query, что бы не загружать postgres.
```mermaid
flowchart TD
    Client[HTTP Client]
    Client -->|UserIp from X-Forwarded-For| RateLimit{RateLimiter}
    RateLimit -->|Exceeded| RestError[REST Error 429]
    RateLimit -->|OK| GetCurrentUserQuery
    Client -->|cookie from request| GetCurrentUserQuery

    GetCurrentUserQuery -->|cookie Option| GetCurrentUserQueryHandler
    GetCurrentUserQueryHandler -->|extract session token| CacheKey[cache_key: user_profile:token]

    CacheKey --> RedisLookup{Redis GET}
    RedisLookup -->|HIT| Deserialize[serde_json::from_str]
    Deserialize -->|UserProfile| RestResponse[GET /auth/me Response]

    RedisLookup -->|MISS| IdentityPort
    IdentityPort -->|get_current_user cookie| KratosIdentityAdapter
    KratosIdentityAdapter -->|GET /sessions/whoami| Kratos
    Kratos -->|401 Unauthorized| AuthError[AuthError::NotAuthenticated]
    AuthError --> RestError

    Kratos -->|SessionResponse| KratosIdentityAdapter
    KratosIdentityAdapter -->|traits.into| UserProfile
    UserProfile -->|serde_json::to_string| RedisSet[Redis SET EX cache_ttl_secs]
    RedisSet --> RestResponse
```

Валидация входных данных:
```mermaid
flowchart LR
    Input[REST Input Body]
    Input --> TryFrom[TryFrom]

    TryFrom --> VO[VO Email / Password]

    VO -->|Ok| Domain[Domain Object]
    VO -->|Err| Error[REST Error 422]

    Domain --> Handler[CommandHandler]
    Handler --> Adapter[KratosAdapter]

    Adapter --> Models[Infra Models]
    Models --> Kratos[Kratos]

    Kratos --> Response[FlowResult / PostFlowResult]

    Response --> Adapter
    Adapter --> Handler
    Handler --> RestResponse[REST Response]
```

## Running  
```bash
make up
```

## Testing  

Для запуска тестов в kratos требуется поднятие инфры (kratos, postgres, mailhog, redis):
```bash
cd web/backend/rust_kratos && make infra-up && cargo test ; cd ../../../
```

На фронтенде:
```bash
cd web/frontend && yarn install && yarn test ; cd ../../
```
