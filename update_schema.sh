#!/usr/bin/env bash
#source '.env'

gq ${HASURA_ENDPOINT} -H "X-Hasura-Admin-Secret: ${HASURA_GRAPHQL_ADMIN_SECRET}" --introspect > schema.graphql
