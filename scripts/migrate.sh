#!/bin/bash
source neardev/.env

# migrate state
NEAR_ENV="$NEAR_ENV" near call $CONTRACT_NAME migrate --accountId $CONTRACT_NAME "{ }"
