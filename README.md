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
    
## compile and run
### 1. clone this repo
```bash
> git clone https://github.com/patractlabs/jupiter.git
```

### 2. init building environment
*note: if you have build substrate successfully, you do not need to do this any more*

you should prepare rust environment before, lookup [this link for rust](https://www.rust-lang.org/learn/get-started) to
downland rust and install it.
 
then:
```bash
> cd jupiter/scripts
> bash init.sh
```

### 3. compile this repo
```bash
> WASM_BUILD_TYPE=release cargo build --release
```
or we advice you to add `WASM_BUILD_TYPE=release` in your global environment variables

### 4. run
now the directory `target/release` would contain two executable files.
* `jupiter`, which is Jupiter Testnet node
* `jupiter-dev`, which is Jupiter local develop node

following parameters means:
* `--dev`: means start with dev mode, would provide default private key to start the chain, and running a independent node.
* `-d <directory>`: means blockchain database stored in the `<directory>`, if you what to clear all data, just stop the
node and delete this directory. If you do not use this parameter, the node would use default directory to store data.
* `--execution=<STRATEGY>`: substrate provide `Native` and `WASM` execution strategy. for test and develop, we suggest to
use `Native`  


1. `jupiter` 
    ```
    TODO
    ```
2. `jupiter-dev`
    ```bash
    ./target/release/jupiter-dev --dev -d .sub --execution=Native
    # or (notice if you have not set `WASM_BUILD_TYPE=release` in environment variables, put this before `cargo run`)
    cargo run --release --bin jupiter-dev -- --dev -d .sub --execution=Native
    ```

### 4. https://polkadot.js.org/apps/ or https://github.com/polkadot-js/apps
If developer try to use `apps` to operate jupiter, he should add those types for `apps` to parse chain type.
```json
{
  "Address": "AccountId",
  "LookupSource": "AccountId"
}
```
1. Click left top and choose `DEVELOPMENT`, `LOCAL NODE`, and click `Swich` on top.(optional, if you run you node on your machine. We would launch a public jupiter network later, so that
all developer could connect to public test network directly).
2. Click `Settings` tab and click `Developer` in this tab view, parse the above json into it, and click `Save`.
3. Refresh website and you could use `apps` to do transfer and other basic operation.
4. NOTE: current `apps` do not suit to substrate-rc6 version(current jupiter substrate version.), thus could not use `apps` to operate contracts.
 
