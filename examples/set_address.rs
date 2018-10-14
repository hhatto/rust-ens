extern crate ens;
extern crate web3;

use web3::types::Address;
use web3::futures::Future;
use ens::ENS;


fn main() {
    let (_evloop, transport) = web3::transports::Http::new(
        "http://localhost:8545",
    ).unwrap();

    let ens_name = "hexacosa.eth";

    let ens = ENS::new(web3::Web3::new(transport));
    let addr = {
        match ens.address(ens_name).wait() {
            Ok(addr) => addr,
            Err(_) => Address::new(),
        }
    };

    println!("name      : {}", ens_name);
    println!("addr      : {:?}", addr);

    ens.set_address(ens_name, "314159265dD8dbb310642f98f50C066173C1259b".parse().unwrap()).wait().expect("set_address");
}
