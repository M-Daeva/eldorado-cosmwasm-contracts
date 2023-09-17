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
CONTRACT_CODE="4288" # tx_hash: BFF9C1317F5818830A0169BB6AD1FA965CB10E779FD143DDC898B2C8912370A1
# INIT="{}"
# yes $KEYRING_PASSWORD | $DAEMON tx wasm instantiate $CONTRACT_CODE "$INIT" --from "dapp" --label "$DIR_NAME_SNAKE-dev" $TXFLAG --admin $DAPP_ADDRESS

# # get smart contract address
# CONTRACT_ADDRESS=$($DAEMON q wasm list-contract-by-code $CONTRACT_CODE --node $RPC --chain-id $CHAIN_ID --output json | jq -r '.contracts[-1]')


# write data to file
CONTRACT_ADDRESS="osmo1pmhdtae4zyvjveva9f6a9tgenslyr46w5ws67u7447473cn9gqxqhl92d7" # tx_hash: A990D3E8E257565BF3695247989EB7D8AAE086BB91472708618A380C1FBCDCAC

# R="{
# \"PREFIX\":\"$PREFIX\",
# \"CHAIN_ID\":\"$CHAIN_ID\",
# \"RPC\":\"$RPC\",
# \"CONTRACT_CODE\":\"$CONTRACT_CODE\",
# \"CONTRACT_ADDRESS\":\"$CONTRACT_ADDRESS\"
# }"

# echo $R > "../../config/${DAEMON::-1}-testnet-config.json"


# # query config
# CONFIG_QUERY_MSG="{\"query_config\":{}}"
# CONFIG=$($DAEMON q wasm contract-state smart $CONTRACT_ADDRESS "$CONFIG_QUERY_MSG" --node $RPC --chain-id $CHAIN_ID --output json)
# echo $CONFIG

# # tx_hash: 2E2E3E8999AB3EAD3DF3966C8E2827EE068C7EF95BA109EF46050F6D9B5846DC
# ATOM="ibc/B28CFD38D84A480EF2A03AC575DCB05004D934A603A5A642888847BCDA6340C0"
# SWAP_RES=$(yes $KEYRING_PASSWORD | $DAEMON tx gamm swap-exact-amount-in "100$ATOM" "1" --swap-route-denoms "uosmo" --swap-route-pool-ids 151 --from=$(echo $DAPP_ADDRESS) $TXFLAG --output json)
# echo $SWAP_RES

# # Execute SwapIn to swap 0.0001 ATOM -> OSMO and send to vault with memo
# # tx_hash: 5CD9496FE4B2497A6DB2E585F6884A08CE3B2627362F31CA337129BC07C7CA2F
# # 1) add vault address to compose 'SWAP_IN_MSG'
# POOL_ID=151
# SWAP_IN_MSG="{\"swap_in\":{\"vault_address\":\"$VAULT_ADDRESS\",\"pool_id\":$POOL_ID}}"
# # 2) add funds (get denom by symbol here https://raw.githubusercontent.com/osmosis-labs/assetlists/main/osmosis-1/osmosis-1.assetlist.json)
# # mainnet: FUNDS="100ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2"
# # testnet:
# FUNDS="100ibc/B28CFD38D84A480EF2A03AC575DCB05004D934A603A5A642888847BCDA6340C0"
# # 3) specify memo and call the contract
# MEMO=":BTC:yourbtcaddress"
# SWAP_IN_RES=$(yes $KEYRING_PASSWORD | $DAEMON tx wasm execute $CONTRACT_ADDRESS "$SWAP_IN_MSG" --from=$(echo $DAPP_ADDRESS) --amount "$FUNDS" --note "$MEMO" $TXFLAG --output json)
# echo $SWAP_IN_RES


# Execute SwapOut to swap 0.0001 OSMO -> ATOM and send to user
# For SwapOut we have same steps but added optional IBC channel_id parameter required to transfer IBC token
# to native network. It also can be found here https://raw.githubusercontent.com/osmosis-labs/assetlists/main/osmosis-1/osmosis-1.assetlist.json
# For example on mainnet we have 'uatom' on Osmosis with IBC denom
# 'ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2', the channel_id is 'channel-0'.

# # Osmisis -> Osmosis, tx_hash: 902DB2A379C4F129CDB3FF3169B158B172A072290065D3276FAB6B1C3FCCAF3E
# POOL_ID=151
# SWAP_OUT_MSG="{\"swap_out\":{\"user_address\":\"$VAULT_ADDRESS\",\"pool_id\":$POOL_ID}}"
# FUNDS="100uosmo"
# SWAP_OUT_RES=$(yes $KEYRING_PASSWORD | $DAEMON tx wasm execute $CONTRACT_ADDRESS "$SWAP_OUT_MSG" --from=$(echo $DAPP_ADDRESS) --amount "$FUNDS" $TXFLAG --output json)
# echo $SWAP_OUT_RES

# # Osmosis -> Cosmos Hub, tx_hash: 8D48FF0C3DDADE559E426CE5A49EE429528F801D6713A1024438F54D98B14483
# USER_ADDRESS_COSMOS_HUB="cosmos1chgwz55h9kepjq0fkj5supl2ta3nwu63327q35"
# CHANNEL_ID="channel-1497"
# POOL_ID=151
# SWAP_OUT_MSG="{\"swap_out\":{\"user_address\":\"$USER_ADDRESS_COSMOS_HUB\",\"pool_id\":$POOL_ID,\"channel_id\":\"$CHANNEL_ID\"}}"
# FUNDS="100uosmo"
# SWAP_OUT_RES=$(yes $KEYRING_PASSWORD | $DAEMON tx wasm execute $CONTRACT_ADDRESS "$SWAP_OUT_MSG" --from=$(echo $DAPP_ADDRESS) --amount "$FUNDS" $TXFLAG --output json)
# echo $SWAP_OUT_RES
