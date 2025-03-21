```bash
cargo tree                                                                                                                                                                              ─╯

    Blocking waiting for file lock on package cache
    Updating crates.io index
   Resolving dependency graph...
error: failed to select a version for `zeroize`.
    ... required by package `rustls v0.23.0`
    ... which satisfies dependency `rustls = "^0.23"` of package `alloy-transport-ws v0.12.6`
    ... which satisfies dependency `alloy-transport-ws = "^0.12"` of package `alloy v0.12.0`
    ... which satisfies dependency `alloy = "^0.12"` of package `meow v0.1.0 (rust_projects/meow)`
versions that meet the requirements `^1.7` are: 1.8.1, 1.7.0

all possible versions conflict with previously selected packages.

  previously selected package `zeroize v1.5.3`
    ... which satisfies dependency `zeroize = "^1"` of package `ark-ec v0.4.2`
    ... which satisfies dependency `ark-ec = "^0.4.0"` of package `solana-bn254 v2.2.1`
    ... which satisfies dependency `solana-bn254 = "^2.2.1"` of package `solana-sdk v2.2.1`
    ... which satisfies dependency `solana-sdk = "^2.2.1"` of package `meow v0.1.0 (rust_projects/meow)`

failed to select a version for `zeroize` which could resolve this conflict
```Dependency conflict between alloyrs and the  solana-sdk 

message-transmitter = { git = "https://github.com/circlefin/solana-cctp-contracts", rev = "4477f889732209dfc9a08b3aeaeb9203a324055c", package = "message-transmitter" }

https://github.com/solana-labs/solana/issues/26688#issuecomment-2411153994