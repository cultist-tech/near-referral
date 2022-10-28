source neardev/.env

INFLUENCER_ID="muzikant.testnet"

NEAR_ENV="$NEAR_ENV" near view $CONTRACT_NAME referrals_supply_by_influencer --accountId $CONTRACT_NAME "{ \"influencer_id\": \"$INFLUENCER_ID\" }"
