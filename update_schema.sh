#!/usr/bin/env bash
source '.env'

# HASURA_ENDPOINT=http://localhost:8079/v1/graphql

# HASURA_GRAPHQL_ADMIN_SECRET=vx-template-app-access-key

gq ${HASURA_ENDPOINT} -H "X-Hasura-Admin-Secret: ${HASURA_GRAPHQL_ADMIN_SECRET}" --introspect > schema.graphql
