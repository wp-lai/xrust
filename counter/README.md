## Counter

A simple counter smart contract on NEAR

## Test

`$ cargo test -- --nocapture`

## Compile the code to WASM

`$ rustup target add wasm32-unknown-unknown`
`$ cargo build --target wasm32-unknown-unknown --release`

## Deploy the smart contract with `near-cli`

`$ near login`
`$ near deploy --wasmFile target/wasm32-unknown-unknown/release/counter.wasm --accountId YOUR_ACCOUNT_HERE`

## Invoke contract methods

Get counter number

`$ near view YOUR_ACCOUNT_HERE get_num --accountId YOUR_ACCOUNT_HERE`

Counter increment

`$ near call YOUR_ACCOUNT_HERE increment --accountId YOUR_ACCOUNT_HERE`

Counter decrement

`$ near call YOUR_ACCOUNT_HERE decrement --accountId YOUR_ACCOUNT_HERE`

Counter reset

`$ near call YOUR_ACCOUNT_HERE reset --accountId YOUR_ACCOUNT_HERE`
