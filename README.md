# jupiter
Testnet blockchain for Substrate `FRAME contracts pallet`.

Currently Jupiter provide an independent substrate blockchain node, a parachain node and a local dev test node(three type executable file), 
which all contain `pallet-contracts` module to provide contract feature. In the future, the Jupiter parachain node would become a
parachain for Rococo and Kusama.

All type node use `FRAME contracts pallet` to provide contract feature, and allow [`ink!`](https://github.com/paritytech/ink),
[`Ask!`](https://github.com/patractlabs/ask) and [`Solong`](https://github.com/hyperledger-labs/solang) or else compatible project
to deploy and run contracts on Jupiter blockchain.

*In following, we name `FRAME contracts pallet` as `pallet-contracts`*

## Jupiter's `FRAME contracts pallet`

Jupiter `FRAME contracts pallet` contains our `ChainExtension` to provide many particular features for contracts, and contains some compatible modifications.
* ChainExtension
    * Contract Logger support
    * Zero-Knowledge support
* Jupiter modifications
    * // No features for now.
    
Thus we call "`src pallet-contract`" for the `pallet-contract` module which comes from substrate directly, and call "`modified pallet-contracts`" for 
the `pallet-contract` which comes from our forked substrate repo in `vendor` directory.

Inside:
* `src pallet-contract`: support Jupiter `ChainExtension`
* `modified pallet-contract`: support Jupiter `ChainExtension` and Jupiter modifications

## Jupiter node type
This repo provide three type node:
* Jupiter Testnet chain node:
 
    It's an independent blockchain network which contains contracts feature. Anyone could submit 
    contract to this network for test. The token decimals and time interval for producing block are
    same with polkadot network. This testnet blockchain uses Babe as consensus algorithm, 
    and **provide Babe VRF random number for contract module.**
  
    This node uses `modified pallet-contract` now.
  
* Jupiter parachain node: (This node is not merged into `master` branch now.)

    It's a parachain blockchain network which would be deployed to Kusama network. Recently, we would deploy this parachain
    node into Rococo V1 network. And in this parachain, we would try to use the random number which relayed from relay chain in future.

    This node uses `src pallet-contract` now.
    
* Jupiter local develop node: 
    
    It's a local develop node for testing contract easily. This node could run in single, and **do not have time interval
    for producing block**, which would produce block only when receiving an extrinsic. This feature let developers do not 
    need to waste time for waiting producing blocks in develop and test.

    This node uses `modified pallet-contract` now.

## Jupiter Extending types
When using [Substrate Portal](https://polkadot.js.org/apps), [@polkadot/api](https://github.com/polkadot-js/api) and [Redspot](https://github.com/patractlabs/redspot) 
or other 3rd parties client to connect Jupiter node, please remember to add ["extending types"](https://polkadot.js.org/docs/api/start/types.extend/) for Jupiter requirements.

Jupiter **current** "extending types" is (This may be changed for different Jupiter version):
```json
{
  "LookupSource": "MultiAddress"
}
```

## Substrate newest master and Substrate v2.0.0 version
Note: Substrate newest master is very different with Substrate v2.0.0 version. In Jupiter `master` branch, we use Substrate newest
master as dependencies, and in Jupiter `substrate-v2.0.0` branch, we use Substrate v2.0.0 from "crates.io" as dependencies.

Thus, jupiter has two main branch. In different branch, we provide different `pallet-contracts` features:
* master: we track newest master, currently we use substrate commit:[`c88b104028b4fdda35311adb12eabf7aa5aa4316`](https://github.com/paritytech/substrate/commit/c88b104028b4fdda35311adb12eabf7aa5aa4316).
    * In master, we use `src pallet-contract` or `modified pallet-contract`
    * `src pallet-contract` is belong to substrate commit version.
    * `modified pallet-contract` is belong to the forked substrate in vendor directory. We usually track the newest substrate version.
* [substrate-v2.0.0](https://github.com/patractlabs/jupiter/tree/substrate-v2.0.0): we keep v2.0.0 dependencies in this branch. **But this branch could only provide version 2.0.0 `pallet-contracts`.**
    * Version 2.0.0 may not match for newest ink!. If you what to test contract in this branch, please peek a suitable ink!
      version for v2.0.0 `palset-contracts`.
    * In this branch, the Extending types must use the following (or switch to this branch, the readme would show more details):
        ```json
        {
           "Address": "AccountId",
           "LookupSource": "AccountId",
           "GeneratedContractAddressType": {
               "_enum": [
                  "Original",
                  "Repeatable"
               ]
           }
        }
        ```

## compile and run
### 1. clone this repo
```bash
> git clone --recurse-submodules https://github.com/patractlabs/jupiter.git
## or do following commands
> git clone https://github.com/patractlabs/jupiter.git
> cd jupiter/vendor
> git submodule update --init --recursive
```
If you want to use `substrate-v2.0.0` branch, do following commands:
```bash
> git clone --branch substrate-v2.0.0 https://github.com/patractlabs/jupiter.git
## or do following commands:
> git clone https://github.com/patractlabs/jupiter.git
> git checkout -t origin/substrate-v2.0.0
```

### 2. init building environment
*Note: if you have build substrate successfully, you do not need to do this any more*

You should prepare rust environment before, lookup [this link for rust](https://www.rust-lang.org/learn/get-started) to
downland rust and install it.
 
then:
```bash
> cd jupiter/scripts
> bash init.sh
```

### 3. compile this repo
Compile this repo follow this step:
```bash
> WASM_BUILD_TYPE=release cargo build --release
```
or we advice you to add `WASM_BUILD_TYPE=release` in your global environment variables, so that you could just execute:
```bash
cargo build --release
```

If you just need debug execution file, just do
```bash
cargo build
```

### 4. run
Now the directory `target/release` would contain two executable files.
* `jupiter`, which is Jupiter Testnet node.
* `jupiter-para`, which is Jupiter Testnet parachain node. (Cannot produce this file now)
* `jupiter-dev`, which is Jupiter local develop node.

Following parameters means:
* `--dev`: means start with dev mode, would provide default private key to start the chain, and running a independent node.
* `-d <directory>`: means blockchain database stored in the `<directory>`, if you what to clear all data, just stop the
node and delete this directory. If you do not use this parameter, the node would use default directory to store data.
* `--execution=<STRATEGY>`: substrate provide `Native` and `WASM` and other execution strategy. for test and develop, 
we suggest to use `Native`  

More parameters please lookup from `--help` or Substrate Developer Hub [https://substrate.dev/](https://substrate.dev/)

#### 4.1 jupiter
`jupiter` is a complete testnet blockchain, which developer could run in local, or join the public jupiter testnet.

The blockchain produce blocks in `babe` algorithm for every **6 second**, and do finality by `grandpa` algorithm. 
All people could join this testnet, either a sync node or a validator node. 

**Welcome use this link (TODO) to claim some JPT from our faucet**

##### 4.1.1 join jupiter testnet
// TODO. not support yet 

Moving the execution file `jupiter` to other place and launch the node by following steps:
```bash
jupiter --chain=testnet --name=<set you custom name> --pruning=archive --execution=NativeElseWasm
```
Those steps is same to any other substrate node, just should notice this parameter `--chain` should use `testnet`

##### 4.1.2 start local dev mode jupiter blockchain
Notice this dev mode just means jupiter start with `--dev`, which is using dev config to start `jupiter`, not the 
`jupiter-dev` node.
```bash
jupiter --dev --execution=NativeElseWasm -d <database path for you>
```

##### 4.1.2 start local test mode jupiter blockchain
Local test is a mode which is used to start a private blockchain network.
```bash
# start first node
jupiter --chain=local --alice --execution=NativeElseWasm -d <database path for you>
# start second node
jupiter --chain=local --bob --execution=NativeElseWasm -d <database path for you>
```
Note if you start those two nodes in a machine, there could connect to each other directly. If you start in different 
but in same network, you should add `--bootnodes` parameter when start second node. More details please lookup [https://substrate.dev/](https://substrate.dev/)

#### 4.2 jupiter-para
`jupiter-para` use for join parachain collector.

// TODO

#### 4.3 jupiter-dev
`jupiter-dev` just use for local develop, it provider a nice and quick experience for contract developing for developer 
do not need wait for producing block time, which is very wasting time in testing.

run jupiter-dev node in your machine:
```bash
./target/release/jupiter-dev --dev -d .sub --execution=Native
# or (notice if you have not set `WASM_BUILD_TYPE=release` in environment variables, put this before `cargo run`)
cargo run --release --bin jupiter-dev -- --dev -d .sub --execution=Native
```

### 5. https://polkadot.js.org/apps/ or https://github.com/polkadot-js/apps or using for `polkadot-js` sdk
If developer try to use `apps` to operate jupiter, he should add Extending types for `apps` to parse chain type. The Extending types [see above](#Jupiter-Extending-types)

For apps, developer or user could do following things:
1. Click left top and choose `DEVELOPMENT`, `LOCAL NODE`, and click `Swich` on top.
 
    1. if you run you node on your machine, you just choose `Local Node`, which is `127.0.0.1:9944` for you websocket 
    address for your node (or other ip:port for you node).
    2. if you want to connect "jupiter testnet", you should choose "custom endpoint" and put the following address
        ````
        wss://jupiter.elara.patract.io/
        ````
        All developer could connect to public test network directly.
    
2. Click `Settings` tab and click `Developer` in this tab view, parse the above json into it, and click `Save`.
3. Refresh website and you could use `apps` to do transfer and other basic operation.
