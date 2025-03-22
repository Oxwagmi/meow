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

build the binary manually:

```sh
cargo build -r
```
The compiled binary will be located in `target/release/meow`.

**Recommended** creating a new solana and a evm wallet for testing , 


```sh

 cast wallet vanity  --starts-with babe         
 solana-keygen grind --starts-with "Babe:1"  

 ```

create the env as the example 

---


## Usage


```sh
 ./target/release/meow bridge --to-chain base  --to 0x000destination_address  --amount 1                                                 
      // note: amount  1 = 0.000001 USDC  cuz USDC token program has 6 decimal points on solana chain             
```
```sh
 ./target/release/meow bridge --mainnnet true --to-chain base  --to 0x000destination_address  --amount 1                                                 
  // note:  default to testnet if u wanna use mainnet use -m or --mainnnet true         
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

