source neardev/.env

INFLUENCER_ID="muzikant.testnet"
PROGRAM_ID="1"

NEAR_ENV="$NEAR_ENV" near view $CONTRACT_NAME referrals_supply_by_program --accountId $CONTRACT_NAME "{ \"contract_id\": \"$CONTRACT_NAME\", \"influencer_id\": \"$INFLUENCER_ID\", \"program_id\": \"$PROGRAM_ID\" }"
