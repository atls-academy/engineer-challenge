### Kratos rust example service

- register
- login
- fake http email endpoint

### Execute

```
make infra-up
make run
```

### Available APP_ENV values

| APP_ENV value    | Используемый конфиг       |
|------------------|---------------------------|
| development      | development.toml          |
| production       | production.toml           |
| docker_local     | docker_local.toml         |
| -                | development.toml          |
