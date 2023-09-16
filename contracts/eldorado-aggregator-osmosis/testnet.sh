PREFIX="osmo"
DENOM="uosmo"
DAEMON="osmosisd"
CHAIN_ID="osmo-test-5"
RPC="https://rpc.osmotest5.osmosis.zone:443"

DAPP_ADDRESS="osmo18tnvnwkklyv4dyuj8x357n7vray4v4zupj6xjt"
VAULT_ADDRESS="osmo1chgwz55h9kepjq0fkj5supl2ta3nwu63e3ds8x"

TXFLAG="--gas-prices 0.0025$DENOM --gas auto --gas-adjustment 1.3 -y --node $RPC --chain-id $CHAIN_ID"
DIR_NAME=$(echo ${PWD##*/})
DIR_NAME_SNAKE=$(echo $DIR_NAME | tr '-' '_')
WASM="$DIR_NAME_SNAKE.wasm"
KEYRING_PASSWORD="12345678"


# $DAEMON q bank balances $DAPP_ADDRESS --node $RPC --chain-id $CHAIN_ID


# cd ../../artifacts
# yes $KEYRING_PASSWORD | $DAEMON tx wasm store $WASM --from dapp $TXFLAG
# CONTRACT_CODE=$(yes $KEYRING_PASSWORD | $DAEMON tx wasm store $WASM --from dapp $TXFLAG --output json | jq -r '.logs[0].events[-1].attributes[1].value')
# echo contract code is $CONTRACT_CODE


# instantiate smart contract
CONTRACT_CODE="4249" # tx_hash: 6E2278989A4AE4595D50CD034F88DBA594673D2CD8DDD03B8D859DC877DBC0EF
# INIT="{}"
# yes $KEYRING_PASSWORD | $DAEMON tx wasm instantiate $CONTRACT_CODE "$INIT" --from "dapp" --label "$DIR_NAME_SNAKE-dev" $TXFLAG --admin $DAPP_ADDRESS

# # get smart contract address
# CONTRACT_ADDRESS=$($DAEMON q wasm list-contract-by-code $CONTRACT_CODE --node $RPC --chain-id $CHAIN_ID --output json | jq -r '.contracts[-1]')


# write data to file
CONTRACT_ADDRESS="osmo1j6v7z76em9ye5jcee22agw9j6g5tmqk7nyg6vcy5cmyk406wgrnq8u0ujc" # tx_hash: 3CF15FEA102835EDDDFAA4E7371FC0B51EB06815707361A98CCC4CF167D8A0FF

R="{
\"PREFIX\":\"$PREFIX\",
\"CHAIN_ID\":\"$CHAIN_ID\",
\"RPC\":\"$RPC\",
\"CONTRACT_CODE\":\"$CONTRACT_CODE\",
\"CONTRACT_ADDRESS\":\"$CONTRACT_ADDRESS\"
}"

# echo $R > "../../config/${DAEMON::-1}-testnet-config.json"


# # query config
# CONFIG_QUERY_MSG="{\"query_config\":{}}"
# CONFIG=$($DAEMON q wasm contract-state smart $CONTRACT_ADDRESS "$CONFIG_QUERY_MSG" --node $RPC --chain-id $CHAIN_ID --output json)
# echo $CONFIG


# Execute SwapIn to swap 0.0001 ATOM -> OSMO and send to vault with memo
# tx_hash: 
# 1) add vault address to compose 'SWAP_IN_MSG'
SWAP_IN_MSG="{\"swap_in\":{\"vault_address\":\"$VAULT_ADDRESS\"}}"
# 2) add funds (get denom by symbol here https://raw.githubusercontent.com/osmosis-labs/assetlists/main/osmosis-1/osmosis-1.assetlist.json)
# mainnet: FUNDS="100ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2"
# testnet:
FUNDS="100ibc/B28CFD38D84A480EF2A03AC575DCB05004D934A603A5A642888847BCDA6340C0"
# 3) specify memo and call the contract
MEMO=":BTC:yourbtcaddress"
SWAP_IN_RES=$(yes $KEYRING_PASSWORD | $DAEMON tx wasm execute $CONTRACT_ADDRESS "$SWAP_IN_MSG" --from=$(echo $DAPP_ADDRESS) --amount "$FUNDS" --note "$MEMO" $TXFLAG --output json)
echo $SWAP_IN_RES


# Execute SwapOut to swap 0.0001 OSMO -> ATOM and send to user
# tx_hash: 
# For SwapOut we have same steps but added optional IBC channel_id parameter required to transfer IBC token
# to native network. It also can be found here https://raw.githubusercontent.com/osmosis-labs/assetlists/main/osmosis-1/osmosis-1.assetlist.json
# For example we have 'uatom' on Osmosis with IBC denom 'ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2'
# The channel_id is 'channel-0'.
# There is no infra for IBC transfer on testnet. Then skip channel_id parameter
DENOM_OUT="ibc/B28CFD38D84A480EF2A03AC575DCB05004D934A603A5A642888847BCDA6340C0"
SWAP_OUT_MSG="{\"swap_out\":{\"user_address\":\"$VAULT_ADDRESS\",\"denom_out\":$DENOM_OUT}}"
FUNDS="100uosmo"
SWAP_OUT_RES=$(yes $KEYRING_PASSWORD | $DAEMON tx wasm execute $CONTRACT_ADDRESS "$SWAP_OUT_MSG" --from=$(echo $DAPP_ADDRESS) --amount "$FUNDS" $TXFLAG --output json)
echo $SWAP_OUT_RES

