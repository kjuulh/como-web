#!/bin/bash

graphql-client introspect-schema \
  http://localhost:3001/graphql \
  --header "Authorization: Basic $COMO_GATEWAY_PAT" \
  --output src/api/graphql/schema/schema.json

graphql-client generate \
  --schema-path src/api/graphql/schema/schema.json \
  src/api/graphql/schema/mutations.graphql \
  --output-directory src/api/graphql/gen

graphql-client generate \
  --schema-path src/api/graphql/schema/schema.json \
  src/api/graphql/schema/queries.graphql \
  --output-directory src/api/graphql/gen
