source neardev/.env

INFLUENCER_ID="muzikant.testnet"
PROGRAM_ID="1"
CONTRACT_ID="mfight-nft_v2.testnet"

NEAR_ENV="$NEAR_ENV" near view $CONTRACT_NAME referral_program --accountId $CONTRACT_NAME "{ \"contract_id\": \"$CONTRACT_ID\", \"influencer_id\": \"$INFLUENCER_ID\", \"program_id\": \"$PROGRAM_ID\" }"
