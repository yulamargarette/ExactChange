# ExactChange Mall

Digital change credits for exact payments in malls and supermarkets.

## Problem
Cashiers struggle with lack of coins, causing delays and long lines.

## Solution
Customers convert bills into digital credits and pay exact amounts via Stellar.

## Timeline
- Week 1: Contract + wallet demo
- Week 2: QR checkout + kiosk simulation

## Stellar Features Used
- Custom tokens
- Soroban smart contracts
- Fast low-cost payments

## Vision
Reduce friction in physical retail transactions.

## Prerequisites
- Rust
- Soroban CLI

## Build
soroban contract build

## Test
cargo test

## Deploy
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/exact_change_mall.wasm

## Sample Call
soroban contract invoke \
--id <CONTRACT_ID> \
--fn pay \
--arg <USER> \
--arg <MERCHANT> \
--arg 47

## License
MIT

https://stellar.expert/explorer/testnet/tx/f7c1fc3b11747a3915545392ed89f7b094ba85f9d6da128090abd9114847b0b1