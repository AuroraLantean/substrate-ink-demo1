# Substrate Ink Contracts

### Installation

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

### Download the contract-pallet node:
https://github.com/paritytech/substrate-contracts-node

Debug output in console and trace host function calls in ContractsUI is automatically enabled when you run the contract-pallet node:
`./substrate-contracts-node`

### Upload and Instanticate the contract
https://use.ink/getting-started/deploy-your-contract

UI to deploy .contract file: https://ui.use.ink/
go to top left > Add New Contract >
... select alice account or something else
... enter contract name
... upload an .contract file at target/ink/xyz.contract

upload the contract code once, and instantiate it many times!
select default constructor and its default value: new, false
//enter "Max Gas Allowed" to 200,000

OR
```
cargo contract upload --suri //Alice
cargo contract instantiate --suri //Alice --args true
```

### Call Contracts
https://use.ink/getting-started/calling-your-contract

#### Read Values: Dry-run via RPC
only for read functions
Use UI or ...
```
cargo contract instantiate --execute --suri //Alice --args true
... get Code Hash and Contract Address
addr=<insert-contract-address>
cargo contract call --contract $addr --message get --suri //Alice
```
#### Write Values: State mutating via submitting a Transaction
This requires tokens of the network to pay for the cost of the transaction. The transaction will be put in a transaction pool and asynchronously processed. During submission of the transaction no result is available. 
Contract developers have to make sure that events are emitted if they want clients to be able to pick up on them.

Use UI or ...
```
cargo contract call --contract $addr --message flip --execute --suri //Alice
```

### Troubleshooting
#### Unexpected Epoch Change
There is a known issue with the Substrate block production (BABE) on a running chain. If you stop your node for too long (closing the terminal, putting your computer to sleep, etc.), you will get the following error:
`ClientImport("Unexpected epoch change")`
To solve this you will need to restart your node and re-deploy any contracts


### TODO 
https://use.ink/examples/smart-contracts

inside https://github.com/use-ink/ink-examples:
- basic_contract_ref ‒ Implements cross-contract calling.
- trait-erc20 ‒ Defines a trait for Erc20 contracts and implements it.
- erc721 ‒ An exemplary implementation of Erc721 NFT tokens.
