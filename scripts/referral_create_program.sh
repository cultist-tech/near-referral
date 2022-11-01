source neardev/.env

INFLUENCER_ID="muzikant.testnet"
CONTRACT_ID="mfight-nft_v2.testnet"
PROGRAM_ID="1"

NEAR_ENV="$NEAR_ENV" near call $CONTRACT_NAME referral_create_program --accountId $CONTRACT_ID "{ \"influencer_id\": \"$INFLUENCER_ID\", \"program_id\": \"$PROGRAM_ID\", \"royalty_percent\": 0, \"metadata\": { \"title\": \"Test title\", \"url\": \"mfight.io\", \"media\": \"https://loremflickr.com/640/480/abstract\", \"description\": \"test description\"} }" --gas 40000000000000 --deposit "0.1"
