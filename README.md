# rust-ens [![](https://travis-ci.org/hhatto/rust-ens.svg?branch=master)](https://travis-ci.org/hhatto/rust-ens) [![Latest Version](https://img.shields.io/crates/v/ens.svg)](https://crates.io/crates/ens)

Rust ENS interface, based on [rust-web3](https://github.com/tomusdrw/rust-web3).

## Usage

Cargo.toml

```toml
[dependencies]
ens = "0.1"
```

```rust
extern crate ens;
extern crate web3;

use ens::ENS;

fn main() {
    let (_evloop, transport) = web3::transports::Http::new(
        "http://localhost:8545",
    ).unwrap();

    let ens_name = "hexacosa.eth";

    let ens = ENS::new(web3::Web3::new(transport));
    match ens.address(ens_name) {
        Ok(addr) => println!("{:?}", addr),
        Err(_) => println!("unknown"),
    };
}
```

## for Developer

### Contract

use ENS Contract from https://github.com/ensdomains/ens/tree/master/contracts .
* ENS.sol
* PublicResolver.sol

```
$ solc --bin --abi --optimize contract/PublicResolver.sol -o contract
$ ls contract
ENS.abi             ENS.bin             ENS.sol             PublicResolver.abi  PublicResolver.bin  PublicResolver.sol
```
