PREFIX="kujira"
DENOM="ukuji"
DAEMON="kujirad"
CHAIN_ID="harpoon-4"
RPC="https://kujira-testnet-rpc.polkachu.com:443"

DAPP_ADDRESS="kujira18tnvnwkklyv4dyuj8x357n7vray4v4zucptwfn"
MANTASWAP_ROUTER_ADDRESS="kujira1j80m5dtnxjcdewgccppjpl3sd6z6gnmvzs8avfnkyu96492qdu3qdsrnx3"
VAULT_ADDRESS="kujira1chgwz55h9kepjq0fkj5supl2ta3nwu63qzucu7"

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
CONTRACT_CODE="2448" # tx_hash: 33A8E5B92903329F7F9D62B1AD0C9B5E54D21F798DAF14A3E7F39E5D932908EE
# INIT="{\"router_address\":\"$MANTASWAP_ROUTER_ADDRESS\"}"
# yes $KEYRING_PASSWORD | $DAEMON tx wasm instantiate $CONTRACT_CODE "$INIT" --from "dapp" --label "$DIR_NAME_SNAKE-dev" $TXFLAG --admin $DAPP_ADDRESS

# # get smart contract address
# CONTRACT_ADDRESS=$($DAEMON q wasm list-contract-by-code $CONTRACT_CODE --node $RPC --chain-id $CHAIN_ID --output json | jq -r '.contracts[-1]')


# write data to file
CONTRACT_ADDRESS="kujira1wlkn8px6y5jfp9suqusdjmnhysd4594qsju87694gl5mlckjwscs87sfdg" # tx_hash: 7870493843B6920847C5970B21E85E1DA7D91888BB77BA8097077E3C1F27C8EF

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


# # Execute SwapIn to swap 0.0001 USK -> KUJI and send to vault with memo
# # tx_hash: 183CA82D8DA84692053D47EB85E496B8C7EC1B1DCF29005C04C4B02D928244B5
# # 1) Find denoms by code here https://api.mantadao.app/whitelist
# # mainnet: USK - factory/kujira1qk00h5atutpsv900x202pxx42npjr9thg58dnqpa72f2p7m2luase444a7/uusk, KUJI - ukuji
# # testnet: USK - factory/kujira1r85reqy6h0lu02vyz0hnzhv5whsns55gdt4w0d7ft87utzk7u0wqr4ssll/uusk, KUJI - ukuji
# # 2) Get contract payload sending POST request to https://api.mantadao.app/route
# # Request body
# # mainnet: {"input":{"denom":"factory/kujira1qk00h5atutpsv900x202pxx42npjr9thg58dnqpa72f2p7m2luase444a7/uusk","amount":"100","slippage":0.001},"output":{"denom":"ukuji"}}
# # testnet: {"input":{"denom":"factory/kujira1r85reqy6h0lu02vyz0hnzhv5whsns55gdt4w0d7ft87utzk7u0wqr4ssll/uusk","amount":"100","slippage":0.001},"output":{"denom":"ukuji"}}
# # We will get 'response' then 'response.routes[0].tx' will be 'MANTASWAP_MSG'
# # mainnet: MANTASWAP_MSG="{\"swap\":{\"stages\":[[[\"kujira14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sl4e867\",\"ibc/295548A78785A1007F232DE286149A6FF512F180AF5657780FC89C009E2C348F\"]],[[\"kujira1rwx6w02alc4kaz7xpyg3rlxpjl4g63x5jq292mkxgg65zqpn5llq202vh5\",\"factory/kujira1qk00h5atutpsv900x202pxx42npjr9thg58dnqpa72f2p7m2luase444a7/uusk\"]]],\"min_return\":[{\"denom\": \"ukuji\",\"amount\": \"1603\"}]}}"
# # testnet:
# MANTASWAP_MSG='{"swap":{"stages":[[["kujira1wl003xxwqltxpg5pkre0rl605e406ktmq5gnv0ngyjamq69mc2kqm06ey6","factory/kujira1r85reqy6h0lu02vyz0hnzhv5whsns55gdt4w0d7ft87utzk7u0wqr4ssll/uusk"]]],"min_return":[{"denom":"ukuji","amount":"80"}]}}'
# # 3) add vault address to compose 'SWAP_IN_MSG'
# SWAP_IN_MSG="{\"swap_in\":{\"vault_address\":\"$VAULT_ADDRESS\",\"mantaswap_msg\":$MANTASWAP_MSG}}"
# # 4) add funds
# # mainnet: FUNDS="100factory/kujira1qk00h5atutpsv900x202pxx42npjr9thg58dnqpa72f2p7m2luase444a7/uusk"
# # testnet:
# FUNDS="100factory/kujira1r85reqy6h0lu02vyz0hnzhv5whsns55gdt4w0d7ft87utzk7u0wqr4ssll/uusk"
# # 5) specify memo and call the contract
# MEMO=":BTC:yourbtcaddress"
# SWAP_IN_RES=$(yes $KEYRING_PASSWORD | $DAEMON tx wasm execute $CONTRACT_ADDRESS "$SWAP_IN_MSG" --from=$(echo $DAPP_ADDRESS) --amount "$FUNDS" --note "$MEMO" $TXFLAG --output json)
# echo $SWAP_IN_RES


# # Execute SwapOut to swap 0.0001 KUJI -> USK and send to user
# # tx_hash: 412BC583C28D59DDA77FA6E78DF536200BD59E09EC015F8305197FD0A06124E0
# # For SwapOut we have same steps but added optional IBC channel_id parameter required to transfer IBC token
# # to native network. It can be found here https://raw.githubusercontent.com/Team-Kujira/kujira.js/master/src/resources/tokens.json
# # For example we have 'usei' on kujira with IBC denom 'ibc/EB7D94B1B3D878F8461959A5A21DBB9D7EC6989F1347D67CC002805E772FE3F9'
# # The path is "transfer/channel-4" where 'channel-4' is channel_id.
# # There is no infra for IBC transfer on testnet. Then skip channel_id parameter
# MANTASWAP_MSG='{"swap":{"stages":[[["kujira1wl003xxwqltxpg5pkre0rl605e406ktmq5gnv0ngyjamq69mc2kqm06ey6","ukuji"]]],"min_return":[{"denom":"factory/kujira1r85reqy6h0lu02vyz0hnzhv5whsns55gdt4w0d7ft87utzk7u0wqr4ssll/uusk","amount":"110"}]}}'
# SWAP_OUT_MSG="{\"swap_out\":{\"user_address\":\"$VAULT_ADDRESS\",\"mantaswap_msg\":$MANTASWAP_MSG}}"
# FUNDS="100ukuji"
# SWAP_OUT_RES=$(yes $KEYRING_PASSWORD | $DAEMON tx wasm execute $CONTRACT_ADDRESS "$SWAP_OUT_MSG" --from=$(echo $DAPP_ADDRESS) --amount "$FUNDS" $TXFLAG --output json)
# echo $SWAP_OUT_RES
