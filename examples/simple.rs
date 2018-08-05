extern crate ens;
extern crate web3;

use web3::types::Address;
use ens::ENS;


fn main() {
    let (_evloop, transport) = web3::transports::Http::new(
        "http://localhost:8545",
    ).unwrap();

    let ens_name = "hexacosa.eth";

    let ens = ENS::new(web3::Web3::new(transport));
    let addr = {
        match ens.address(ens_name) {
            Ok(addr) => addr,
            Err(_) => Address::new(),
        }
    };
    let owner_addr = {
        ens.owner(ens_name).expect("ens.owner() error")
    };
    let reverse_addr = match ens.name(addr) {
        Ok(name) => name,
        Err(_) => "unknown".to_string(),
    };

    println!("name      : {}", ens_name);
    println!("owner_addr: {:?}", owner_addr);
    println!("addr      : {:?}", addr);
    println!("rev_addr  : {}", reverse_addr);
}
