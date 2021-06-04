# Jupiter
Wasm smart contract networks powered by Substrate `FRAME Contracts pallet` in Polkadot ecosystem.

Jupiter aims to be an open blockchain network, which supports contract developers to use [ink!](https://github.com/paritytech/ink), [Ask!](https://github.com/patractlabs/ask) and other language frameworks which can run on `FRAME Contracts pallet` contract model. We will directly use DOT(test coin)/ROC(rococo coin) as native token for gas fee and governance.

For different requirements, this repo contains different runtime for different situations (The content in following parentheses is the name of the executable file):

* Jupiter Testnets (already running):
    * Jupiter PoA (jupiter-prep): The independent blockchain maintained by PoA validators is only for deploying test contracts and issuing test assets with no value. This blockchain network contains newest features.
    * Jupiter Rococo (jupiter): Almost the same position of Jupiter PoA, excepted the consensus algorithm switched from PoA to Rococo shared security, and can simulate cross-chain messaging functions.

* Development Node:
  
    * Jupiter Dev Node (jupiter-dev): Local development node with pure `FRAME contracts pallet`. Contract developers could test contracts quickly **without waiting block interval**.

> Currently, Polkadot & Kusama have not released the parachain feature, thus we use branch `rococo-v1` to track the newest Rococo.
> And the executable file `jupiter` just could be compiled in `rococo-v1` branch. `master` branch does not provide `jupiter` now.

*In following, we name `FRAME contracts pallet` as `pallet-contracts`*

## Patract's `FRAME contracts pallet` (We will name it as `pallet-patracts` in future)

Patract `FRAME contracts pallet` contains our `ChainExtension` to provide many particular features for contracts, and contains some compatible modifications.

* ChainExtension
    - [ ] Contract Logger support
    - [x] Zero-Knowledge support, refer to this link [PIP-101](https://github.com/patractlabs/PIPs/blob/main/PIPs/pip-101.md)
        
        *Currently we use a simple static way to charge weight for ZKP, we would change this part with benchmarks result in future.*
    
* Patract modifications (`pallet-patracts`)
    * // No features for now.
    
Thus we call "`src pallet-contract`" for the `pallet-contract` module which comes from substrate directly, and call "`modified pallet-contracts`" for 
the `pallet-contract` which comes from our forked substrate repo in `vendor` directory.

Inside:
* `src pallet-contract`: support Patract `ChainExtension`
* `modified pallet-contract`: support Patract `ChainExtension` and Patract modifications

## Jupiter living network
When using [Substrate Portal](https://polkadot.js.org/apps), [@polkadot/api](https://github.com/polkadot-js/api) and [Redspot](https://github.com/patractlabs/redspot)
or other 3rd parties client to connect Jupiter node, please remember to add ["extending types"](https://polkadot.js.org/docs/api/start/types.extend/) for Jupiter requirements.
Notice different network may have different Extending types.

Now, Jupiter has launched following network:

* Jupiter PoA V1 (Jupiter A1):

    This network uses `modified pallet-contract` now.

    It's an independent blockchain network which contains contracts feature. Anyone could submit
    contract to this network for test. The token decimals and time interval for producing block are
    same with Polkadot network. This testnet blockchain uses Babe as consensus algorithm,
    and **provide Babe VRF random number for contract module.**
  
    **Jupiter PoA's SS58 Address Prefix changes from 42 (address: `5xxxx`) to 26 (address: `3xxxx`) in 1.0.1 version.**
  
    For this network, the Extending types is:

    ```json
    {
      "LookupSource": "MultiAddress",
      "Address": "MultiAddress",
      "AccountInfo": "AccountInfoWithTripleRefCount",
      "AliveContractInfo": {
        "trieId": "TrieId",
        "storageSize": "u32",
        "pairCount": "u32",
        "codeHash": "CodeHash",
        "rentAllowance": "Balance",
        "rentPayed": "Balance",
        "deductBlock": "BlockNumber",
        "lastWrite": "Option<BlockNumber>",
        "_reserved": "Option<Null>"
      },
      "FullIdentification": "AccountId",
      "AuthorityState": {
        "_enum": [
          "Working",
          "Waiting"
        ]
      },
      "EraIndex": "u32",
      "ActiveEraInfo": {
        "index": "EraIndex",
        "start": "Option<u64>"
      },
      "UnappliedSlash": {
        "validator": "AccountId",
        "reporters": "Vec<AccountId>"
      }
    }
    ```
    
    **Jupiter PoA open provider links are:**
    
    - `wss://jupiter-poa.elara.patract.io/`
    - `wss://ws.jupiter-poa.patract.cn/`
    
    Jupiter open telemetry link is [https://telemetry.patract.io/](https://telemetry.patract.io/)

* Jupiter Rococo:

    This part of code in `rococo-v1` branch now. Please checkout `rococo-v1` and compile it.

    It's a parachain blockchain network which would be deployed to Rococo network. And in this parachain, we would try 
    to use the random number which relayed from relay chain in future.
  
    This network uses `modified pallet-contract` now.

    For this network, the Extending types is:

    ```json
    {
      "LookupSource": "MultiAddress",
      "Address": "MultiAddress"
    }
    ```

    **Jupiter Rococo open provider links are:**
      - `wss://jupiter.elara.patract.io/`
      - `wss://ws.jupiter.patract.cn/`

* Jupiter Dev node: 
    
    It's a local develop node for testing contract easily. This node could run in single, and **do not have time interval
    for producing block**, which would produce block only when receiving an extrinsic. This feature let developers do not 
    need to waste time for waiting producing blocks in develop and test.

    This node uses `modified pallet-contract` now.

    For this network, the Extending types is:

    ```json
    {
      "LookupSource": "MultiAddress",
      "Address": "MultiAddress"
    }
    ```

## Substrate newest master and Substrate v2.0.0 version
Note: Substrate newest master is very different with Substrate v2.0.0 version. In Jupiter `master` branch, we use Substrate newest
master as dependencies, and in Jupiter `substrate-v2.0.0` branch, we use Substrate v2.0.0 from "crates.io" as dependencies. 
(`substrate-v2.0.0` branch only contains jupiter part.)

Thus, Jupiter has two main branch. In different branch, we provide different `pallet-contracts` features:
* master: we track newest master, currently we use substrate commit:[contracts: Release as v3.0.0 and add reserved field to `ContractInfoOf` (#8175)(`debec916`)](https://github.com/paritytech/substrate/commit/debec916998233a287fb9e5a099c08d5e4a23db2).
    * In master, we use `src pallet-contract` or `modified pallet-contract`
    * `src pallet-contract` is belong to substrate commit version.
    * `modified pallet-contract` is belong to the forked substrate in vendor directory. We usually track the newest substrate version.
* [substrate-v2.0.0](https://github.com/patractlabs/patract/tree/substrate-v2.0.0): we keep v2.0.0 dependencies in this branch. **But this branch could only provide version 2.0.0 `pallet-contracts`.**
    * Version 2.0.0 may not match for newest ink!. If you what to test contract in this branch, please peek a suitable ink!
      version for v2.0.0 `pallet-contracts`.
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

When running node, following parameters means:

* `--dev`: means start with dev mode, would provide default private key to start the chain, and running a independent node.
* `-d <directory>`: means blockchain database stored in the `<directory>`, if you what to clear all data, just stop the
node and delete this directory. If you do not use this parameter, the node would use default directory to store data.
* `--execution=<STRATEGY>`: substrate provide `Native` and `WASM` and other execution strategy. for test and develop, 
we suggest to use `Native`  

More parameters please lookup from `--help` or Substrate Developer Hub [https://substrate.dev/](https://substrate.dev/)

#### 4.1 Jupiter PoA
The blockchain produces blocks in `babe` algorithm for every **6 second**, and do finality by `grandpa` algorithm. 
All people could join this testnet as a sync node. 

**Welcome use this link [https://patrastore.io/](https://patrastore.io/) to claim some DOT from our faucet**

##### 4.1.1 join Jupiter PoA testnet
Moving the execution file `jupiter-prep` to other place and launch the node by following steps:
```bash
./jupiter-prep --chain=jupiter-poa --name=<set you custom name> --pruning=archive --execution=NativeElseWasm
```
Those steps is same to any other substrate node, just should notice this parameter `--chain` should use `jupiter`.

You could lookup your node in [https://telemetry.patract.io/](https://telemetry.patract.io/).

##### 4.1.2 start dev mode Jupiter PoA blockchain
Notice this dev mode just means Jupiter PoA start with `--dev`, which is using dev config to start Jupiter PoA, not the 
`jupiter-dev` node.
```bash
./jupiter-prep --dev --execution=NativeElseWasm -d <database path for you>
```

##### 4.1.2 start local test mode Jupiter PoA blockchain
Local test is a mode which is used to start a private blockchain network.
```bash
# start first node
./jupiter-prep --chain=jupiter-poa-local --alice --execution=NativeElseWasm -d <database path for you>
# start second node
./jupiter-prep --chain=jupiter-poa-local --bob --execution=NativeElseWasm -d <database path for you>
```
Note if you start those two nodes in a machine, there could connect to each other directly. If you start in different 
but in same network, you should add `--bootnodes` parameter when start second node. More details please lookup [https://substrate.dev/](https://substrate.dev/)

#### 4.2 Jupiter Rococo (in `rococo-v1` branch)
`jupiter` use for join parachain collector.
```bash
./jupiter --parachain-id=1000 --collator -d .sub -- --chain=rococo.json
```

#### 4.3 Jupiter Dev Node
`jupiter-dev` just use for local development, it providers a nice and quick experience for contract developing for developers 
do not need wait for producing block time, which is very wasting time in testing.

running jupiter-dev node in your machine:
```bash
./jupiter-dev --dev -d .sub --execution=Native
```

### 5. https://polkadot.js.org/apps/ or https://github.com/polkadot-js/apps or using for `polkadot-js` sdk
Now, Jupiter has provided in `apps`. And we would provide Jupiter Rococo in `TEST RELAYS & PARACHAINS` soon.
