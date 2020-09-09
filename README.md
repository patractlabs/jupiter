# jupiter
Testnet for Substrate contracts parachain.

Currently Jupiter provide an independent substrate blockchain network and a local dev test node(two type executable file), 
which both contain `pallet-contracts` module to provide contract feature. In the future, the Jupiter would become an
parachain for Rococo and Kusama.  

This repo provide two type node:
* Jupiter Testnet node:
 
    it's an independent blockchain network which contains contracts feature. Anyone could submit 
    contract to this network for test. The token decimals and time interval for producing block are
    same with polkadot network.
    
* Jupiter local develop node: 
    
    It's a local develop node for testing contract easily. This node could run in single, and **do not have time interval
    for producing block**, which would produce block only when receiving an extrinsic. This feature let developers do not 
    need to waste time for waiting producing blocks in develop and test.
    
**Note: current jupiter is dependent on substrate-rc6 version. `pallet-contracts` for rc6 version is still in refactoring, 
and newest ink! do not match for current `pallet-contracts`, some example contracts could not play well in current version.**
    
## compile and run
### 1. clone this repo
```bash
> git clone https://github.com/patractlabs/jupiter.git
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

The blockchain produce blocks in `aura` algorithm for every **6 second**, and do finality by `grandpa` algorithm. Currently
this blockchain is in PoA mode, later we would provide a way to replace the validators in PoA.

##### 4.1.1 join jupiter staging testnet
Moving the execution file `jupiter` to other place and launch the node by following steps:
```bash
jupiter --chain=staging --name=<set you custom name> --pruning=archive --execution=NativeElseWasm
```
Those steps is same to any other substrate node, just should notice this parameter `--chain` should use `staging`

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
   
#### 4.2 jupiter-dev
`jupiter-dev` just use for local develop, it provider a nice and quick experience for contract developing for developer 
do not need wait for producing block time, which is very wasting time in testing.

run jupiter-dev node in your machine:
```bash
./target/release/jupiter-dev --dev -d .sub --execution=Native
# or (notice if you have not set `WASM_BUILD_TYPE=release` in environment variables, put this before `cargo run`)
cargo run --release --bin jupiter-dev -- --dev -d .sub --execution=Native
```

### 5. https://polkadot.js.org/apps/ or https://github.com/polkadot-js/apps or using for `polkadot-js` sdk
If developer try to use `apps` to operate jupiter, he should add those types for `apps` to parse chain type.
```json
{
  "Address": "AccountId",
  "LookupSource": "AccountId"
}
```
Above json also use for types in `polkadot-js` sdk, please refer to [https://polkadot.js.org/api/start/types.extend.html](https://polkadot.js.org/api/start/types.extend.html)

For apps, developer or user could do following things:
1. Click left top and choose `DEVELOPMENT`, `LOCAL NODE`, and click `Swich` on top.
 
    1. if you run you node on your machine, you just choose `Local Node`, which is `127.0.0.1:9944` for you websocket 
    address for your node (or other ip:port for you node).
    2. if you want to connect "jupiter staging testnet", you should choose "custom endpoint" and put the following address
        ````
        wss://ws.patract.io
        ````
        All developer could connect to public test network directly.
    
2. Click `Settings` tab and click `Developer` in this tab view, parse the above json into it, and click `Save`.
3. Refresh website and you could use `apps` to do transfer and other basic operation.
4. NOTE: current `apps` do not suit to substrate-rc6 version(current jupiter substrate version.), thus could not use `apps` to operate contracts.
