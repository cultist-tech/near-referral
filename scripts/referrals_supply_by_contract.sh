source neardev/.env

NEAR_ENV="$NEAR_ENV" near view $CONTRACT_NAME referrals_supply_by_contract --accountId $CONTRACT_NAME "{ \"contract_id\": \"$CONTRACT_NAME\" }"
