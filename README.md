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
5. Нету полноценного IaC, выяснилось что minikube забирает себе порты нужные кластеру для cillium,
при его применении через helm, развернуть чистую ВМ с кластером долго.

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

### Continue

1. GitOps чтение новых helm релизов, из применение.
2. Локальный раннер github actions.
3. Coverage тесты в ci, codecov, SonarQube

Схема упрощена, без **CommandHandler**

```mermaid
flowchart TD
    A[graphql-gateway]
    A --> D{Commands}
    A --> E[Queries]

    D --> F[LoginCommand]
    D --> FF[LogoutCommand]
    D --> FFF[RecoveryCommand]
    D --> FFFF[SettingsCommand]
    D --> FFFFF[RegisterCommand]
    D --> FFFFFF[VerificationCommand]

    E --> AA[GetCurrentUserQuery]

    F --> U{LoginPort}
    FF --> UU{LogoutPort}
    FFF --> UUU{RecoveryPort}
    FFFF --> UUUU{SettingsPort}
    FFFFF --> UUUUU{RegisterPort}
    FFFFFF --> UUUUUU{VerificationPort}

    AA --> UUUUUUU{SessionPort}

    T[KratosAdapter]

    U --> T
    UU --> T
    UUU --> T
    UUUU --> T
    UUUUU --> T
    UUUUUU --> T
    UUUUUUU --> T
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
