# script for building contract

DIR_NAME=$(echo ${PWD##*/})
DIR_NAME_SNAKE=$(echo $DIR_NAME | tr '-' '_')
WASM="$DIR_NAME_SNAKE.wasm"


# generate schema
cargo schema

# fix for ts-codegen MissingPointerError
# https://github.com/CosmWasm/ts-codegen/issues/90
rm -rf ./schema/raw

# build optimized binary
cd ../..
cargo cw-optimizoor

# rename wasm files
cd artifacts
for file in *-*\.wasm; do
    prefix=${file%-*}
    mv "$file" "$prefix.wasm"
done

# check if contract is ready to be uploaded to the blockchain
if [ -e $WASM ]; then
    cosmwasm-check --available-capabilities iterator,stargate,staking $WASM
fi
