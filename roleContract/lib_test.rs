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
mod roleContract {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_core::storage;
    use ink_core::storage::BTreeMap;
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    // use ink_prelude::collections::BTreeMap as BTreeMap;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    struct RoleContract {
        /// Stores a single `bool` value on the storage.
        test_hashmap: storage::hash_map::HashMap<AccountId, u64>,
        test_btreemap: storage::BTreeMap<AccountId, (String, u32)>,
        val: storage::Value<String>,
        test_vec: storage::Value<Vec<u32>>,
    }

    #[ink(event)]
    struct EventAddRole {
        #[ink(topic)]
        func: String,
        #[ink(topic)]
        permission: u32,
    }

    #[ink(event)]
    struct EventGetRole {
        #[ink(topic)]
        func: String,
        #[ink(topic)]
        permission: u32,
    }

    #[ink(event)]
    struct TestEvent {
        #[ink(topic)]
        isOk: u32,
    }

    #[ink(event)]
    struct TestStrEvent {
        #[ink(topic)]
        value: String,
    }

    impl RoleContract {
        /// Constructor that initializes the caller as roleType = 1 (System)
        #[ink(constructor)]
        fn newRoleContract(&mut self) {}

        /// Test Vec input
        #[ink(message)]
        fn testVec(&mut self, array: &[u32]) {
            let mut list: Vec<u32> = Vec::new();
            list.extend_from_slice(array);
            self.test_vec.set(list);
        }

        /// Get test_vec
        #[ink(message)]
        fn getTestVec(&self) -> Vec<u32> {
            let mut list: Vec<u32> = Vec::new();
            for ele in self.test_vec.get().iter() {
                list.push(*ele);
            }

            list
        }

        // /// Add role
        // #[ink(message)]
        // fn addRole(&mut self, addr: AccountId, func_name: &str, permission: u32) {
        //     let name = &func_name.to_string();
        //     // self.test_btreemap.insert(addr, (name.clone(), permission));

        //     // self.env()
        //     //     .emit_event(
        //     //         EventAddRole {
        //     //             func: func_name.to_string(),
        //     //             permission: permission,
        //     //         }
        //     //     );
        // }

        /// Add string value
        #[ink(message)]
        fn addVal(&mut self, val: String) {
            self.val.set(val.clone());

            self.env()
                .emit_event(
                    TestStrEvent {
                        value: val.clone(),
                    }
                );
        }
        //problem: didn't add val successfully

        /// Get string value
        #[ink(message)]
        fn getVal(&self) -> String {
            let value = &*self.val.get();
            value.clone()
        }
        //problem: always return 0x3c5b6f626a656374204f626a6563745d

        /// Get role permission
        #[ink(message)]
        fn getRole(&self, addr: AccountId) -> (String, u32) {
            let mut func_name;
            let mut permissioned = 0u32;
            let roleMap = self.test_btreemap.get(&addr);
            match roleMap {
                Some(_) => {
                    let (ref name, ref p) = roleMap.unwrap();
                    func_name = name.clone();
                    permissioned = *p;
                }
                None => {
                    let empty_str = String::from("");
                    func_name = empty_str.clone();
                    permissioned = 0u32;
                }
            };

            self.env()
                .emit_event(
                    EventGetRole {
                        func: func_name.clone(),
                        permission: permissioned,
                    }
                );

            (func_name.clone(), permissioned)
        }

        /// Add caller type
        #[ink(message)]
    fn setCallerType(&mut self) {
        let bytes: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let acc = AccountId::from(bytes);
        let default_value:u64 = 1;
        // self.test_hashmap.insert(acc, default_value);
        // self.test_btreemap.insert(acc, default_value);

        self.env()
            .emit_event(
                TestEvent {
                    isOk: 1,
                }
            );
        }
    }
}