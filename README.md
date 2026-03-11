# Auth Service

[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=vwency_engineer-challenge&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=vwency_engineer-challenge) [![Bugs](https://sonarcloud.io/api/project_badges/measure?project=vwency_engineer-challenge&metric=bugs)](https://sonarcloud.io/summary/new_code?id=vwency_engineer-challenge) [![Code Smells](https://sonarcloud.io/api/project_badges/measure?project=vwency_engineer-challenge&metric=code_smells)](https://sonarcloud.io/summary/new_code?id=vwency_engineer-challenge) ![License](https://img.shields.io/github/license/vwency/engineer-challenge)

## Запуск
```bash
make up
```

## Функционал

1. Регистрация
2. Авторизация
3. Восстановление по почте

### Trade-offs

1. Есть дублирование стилей/tsx. (скорость прототипирования)
2. Использование redux. (скорость прототипирования + архитектура)
3. Webpack (HMR, hot-reload)
4. Нет подтверждения пароля по почте при регистрация.(время отладки)
5. Нету полноценного IaC, minikube не очень хорош bare metal развертке.
6. Не использовал jwt посколько сервис 1 нету экосистемы сервисов, сессия шариться cross-domain, если cookie-based и делаем запрос с другого домена с credentials: include. 

## ADR

### [backend](./backend)

GraphQL поддерживает `Set-Cookies`.
Паттерны **DDD** и **DI**.
Ory экосистема

### [frontend](./frontend)

Монорепозиторий на **webpack** (поддержка HMR), **Nx**, **Next.js**.
**Redux**

### Проблемные места

1. После login, registeration нет редиректов на homepage.
2. Нету rate-limiting.
3. Hardcode
4. Hydra если будет расти экосистема и будут отдельные сервисы для sharing-permissions

### Continue

1. rust_hydra, централизованный сервис по валидации и extract user traits из access_token.
2. имплементация OpenID compability.
3. GitOps чтение новых helm релизов, из применение.
4. Локальный раннер github actions.
5. Coverage тесты в ci, codecov, SonarQube

Схема упрощена, без **CommandHandler**

```mermaid
flowchart TD
flowchart TD
    GQL[GraphQL Gateway]

    GQL --> Commands
    GQL --> Queries

    subgraph Commands
        LoginCommand
        LogoutCommand
        RecoveryCommand
        UpdateSettingsCommand
        RegisterCommand
        VerificationCommand
    end

    subgraph Queries
        GetCurrentUserQuery
    end

    subgraph Ports
        AuthenticationPort
        SessionPort
        RecoveryPort
        SettingsPort
        RegistrationPort
        VerificationPort
        IdentityPort
    end

    LoginCommand --> AuthenticationPort
    LogoutCommand --> SessionPort
    RecoveryCommand --> RecoveryPort
    UpdateSettingsCommand --> SettingsPort
    RegisterCommand --> RegistrationPort
    VerificationCommand --> VerificationPort
    GetCurrentUserQuery --> IdentityPort

    subgraph KratosAdapters
        KratosAuthenticationAdapter
        KratosSessionAdapter
        KratosRecoveryAdapter
        KratosSettingsAdapter
        KratosRegistrationAdapter
        KratosVerificationAdapter
        KratosIdentityAdapter
    end

    AuthenticationPort --> KratosAuthenticationAdapter
    SessionPort --> KratosSessionAdapter
    RecoveryPort --> KratosRecoveryAdapter
    SettingsPort --> KratosSettingsAdapter
    RegistrationPort --> KratosRegistrationAdapter
    VerificationPort --> KratosVerificationAdapter
    IdentityPort --> KratosIdentityAdapter

    KratosAuthenticationAdapter --> Kratos
    KratosSessionAdapter --> Kratos
    KratosRecoveryAdapter --> Kratos
    KratosSettingsAdapter --> Kratos
    KratosRegistrationAdapter --> Kratos
    KratosVerificationAdapter --> Kratos
    KratosIdentityAdapter --> Kratos
```

### Тесты

Для запуска тестов в kratos требуется поднятие инфры(kratos, postgres, mailhog)
```
cd backend/rust-kratos ; make infra-up ; cargo test
```
На фронтенде:
```
cd frontend ; yarn test
```

## PS

#### Почему не jwt/OpenID?

1. Работать с jwt не так приятно как с сессий, нужно вручную сохранять токены на frontend.
2. Безопаснее
3. Проще

#### Почему не grpc

1. Ручное сохранение ответа от gRPC на frontend в куки(в случае cookie-based).

#### Когда jwt?

1. Использование OAuth2 с OpenID, что бы была интеграция между разными сервисами 1 экосистемы,
например чтение письм пользователей в яндекс почте, яндекс телемост итд.
kratos сохраняет original токены jwt при использовании OAuth2.
