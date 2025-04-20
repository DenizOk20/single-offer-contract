# Soroban Single Offer Smart Contract

This is a smart contract built using [Soroban](https://soroban.stellar.org/) that allows a seller to create a **single fixed-rate offer** to trade one token for another.

## 🧩 Features

- Create a trade offer between two tokens
- Enforce fixed-price trades (reject trades that don't meet expected price)
- Allow the seller to update prices or withdraw remaining tokens
- Includes full unit tests for functionality and authorization logic

## 📂 Project Structure

- `src/lib.rs`: Contract implementation
- `src/test.rs`: Unit test

## 🚀 Getting Started

Make sure you have Rust and Soroban CLI installed. Then:

```bash
git clone https://github.com/DenizOk20/single-offer.git
cd single-offer
cargo test
