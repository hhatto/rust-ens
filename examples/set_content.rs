extern crate ens;
extern crate web3;

use web3::types::H256;
use web3::futures::Future;
use ens::ENS;
use std::env;

fn main() {
    let ens_addr = env::vars().filter(|(k, _)| k == "ENS_ADDR" ).next().map(|(_, v)| v);
    let ens_root = env::vars().filter(|(k, _)| k == "ENS_ROOT" ).next().map(|(_, v)| v).unwrap_or(String::from("eth"));
    let mut args = env::args();
    let ens_name = args.nth(1).expect("ENS name CLI arg");
    let new_content = args.next().expect("content CLI arg").parse().unwrap();

    let (_evloop, transport) = web3::transports::Http::new(
        "http://localhost:8545",
    ).unwrap();

    let ens = match ens_addr {
        Some(addr) => ENS::with_ens_addr(web3::Web3::new(transport), addr.parse().unwrap()),
        None => ENS::new(web3::Web3::new(transport)),
    };
    let old_content = ens.content(&ens_root, &ens_name).wait().unwrap_or(H256::new());

    println!("name       : {}", ens_name);
    println!("old content: {:?}", old_content);

    ens.set_content(&ens_root, &ens_name, new_content).wait().expect("set_content");
 
    println!("new content: {:?}", new_content);
}
