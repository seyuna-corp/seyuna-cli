#!/bin/bash

if npx semantic-release --dry-run; then
  echo "release_needed=true"
else
  echo "release_needed=false"
fi
