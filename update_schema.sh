#!/usr/bin/env bash

gq https://api.vx-template-app.network/v1/graphql -H "X-Hasura-Admin-Secret: vx-template-app-access-key" --introspect > schema.graphql
