source neardev/.env

INFLUENCER_ID="muzikant.testnet"
PROGRAM_ID="1"

NEAR_ENV="$NEAR_ENV" near call $CONTRACT_NAME referral_create_program --accountId $CONTRACT_NAME "{ \"influencer_id\": \"$INFLUENCER_ID\", \"program_id\": \"$PROGRAM_ID\", \"royalty_percent\": 0 }" --gas 40000000000000 --deposit "0.1"
