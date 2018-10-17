extern crate ens;
extern crate web3;

use web3::types::Address;
use web3::futures::Future;
use ens::ENS;
use std::env;

fn main() {
    let ens_addr = env::vars().filter(|(k, _)| k == "ENS_ADDR" ).next().map(|(_, v)| v);
    let ens_root = env::vars().filter(|(k, _)| k == "ENS_ROOT" ).next().map(|(_, v)| v).unwrap_or(String::from("eth"));
    let mut args = env::args();
    let ens_name = args.nth(1).expect("ENS name CLI arg");
    let new_addr = args.next().expect("address CLI arg").parse().unwrap();

    let (_evloop, transport) = web3::transports::Http::new(
        "http://localhost:8545",
    ).unwrap();

    let ens = match ens_addr {
        Some(addr) => ENS::with_ens_addr(web3::Web3::new(transport), addr.parse().unwrap()),
        None => ENS::new(web3::Web3::new(transport)),
    };
    let old_addr = ens.address(&ens_root, &ens_name).wait().unwrap_or(Address::new());

    println!("name      : {}", ens_name);
    println!("old addr  : {:?}", old_addr);

    ens.set_address(&ens_root, &ens_name, new_addr).wait().expect("set_address");

    println!("new addr  : {:?}", new_addr);
}
