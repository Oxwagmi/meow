# `Meow`

 **A CLI tool for transfering USDC from Solana to other evm chains via Circle’s CCTP.**


---

##  Overview
![meooow](meow.jpeg )

`meow` is a command-line tool that enables seamless **USDC transfers** from **Solana** to **other evm chains** using Circle’s Cross-Chain Transfer Protocol (CCTP).

This tool interacts directly with the Solana blockchain and Circle’s CCTP smart contracts, ensuring efficient cross-chain USDC transfers.



---

##  Features

- **Transfer USDC** from **Solana** to supported **EVM chains** (Ethereum, Unichain, etc.).
- **Cross-chain execution** powered by **Circle CCTP**.Bridge USDC across different chains with zero slippage and fees.
- **Lightweight & fast**, built in Rust for performance.
- **Non-custodial**, you control your private keys.

---

## 🛠 Installation

Make sure you have **Rust** installed:

#### Quick installation

```sh

cargo install --git https://github.com/Oxwagmi/meow.git --tag v0.1.0  
cd urfolder 
```
create a .env  like env.example 
then run

```sh 
meow --help                                                                                                                                           .env file loaded successfully
Usage: meow <COMMAND>
```


#### Mannual installation
build the binary manually:

git clone https://github.com/Oxwagmi/meow.git

```sh
cargo build -r
```
The compiled binary will be located in `target/release/meow`.

**Recommended** creating a new solana and a evm wallet for testing , 
<br/>

Example :

```sh

 cast wallet vanity  --starts-with babe         
 solana-keygen grind --starts-with "Babe:1"  
 solana airdrop 0.1  Babe_address

```

- get some test usdc on solana  -> https://faucet.circle.com/
- get some test native tokens on destination_chain  -> https://www.alchemy.com/faucets/
- create the env as the example 

---


## Usage


```js
Usage: meow <COMMAND>

Commands:
  bridge-solana-usdc   
  bridge-evm-usdc      
  mannual-redeem-usdc  
  help                 Print this message or the help of the given subcommand(s)

```
###### Solana => Evm

```sh
 ./target/release/meow bridge-solana-usdc --to-chain base  --to 0x000destination_address  --amount 1                                                 
      // note: amount  1 = 0.000001 USDC  cuz USDC token program has 6 decimal points on solana chain             
```

```sh
 ./target/release/meow bridge-solana-usdc --safe-format-usdc --to-chain base  --to 0x000destination_address  --amount 1                                               
// note: with the --safe-format-usdc command the --amount will be converted to correct decimal points and default max transfer to 100 USDC for safety. src/programs.rs line 144        
```

```sh
 ./target/release/meow bridge-solana-usdc --mainnnet  --to-chain base  --to 0x000destination_address --amount 1                                                 
  // note:  default to testnet if u wanna use mainnet use -m or --mainnnet         
```
```sh
./target/release/meow mannual-redeem-usdc --txn-hash 0xevm_txn_hash --remote-domain 10 --retry-secs 100 --remote-usdc 0x31d0220469e10c4E71834a79b1f276d740d3768F

// eg: for unichain remote-domain is 10 in unichain testnet usdc address is 0x3.. 

```

######   Evm => Solana

```sh
 ./target/release/meow bridge-evm-usdc  --from-chain unichain   --amount 1  --retry-secs 300   
   
   // amount 1 means 1/10**6 usdc  
  // note:  default to testnet if u wanna use mainnet use -m or --mainnnet         
```



### Flags & Options
| Flag | Description |
|------|------------|
| `--amount` | Amount of USDC to transfer. |
| `--to-chain` | Destination chain (e.g., `unichain`, `base`). |
| `--to` | The recipient’s wallet address on the target chain. |
| `--mainnet` |  mainnet bool true or false|


---
## ⚠ Disclaimer

> [!WARNING]  
>This is a **personal project**, provided  without any guarantees. **Use at your own risk!** Always read the code before usage.


---

