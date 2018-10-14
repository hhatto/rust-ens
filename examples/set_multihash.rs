extern crate ens;
extern crate web3;

use web3::futures::Future;
use ens::ENS;
use std::env;

fn main() {
    let ens_addr = env::vars().filter(|(k, _)| k == "ENS_ADDR" ).next().map(|(_, v)| v);
    let ens_root = env::vars().filter(|(k, _)| k == "ENS_ROOT" ).next().map(|(_, v)| v).unwrap_or(String::from("eth"));
    let mut args = env::args();
    let ens_name = args.nth(1).expect("ENS name CLI arg");
    let new_multihash = args.next().expect("multihash CLI arg").as_bytes().to_vec();

    let (_evloop, transport) = web3::transports::Http::new(
        "http://localhost:8545",
    ).unwrap();

    let ens = match ens_addr {
        Some(addr) => ENS::with_ens_addr(web3::Web3::new(transport), addr.parse().unwrap()),
        None => ENS::new(web3::Web3::new(transport)),
    };
    let old_multihash = ens.multihash(&ens_root, &ens_name).wait().unwrap_or(Vec::new());

    println!("name         : {}", ens_name);
    println!("old multihash: {:?}", old_multihash);

    ens.set_multihash(&ens_root, &ens_name, new_multihash.clone()).wait().expect("set_multihash");
 
    println!("new multihash: {:?}", new_multihash);
}
