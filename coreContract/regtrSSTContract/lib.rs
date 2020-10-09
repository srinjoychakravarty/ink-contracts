#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]

mod regtrSSTContract {
    #[cfg(not(feature = "ink-as-dependency"))]
    use roleContract::RoleContract;
    use ink_core::{
        env::println,
        storage,
    };
    use ink_prelude::vec::Vec;
    use ink_prelude::string::String;
    // use  ink_prelude::string::String;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    struct RegtrSstContract {
        /// Stores a SST list map <id -> coreAddress>
        symbol_addr_map: storage::HashMap<String, AccountId>,
        sstList: storage::Vec<String>,
        roleContract: storage::Value<RoleContract>,
    }

    #[ink(event)]
    struct RegtrSymbEvent {
        #[ink(topic)]
        symbol: String,
    }

    #[ink(event)]
    struct RegtrCoreEvent {
        #[ink(topic)]
        core: AccountId,
    }

    #[ink(event)]
    struct GetAddrEvent {
        #[ink(topic)]
        core: AccountId,
    }

    #[ink(event)]
    struct GetListEvent {
        #[ink(topic)]
        data: Vec<String>,
    }

    #[ink(event)]
    struct RegtrCheckEvent {
        #[ink(topic)]
        res: bool,
    }

    #[ink(event)]
    struct StrEvent {
        #[ink(topic)]
        id: String,
    }

    impl RegtrSstContract {
        /// Constructor that initializes register sst contract.
        #[ink(constructor)]
        fn new(&mut self) {
            let bytes: [u8; 32] = [18, 196, 115, 44, 96, 88, 10, 122, 72, 206, 170, 232, 229, 92, 191, 83, 96, 15, 203, 207, 155, 141, 222, 142, 187, 174, 115, 29, 169, 204, 39, 237];
            let account = AccountId::from(bytes);
            let roleContract = ink_core::env::call::FromAccountId::from_account_id(account);

            self.roleContract.set(roleContract);
        }

        /// register a new SST 
        #[ink(message)]
        fn registerSST(&mut self, id: String, addr: AccountId) -> bool {
            let caller = self.env().caller();
            if self.roleContract.getRoleType(caller) != 1 {
                println("Register failed: you have to be a system account!");
                return false;
            }

            if self.symbol_addr_map.contains_key(&id) {
                println("Register failed: the symbol is already registered!");
                return false;
            }

            match self.symbol_addr_map.get(&id) {
                Some(_) => {
                    *self.symbol_addr_map.get_mut(&id).unwrap() = addr
                }
                None => {
                    self.symbol_addr_map.insert(id.clone(), addr);
                    self.sstList.push(id.clone());
                }
            };

            self.env()
                .emit_event(
                    RegtrSymbEvent {
                        symbol: id,
                    }
                );

            self.env()
                .emit_event(
                    RegtrCoreEvent {
                        core: addr,
                    }
                );
            true
        }

        /// get core address by SST id
        #[ink(message)]
        fn getSSTcoreAddress(&self, id: String) -> AccountId {
            let bytes: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            let empty_addr = AccountId::from(bytes);
            let coreAddr = *self.symbol_addr_map.get(&id).unwrap_or(&empty_addr);
            self.env()
                .emit_event(
                    GetAddrEvent {
                        core: coreAddr,
                    }
                );

            coreAddr
        }

        /// get SST list
        #[ink(message)]
        fn listSSTs(&self) -> Vec<String> {
            let mut list: Vec<String> = Vec::new();
            for id in self.sstList.iter() {
                list.push(id.clone());

                self.env()
                    .emit_event(
                        StrEvent {
                            id: id.clone(),
                        }
                    );
            }

            list
        }

        /// check if the SST is registered
        #[ink(message)]
        fn isRegistered(&self, sstId: String) -> bool {
            let registered = self.symbol_addr_map.contains_key(&sstId);
            self.env()
                .emit_event(
                    RegtrCheckEvent {
                        res: registered,
                    }
                );
            
            registered
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        // /// We test if the default constructor does its job.
        // #[test]
        // fn default_works() {
        //     let mut regtrContract = RegtrSstContract::new();
        //     let bytes: [u8; 32] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        //     let bytes2: [u8; 32] = [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        //     let coreAddr = AccountId::from(bytes);
        //     let coreAddr2 = AccountId::from(bytes2);
        //     let res = regtrContract.registerSST(1, coreAddr);
        //     let res_2 = regtrContract.registerSST(2, coreAddr2);
        //     println!("Register SST success or not: ID 1 -> {x:?}, ID 2 -> {y:?}", x=res, y=res_2);

        //     let res2 = regtrContract.getSSTcoreAddress(2);
        //     println!("Core address is: {:?}", res2);

        //     let res3 = regtrContract.listSSTs();
        //     println!("List all SSTs id: {:?}", res3);
        // }
    }
}

pub use crate::regtrSSTContract::RegtrSstContract;