#!/bin/bash

graphql-client introspect-schema \
  http://localhost:3001/graphql \
  --header "Authorization: Basic $COMO_GATEWAY_PAT" \
  --output src/api/graphql/schema/schema.json

graphql-client generate \
  --schema-path src/api/graphql/schema/schema.json \
  src/features/navbar_projects/graphql/queries.graphql \
  --output-directory src/features/navbar_projects/gen \
  --custom-scalars-module='crate::common::graphql' \
  --variables-derives='Clone,Debug' \
  --response-derives='Clone,Debug'

graphql-client generate \
  --schema-path src/api/graphql/schema/schema.json \
  src/features/dashboard_list_view/graphql/queries.graphql \
  --output-directory src/features/dashboard_list_view/gen \
  --custom-scalars-module='crate::common::graphql' \
  --variables-derives='Clone,Debug' \
  --response-derives='Clone,Debug'
