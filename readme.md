# Substrate Ink Contracts

## Installation

https://github.com/use-ink/ink
```
cargo install cargo-contract --force
cargo contract --version
git clone <THIS_REPO_GIT_LINK>:<THIS_REPO_NAME> && cd <THIS_REPO_NAME>
cargo test
cargo contract build
cargo contract build --release
```

Then you should get those files inside `target` folder:
- flipper.wasm ... raw Wasm binary(contract bytecode)
- flipper.json ... metadata, which is not stored on-chain, used by dApps
- <contract-name>.contract ... Combines both the contract's bytecode and the metadata. 

Download the contract-pallet node:
https://github.com/paritytech/substrate-contracts-node

Debug output in console and trace host function calls in ContractsUI is automatically enabled when you run the contract-pallet node:
`./substrate-contracts-node`

TODO inside https://github.com/use-ink/ink-examples:
- basic_contract_ref ‒ Implements cross-contract calling.
- trait-erc20 ‒ Defines a trait for Erc20 contracts and implements it.
- erc721 ‒ An exemplary implementation of Erc721 NFT tokens.
