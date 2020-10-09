#![cfg_attr(not(feature = "std"), no_std)]
// #[macro_use] extern crate slice_as_array;
extern crate hex;
// pub use self::accContract::AccContract;
use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    scale::Encode,
    scale::Decode,
    SpreadLayout,
    PackedLayout,
)]

#[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_core::storage2::traits::StorageLayout)
)]
mod accContract {
    // #![no_std]
    use roleContract::RoleContract;
    // use ink_core::storage;
    use ink_core::{
        env::println,
        storage,
    };
    use ink_prelude::{
        format,
        string::String,
    };
    // use ink_prelude::format;
    // use ink_prelude::string::String;
    // use std::convert::TryInto;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    struct AccContract {
        /// The roleContract smart contract
        roleContract: storage::Value<RoleContract>,
        /// Store parent map: address -> assigner_addr
        parent_map: storage::HashMap<AccountId, AccountId>,
        /// Hardcode hashcode of roleContract 
        codehash: storage::Value<Hash>,
        /// Hardcode sub contract address of roleContract
        subContractAddr: storage::Value<AccountId>,
    }

    #[ink(event)]
    struct EventGetRoleType {
        #[ink(topic)]
        roleType: u32
    }

    #[ink(event)]
    struct EventGetRole {
        #[ink(topic)]
        addr: AccountId,
    }

    #[ink(event)]
    struct EventGetCodeHash {
        #[ink(topic)]
        value: Hash,
    }

    #[ink(event)]
    struct EventInstSubContract {
        #[ink(topic)]
        isOk: u32,
    }

    #[ink(event)]
    struct EventTestNum {
        #[ink(topic)]
        value: u128,
    }

    impl AccContract {
        /// Constructor that initializes the accContract with sub -contract (roleContract)
        #[ink(constructor)]
        fn new(&mut self) {
            // set up sub up sub contract
            // self.setSubContracts();
            let bytes: [u8; 32] = [18, 196, 115, 44, 96, 88, 10, 122, 72, 206, 170, 232, 229, 92, 191, 83, 96, 15, 203, 207, 155, 141, 222, 142, 187, 174, 115, 29, 169, 204, 39, 237];
            let account = AccountId::from(bytes);
            let roleContract = ink_core::env::call::FromAccountId::from_account_id(account);

            self.env()    
                .emit_event(
                    EventInstSubContract {
                        isOk: 1,
                    }
                );
            self.roleContract.set(roleContract);
            self.subContractAddr.set(account);

            // set caller's role type as System type (1)
            let caller = self.env().caller();
            let role_type = self.getAccountType(caller);
            if role_type == 0 {
                self.grantAccountType(caller, 1);
                self.env()    
                .emit_event(
                    EventGetRoleType {
                        roleType: 1,
                    }
                );
                println("Caller is set as System role type: 1.");
            } else {
                println("Caller already has role type.");
            }
        }

        /// Create instance of sub contracts (roleContract) and store it as a variable (roleContract) in accContract
        #[ink(message)]
        fn setSubContracts(&mut self) {
            let bytes: [u8; 32] = [18, 196, 115, 44, 96, 88, 10, 122, 72, 206, 170, 232, 229, 92, 191, 83, 96, 15, 203, 207, 155, 141, 222, 142, 187, 174, 115, 29, 169, 204, 39, 237];
            let account = AccountId::from(bytes);
            let roleContract = ink_core::env::call::FromAccountId::from_account_id(account);

            self.env()    
                .emit_event(
                    EventInstSubContract {
                        isOk: 1,
                    }
                );
            self.roleContract.set(roleContract);
            self.subContractAddr.set(account);
        }

        /// A message that can be called on instantiated contracts.
        /// This is to call addRoleType() in roleContract 
        #[ink(message)]
        fn grantAccountType(&mut self, addr: AccountId, value: u32) {
            let grant_role_type = self.getAccountType(addr);
            if grant_role_type != 0 {
                println("Grant account type failed: account already has role type!");
                return;
            }

            let caller = self.env().caller();
            let caller_role_type = self.getAccountType(caller);
            if caller_role_type == 1 && value == 2 {
                self.roleContract.addRoleType(addr, value);
                self.roleContract.addParent(addr, caller);
                println("Grant account type successfully!");
            } else {
                println("Grant account type failed: granter isn't system role type or grant role type isn't GWAL: 2");
            }
        }

        /// Get account type by address
        #[ink(message)]
        fn getAccountType(&self, addr: AccountId) -> u32 {
            self.roleContract.getRoleType(addr)
        }

        /// Add role permission. This is to call addRole() in roleContract 
        #[ink(message)]
        fn grantAddressRole(&mut self, addr: AccountId, func_name: u128, permission: u32) {
            let grant_role_type = self.getAccountType(addr);
            if grant_role_type == 0 {
                println("Grant account role failed: account type hasn't registered yet!");
                return;
            }

            if self.roleContract.hasRole(addr) {
                println("Grant account role failed: account already set role permission!");
                return;
            }

            // let name = func_name.clone();
            self.roleContract.addRole(addr, func_name, permission);
            println("Grant account role successfully!");
        }

        /// Get account role by address
        #[ink(message)]
        fn getAccountRole(&self, addr: AccountId) -> (u128, u32) {
            self.roleContract.getRole(addr)
        }

        /// Remove account role by address
        #[ink(message)]
        fn removeAddressRole(&mut self, addr: AccountId) -> u128 {
            let removed_func_name = self.roleContract.removeRole(addr);
            println("Removed account role permission successfully!");
            removed_func_name
        }

        /// Check if account has role permission by address
        #[ink(message)]
        fn doesAccountHaveRole(&self, addr: AccountId) -> bool {
            self.roleContract.hasRole(addr)
        }

        /// Register approval address
        #[ink(message)]
        fn registerRoleApprovalAddress(&mut self, addr: AccountId, func_name: u128, stage: u32) {
            if self.getAccountType(addr) != 2 {
                println("Approved address isn't GWAL type.");
                return;
            }

            // let name = func_name.clone();
            self.roleContract.addApprover(addr, func_name, stage);
            println("Register approval account successfully!");
        }

        /// Check if account is already registered as an approver
        #[ink(message)]
        fn isApprovalAddress(&self, addr: AccountId, func_name: u128, stage: u32) -> bool {
            // let name = func_name.clone();
            self.roleContract.isApprover(addr, func_name, stage)
        }

        /// Get parent account by address
        #[ink(message)]
        fn getAccountParent(&self, addr: AccountId) -> AccountId {
            self.roleContract.getParent(addr)
        }

        /// Simple return caller of roleContract
        #[ink(message)]
        fn getCallerFromSub(&self) {
            self.roleContract.getCaller();
        }

        /// Get caller
        #[ink(message)]
        fn getCaller(&self) -> AccountId {
            let caller = self.env().caller();
            self.env()    
                .emit_event(
                    EventGetRole {
                        addr: caller,
                    }
                );
            caller
        }

        /// Get total balance
        #[ink(message)]
        fn getBalance(&self) -> u128 {
            let total_balance = self.env().balance();
            self.env()
                .emit_event(
                    EventTestNum {
                        value: total_balance,
                    }
                );
            total_balance
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use std::any::type_name;

        fn type_of<T>(_: T) -> &'static str {
            type_name::<T>()
        }

        // We test if the default constructor does its job.
        #[test]
        fn test_new() {
            let bytes: [u8; 32] = [70, 229, 185, 240, 139, 39, 86, 26, 218, 180, 253, 83, 104, 179, 133, 218, 180, 63, 215, 123, 32, 78, 48, 9, 75, 78, 14, 177, 50, 45, 6, 60];
            let role_codehash = Hash::from(bytes);
            let total_balance = 20000000000000000;
            let roleContract = RoleContract::new()
                                    .endowment(total_balance / 2)
                                    .using_code(role_codehash);
                                    // .instantiate();
            println!("total balance: {:?}", total_balance);
            
            // let bal = AccContract::new().getBalance();
            // println!("Get balance: {:?}", bal);
            // println!("{}", type_of(bal));
                // .endowment(total_balance / 2)
                // .using_code(role_codehash)
                // .instantiate()
                // .expect("failed at instantiating the `RoleContract` contract");
        }

        #[test]
        fn test_codehash() {
            // let accContract = AccContract::new();

            // let bytes: [u8; 32] = [184, 9, 249, 87, 31, 98, 81, 126, 204, 241, 96, 197, 139, 131, 202, 70, 104, 69, 37, 197, 82, 116, 186, 208, 94, 137, 42, 60, 242, 214, 92, 224];
            // let account = AccountId::from(bytes);
            // let roleContract = ink_core::env::call::FromAccountId::from_account_id(account);

            let str = "02edabdddc243f2cabf4924fa4fe85b31ffb90a875096543fd84de4377856852";
            let accid = "5GE1dYPr59XAngcfCm2P9DEzaVr7MQ1i7s7WpJNJCuEychhf"; //Get bytes [u8; 32] from api keyring.decodeAddress()
            let mut bytes = [0u8; 32];
            println!("Print!");
            let status = match hex::decode_to_slice(str, &mut bytes) {
                Ok(b)  => b,
                Err(e) => eprintln!("Error: {:?}", e),
            };
            // let hash = accContract.getCodehash();
            // assert!(1==2);
            let role_codehash = Hash::from(bytes);
            println!("Print out bytes: {:?}", bytes);

            let selector = ink_core::env::call::Selector::from_str("settlement");
            let selector_bytes = ink_core::env::call::Selector::to_bytes(selector);
            println!("Selector: {:?}", selector);
            println!("Bytes(selector): {:?}", selector_bytes);

            // let newBytes: [u8; 32] = [123, 38, 58, 172, 130, 234, 73, 96, 1, 220, 227, 92, 10, 31, 148, 177, 82, 22, 67, 232, 169, 81, 144, 64, 27, 194, 82, 116, 220, 9, 71, 102];
            // println!("Print out new bytes: {:?}", newBytes);

            // let default_hash = Hash::default();
            // println!("Print out default_hash: {:?}", default_hash);

            // assert_eq!(1, 2);
            // assert!(1 == 2);
        }

        #[test]
        fn temp_test() {
            let mut s = String::from("settlement");
            let bytes = unsafe { s.as_bytes_mut() };
            println!("Bytes is: {:x?}", bytes);
        }
    }
}
