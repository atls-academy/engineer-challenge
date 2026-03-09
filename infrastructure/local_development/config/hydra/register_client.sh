#!/bin/sh
sleep 5
hydra create client \
  --endpoint http://hydra:4445 \
  --id my-client \
  --secret my-secret \
  --grant-type authorization_code,refresh_token \
  --response-type code \
  --scope openid,profile,email,offline \
  --redirect-uri http://localhost:3000/callback \
  --redirect-uri http://localhost:8080/ \
  --token-endpoint-auth-method client_secret_basic
