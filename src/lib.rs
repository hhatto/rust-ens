#[macro_use]
extern crate lazy_static;
extern crate web3;
extern crate tiny_keccak;

use web3::contract::{Contract, Options};
use web3::types::{Address, H256};
use web3::futures::Future;
use tiny_keccak::Keccak;

const ENS_MAINNET_ADDR: &str = "314159265dD8dbb310642f98f50C066173C1259b";
const ENS_REVERSE_REGISTRAR_DOMAIN: &str = "addr.reverse";

struct EnsSetting {
    mainnet_addr: Address,
}

lazy_static! {
    static ref ENS_SETTING: EnsSetting = EnsSetting {
        mainnet_addr: ENS_MAINNET_ADDR.parse().expect("don't parse ens.mainnet.addr")
    };
}

struct Resolver {
    name: String
}

impl Resolver {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }

    fn get(self, name: &str) {
    }
}

//fn resolve(name: &str) -> Vec<u7> {
//    let resolver = Resolver::new(name);
//    let nhash = namehash(name);
//    let addr = resolver.get(nhash);
//    addr
//}

//fn address(name: &str) -> Vec<u7> {
//    resolve(name)
//}

#[derive(Debug, Clone)]
pub struct ENS<T: web3::Transport> {
    web3: web3::Web3<T>,
}

impl<T: web3::Transport> ENS<T> {

    pub fn new(web3: web3::Web3<T>) -> Self {
        ENS {
            web3: web3,
        }
    }

    pub fn name(&self, address: Address) -> Result<String, String> {
        let contract = Contract::from_json(
            self.web3.eth(),
            ENS_SETTING.mainnet_addr,
            include_bytes!("../contract/ENS.abi"),
        ).expect("fail contract::from_json(ENS.abi)");
        let addr = H256::from_slice(namehash(format!("{:x}.{}", address, ENS_REVERSE_REGISTRAR_DOMAIN).as_str()).as_slice());
        let result = contract.query("resolver", (addr, ), None, Options::default(), None);
        let resolver_addr: Address = result.wait().expect("resolver.result.wait()");

        // resolve
        let resolver_contract = Contract::from_json(
            self.web3.eth(),
            resolver_addr,
            include_bytes!("../contract/PublicResolver.abi"),
        ).expect("fail load resolver contract");

        //let addr = H256::from_slice(namehash(format!("{:x}", address).as_str()).as_slice());
        let result = resolver_contract.query("name", (addr, ), None, Options::default(), None);
        match result.wait() {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("error: name.result.wait(): {:?}", e)),
        }
    }

    pub fn owner(&self, name: &str) -> Result<Address, String> {
        let contract = Contract::from_json(
            self.web3.eth(),
            ENS_SETTING.mainnet_addr,
            include_bytes!("../contract/ENS.abi"),
        ).expect("fail contract::from_json(ENS.abi)");
        let ens_namehash = H256::from_slice(namehash(name).as_slice());
        let result = contract.query("owner", (ens_namehash, ), None, Options::default(), None);
        match result.wait() {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("error: owner.result.wait(): {:?}", e)),
        }
    }

    pub fn address(&self, name: &str) -> Result<Address, String> {
        let contract = Contract::from_json(
            self.web3.eth(),
            ENS_SETTING.mainnet_addr,
            include_bytes!("../contract/ENS.abi"),
        ).expect("fail contract::from_json(ENS.abi)");
        let ens_namehash = H256::from_slice(namehash(name).as_slice());

        let result = contract.query("resolver", (ens_namehash, ), None, Options::default(), None);
        let resolver_addr: Address = result.wait().expect("resolver.result.wait()");

        // resolve
        let resolver_contract = Contract::from_json(
            self.web3.eth(),
            resolver_addr,
            include_bytes!("../contract/PublicResolver.abi"),
        ).expect("fail load resolver contract");

        let result = resolver_contract.query("addr", (ens_namehash, ), None, Options::default(), None);
        match result.wait() {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("error: addr.result.wait(): {:?}", e)),
        }
    }
}

fn namehash(name: &str) -> Vec<u8> {
    let mut node = vec![0u8; 32];
    if name.is_empty() {
        return node;
    }
    let mut labels: Vec<&str> = name.split(".").collect();
    labels.reverse();
    for label in labels.iter() {
        let mut labelhash = [0u8; 32];
        Keccak::keccak256(label.as_bytes(), &mut labelhash);
        node.append(&mut labelhash.to_vec());
        labelhash = [0u8; 32];
        Keccak::keccak256(node.as_slice(), &mut labelhash);
        node = labelhash.to_vec();
    }
    node
}

#[cfg(test)]
mod test {
    use super::namehash;
    use web3::types::Address;

    #[test]
    fn test_namehash() {
        let addresses = vec![
            ("", "0x0000000000000000000000000000000000000000"),
            ("eth", "0x93cdeb708b7545dc668eb9280176169d1c33cfd8"),
            ("foo.eth", "0xde9b09fd7c5f901e23a3f19fecc54828e9c84853"),
        ];
        for (name, address) in addresses {
            let hash_address = Address::from_slice(namehash(name).as_slice());
            let h = format!("{:?}", hash_address);
            assert_eq!(address.to_string(), h);
        }
    }
}
