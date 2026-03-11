# Auth Service
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=vwency_engineer-challenge&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=vwency_engineer-challenge) [![Bugs](https://sonarcloud.io/api/project_badges/measure?project=vwency_engineer-challenge&metric=bugs)](https://sonarcloud.io/summary/new_code?id=vwency_engineer-challenge) [![Code Smells](https://sonarcloud.io/api/project_badges/measure?project=vwency_engineer-challenge&metric=code_smells)](https://sonarcloud.io/summary/new_code?id=vwency_engineer-challenge) ![License](https://img.shields.io/github/license/vwency/engineer-challenge)

## Description  
Проект реализует функции восстановление пароля, регистрация, авторизации, максимально приближенные к prod-ready решениям.  
 
## Architecture

**DDD**  
- Фокус на доменной логике  
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
- [GraphQL Gateway Architecture](./docs/adr/0002-graphql-gateway.md)  

## Tech stack
1. **GraphQL**, поскольку поддерживает в запросе `Set-Cookies`, и дает Backward Compatibility.    
2. **Yarn berry** большое сообщество, кастомизация.  
3. **NX** время сборки, уменьшение времени на CI.  
4. **Rust** строгая типипизация, гарантия доставки, гибкость в архитектуре.


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
1. Написание сервиса `rust_hydra`, и сервиса для экстракции identity из access_token, кастомная реализация с кастомным payload для jwt.
2. Кэширование GetCurrentUserQuery в redis, при подготовке к prod-ready.
3. GitOps — чтение новых helm релизов и их применение.
4. Coverage тесты в CI, codecov, SonarQube.  

Схема command запроса:
```mermaid
flowchart TD
    GQL[GraphQL Gateway]
    GQL -->|LoginInput| TryFrom[TryFrom]
    TryFrom -->|Email + Password VO| LoginCommand
    TryFrom -->|Err| GQLError[GraphQL Error]
    LoginCommand --> LoginCommandHandler
    LoginCommandHandler -->|initiate_login cookie| AuthenticationPort
    AuthenticationPort -->|initiate_login cookie| KratosAuthenticationAdapter
    KratosAuthenticationAdapter -->|whoami| Kratos
    Kratos -->|SessionStatus| KratosAuthenticationAdapter
    KratosAuthenticationAdapter -->|fetch_flow| Kratos
    Kratos -->|flow_id + csrf_token| KratosAuthenticationAdapter
    LoginCommandHandler -->|complete_login credentials| AuthenticationPort
    AuthenticationPort -->|complete_login credentials| KratosAuthenticationAdapter
    KratosAuthenticationAdapter -->|build| LoginPayload[LoginPayload\nInfra Model]
    LoginPayload -->|POST flow| Kratos
    Kratos -->|SessionCookie| KratosAuthenticationAdapter
    KratosAuthenticationAdapter -->|SessionCookie| LoginCommandHandler
    LoginCommandHandler -->|session_token| GQL
    GQL -->|Set-Cookie| GQLResponse[GraphQL Response]
```

Валидация входных данных:
```mermaid
flowchart LR
    Input[GraphQL Input] --> TryFrom[TryFrom]
    TryFrom --> VO["VO валидация
Email / Password"]
    VO -->|Ok| Domain[Domain Object]
    VO -->|Err| Error[GraphQL Error]
    Domain --> Handler[CommandHandler]
    Handler --> Adapter[KratosAdapter]
    Adapter --> Models[Infra Models]
    Models --> Kratos[Kratos]
    Kratos --> Response[FlowResult / PostFlowResult]
    Response --> Adapter
    Adapter --> Handler
    Handler --> GQLResponse[GraphQL Response]
```


## Running  
```bash
make up
```

## Testing  

Для запуска тестов в kratos требуется поднятие инфры (kratos, postgres, mailhog):
```bash
cd web/backend/rust_kratos && make infra-up && cargo test ; cd ../../../
```

На фронтенде:
```bash
cd web/frontend && yarn install && yarn test ; cd ../../
```
