/* 
    Using the standard library if we run the tests module, 
    or if we use a std feature flag within our code. 
    Otherwise the contract will always compile with no_std.
*/
#![cfg_attr(not(feature = "std"), no_std)]

extern crate hex;

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
mod coreContract {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_core::storage;
    use ink_core::storage::BTreeMap;
    use ink_core::storage::HashMap;
    // use ink_core::storage::Vec;
    use ink_core::storage::Stash;
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use ink_primitives::Key;
    use ink_core::storage::stash::Values;
    use bigdecimal::{BigDecimal, Zero, One};

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    struct CoreContract {
        /// Stores a single `bool` value on the storage.
        // test_stash: Stash<(u128, u32, bool)>,
        // test_hashmap_stash: HashMap<u128, Vec<(u128, u32, AccountId)>>,
        settle_items: storage::Vec<(AccountId, u32, String)>,
        // bigdecimal: storage::Value<BigDecimal>,
    }

    #[ink(event)]
    struct TestVecEvent {
        #[ink(topic)]
        items: Vec<(AccountId, u32, String)>,
    }

    impl CoreContract {
        /// Constructor that initializes the caller as roleType = 1 (System)
        #[ink(constructor)]
        fn new(&mut self) {}

        #[ink(message)]
        fn addItem(&mut self, sub_mwal: AccountId, class: u32, amount: String){
            self.settle_items.push((sub_mwal, class, amount));
        }

        #[ink(message)]
        fn getItems(&mut self) -> Vec<(AccountId, u32, String)> {
            let arr = self.settle_items.iter();
            let mut vec = Vec::new();
            for c in arr {
                let sub_mwal = c.0;
                let class = c.1;
                let amount = c.2.clone();
                // let (sub_mwal, class, amount) = *c;
                vec.push((sub_mwal, class, amount));
            }

            vec
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // #[test]
        // fn test_default() {
        //     let mut coreContract = CoreContract::new();
        //     let contract = coreContract.getContractAddr();
        //     println!("Contract address is: {:?}", contract);
        //     let id_0 = coreContract.addStash(2008, 0);
        //     let id_1 = coreContract.addStash(2009, 1);
        //     let id_2 = coreContract.addStash(2010, 2);
        //     let id_3 = coreContract.addStash(2010, 2);

        //     println!("id_0: {x:?}, id_1: {y:?}, id_2: {z:?}", x=id_0, y=id_1, z=id_2);

        //     // let (func_0, stage_0, isOk_0) = coreContract.takeStash(id_0);
        //     // println!("func_0: {x:?}, stage_0: {y:?}, isOk_0: {z:?}", x=func_0, y=stage_0, z=isOk_0);

        //     let (func_0, stage_0, isOk_0) = coreContract.getStash(id_0);
        //     println!("func_0: {x:?}, stage_0: {y:?}, isOk_0: {z:?}", x=func_0, y=stage_0, z=isOk_0);

        //     let key = coreContract.getKey();
        //     println!("key: {:?}", key);

        //     let len = coreContract.getLen();
        //     println!("len: {:?}", len);

        //     let arr = coreContract.loopOver();
        //     for v in arr.iter() {
        //         println!("v: {:?}", v);
        //     }

        //     let arr_kv = coreContract.loopOverKV();
        //     for (k, v) in arr_kv.iter() {
        //         println!("k: {:?}", k);
        //         println!("v: {:?}", v);
        //     }
        // }
    }
}