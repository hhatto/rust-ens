extern crate ens;
extern crate web3;

use web3::types::{Address, H256};
use web3::futures::Future;
use ens::ENS;
use std::env;

fn main() {
    let ens_addr = env::vars().filter(|(k, _)| k == "ENS_ADDR" ).next().map(|(_, v)| v);
    let ens_root = env::vars().filter(|(k, _)| k == "ENS_ROOT" ).next().map(|(_, v)| v).unwrap_or(String::from("eth"));
    let mut args = env::args();
    let ens_name = args.nth(1).unwrap_or(String::from("eth"));

    let (_evloop, transport) = web3::transports::Http::new(
        "http://localhost:8545",
    ).unwrap();

    let ens = match ens_addr {
        Some(addr) => ENS::with_ens_addr(web3::Web3::new(transport), addr.parse().unwrap()),
        None => ENS::new(web3::Web3::new(transport)),
    };
    let addr = ens.address(&ens_root, &ens_name).wait().unwrap_or(Address::new());
    let content = ens.content(&ens_root, &ens_name).wait().unwrap_or(H256::new());
    let multihash = ens.multihash(&ens_root, &ens_name).wait().unwrap_or(Vec::new());
    let owner_addr = ens.owner(&ens_root).wait().expect("ens.owner() error");
    let reverse_addr = ens.name(addr).wait().unwrap_or("unknown".to_string());

    println!("name      : {}", ens_name);
    println!("owner_addr: {:?}", owner_addr);
    println!("addr      : {:?}", addr);
    println!("rev_addr  : {}", reverse_addr);
    println!("content   : {:?}", content);
    println!("multihash : {:?}", multihash);
}
