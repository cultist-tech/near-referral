source neardev/.env

INFLUENCER_ID="muzikant.testnet"
PROGRAM_ID="1"

NEAR_ENV="$NEAR_ENV" near call $CONTRACT_NAME referral_accept --accountId "mfight-nft_v2.testnet" "{ \"contract_id\": \"$CONTRACT_NAME\", \"influencer_id\": \"$INFLUENCER_ID\", \"program_id\": \"$PROGRAM_ID\" }" --gas 40000000000000 --deposit "0.02"
