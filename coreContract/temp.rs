/* Status is only used for check, not involved in core logic */
// status = 0, coreContract initialized
// status = 1, new SST created, ready for distribution
// status = 2, ready for settlement in a session
// status = 3, complete settlement
// status = 4, distribute over
// status = 5, start multi approval process
// status = 6, complete aprroval process

/* Role codes
    1001 Call a system contract
    ----
    2001 Call a GWAL contract
    2002 Receive SST issuance holding
    2003 Receive settle from other CF (not compatible with 2002)
    2004 Call execute
    2005 Call settleMWALs
    2006 Call distribute 
    2008 canTransferToOther (CF operations/interface)
    2009 CF GWAL cold storage
    2010 CF GWAL operations
*/
#![cfg_attr(not(feature = "std"), no_std)]

// pub use self::coreContract::{
//     pendingTransfer,
// };

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
#[derive(
    Debug,
    Copy,
    Clone,
)]

// #[ink::contract(version = "0.1.0")]
mod coreContract {
    use ink_core::{
        env::println,
        storage,
    };
    use roleContract::RoleContract;
    use regtrSSTContract::RegtrSstContract;
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_core::storage::Stash;
    use ink_primitives::Key;
    use ink_core::storage::stash::Values;
    // use std::ops::{Add, Sub, Mul, MulAssign, Div, Rem, Neg};
    // use std::iter::Sum;
    // use std::str::FromStr;
    use core::str::FromStr;
    // use bigdecimal::{BigDecimal, Zero, One};
    // use std::fmt;

    // pub struct pendingTransfer {
    //     /// unique id for transferToOther()
    //     pub id: u128,
    //     /// approval stage
    //     pub stage: u8,
    //     /// SST class
    //     pub class: u32,
    //     /// amount of SST to be approved for transferring
    //     pub amount: Balance,
    //     /// the storage GWAL transfer to
    //     pub to: AccountId,
    //     /// the storage GWAL transfer from
    //     pub from: AccountId,
    // }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    struct CoreContract {
        // sub contracts
        roleContract: storage::Value<RoleContract>,
        regtrSSTContract: storage::Value<RegtrSstContract>,
        // core storage
        gwal_sst_map: storage::HashMap<AccountId, (u32, String)>,
        executed_trades: storage::HashMap<u128, (u128, u32, String, u128, u8)>,
        settlements: storage::HashMap<u128, Vec<(AccountId, u32, String)>>,
        pending_transfer: storage::HashMap<u128, (u128, u8, u32, String, AccountId, AccountId)>,
        confirmations: storage::HashMap<u128, Vec<(u128, u32, AccountId)>>,
        // store temp variables, values will be reset before each new SST creation
        holding_gwal: storage::Value<AccountId>,
        readySettlement: storage::Value<bool>,
        isDistributedOver: storage::Value<bool>,
        status: storage::Value<u8>,
        symbol: storage::Value<u128>,
        distributions: storage::Vec<(AccountId, u32, String)>,
        settle_items: storage::Vec<(AccountId, u32, String)>,
        // store latest ptId
        pending_transfer_id: storage::Value<u128>,
    }

    #[ink(event)]
    struct EventInstSubContract {
        #[ink(topic)]
        isOk: u32,
    }

    #[ink(event)]
    struct CreateSSTEvent {
        #[ink(topic)]
        holding: AccountId,
        #[ink(topic)]
        classId: u32,
        #[ink(topic)]
        sstAmount: String,
    }

    #[ink(event)]
    struct TransferSSTEvent {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        amount: String,
        #[ink(topic)]
        class: u32,
    }

    #[ink(event)]
    struct DistributeEvent {
        #[ink(topic)]
        subAddr: AccountId,
        #[ink(topic)]
        amount: String,
        #[ink(topic)]
        class: u32,
    }

    #[ink(event)]
    struct ExecuteTradeEvent {
        #[ink(topic)]
        sessionId: u128,
        #[ink(topic)]
        class: u32,
        #[ink(topic)]
        amount: String,
        #[ink(topic)]
        timestamp: u128,
        #[ink(topic)]
        tradeType: u8,
    }

    #[ink(event)]
    struct SettleEvent {
        #[ink(topic)]
        sessionId: u128,
        settle_items: Vec<(AccountId, u32, String)>,
    }

    #[ink(event)]
    struct StartApprovEvent {
        #[ink(topic)]
        id: u128,
    }

    #[ink(event)]
    struct CancelApprovEvent {
        #[ink(topic)]
        id: u128,
    }

    #[ink(event)]
    struct TransferToOtherStatus {
        #[ink(topic)]
        id: u128,
        #[ink(topic)]
        func: u128,
        #[ink(topic)]
        stage: u32,
        #[ink(topic)]
        state: String,
    }

    impl CoreContract {
        /// Constructor that initializes core contract
        #[ink(constructor)]
        fn new(&mut self) {
            self.status.set(0);
            self.isDistributedOver.set(false);
            self.readySettlement.set(false);
            self.pending_transfer_id.set(0);
            // self.setSubContracts();
            let bytes: [u8; 32] = [140, 132, 146, 159, 233, 145, 197, 8, 154, 234, 148, 207, 56, 152, 14, 13, 215, 104, 6, 170, 63, 185, 216, 89, 239, 184, 36, 168, 251, 19, 27, 124];
            let bytes2: [u8; 32] = [253, 156, 70, 133, 233, 174, 178, 242, 131, 218, 180, 166, 41, 88, 41, 11, 163, 20, 193, 69, 61, 73, 14, 76, 162, 203, 109, 70, 81, 190, 251, 239];
            let account = AccountId::from(bytes);
            let account2 = AccountId::from(bytes2);
            let roleContract = ink_core::env::call::FromAccountId::from_account_id(account);
            let regtrSSTContract = ink_core::env::call::FromAccountId::from_account_id(account2);

            self.env()    
                .emit_event(
                    EventInstSubContract {
                        isOk: 1,
                    }
                );
            self.roleContract.set(roleContract);
            self.regtrSSTContract.set(regtrSSTContract);
        }

        /// Create instance of sub contracts (roleContract & regtrSSTContract) and store it as a variable (roleContract & regtrSSTContract) in coreContract
        #[ink(message)]
        fn setSubContracts(&mut self) {
            let bytes: [u8; 32] = [140, 132, 146, 159, 233, 145, 197, 8, 154, 234, 148, 207, 56, 152, 14, 13, 215, 104, 6, 170, 63, 185, 216, 89, 239, 184, 36, 168, 251, 19, 27, 124];
            let bytes2: [u8; 32] = [253, 156, 70, 133, 233, 174, 178, 242, 131, 218, 180, 166, 41, 88, 41, 11, 163, 20, 193, 69, 61, 73, 14, 76, 162, 203, 109, 70, 81, 190, 251, 239];
            let account = AccountId::from(bytes);
            let account2 = AccountId::from(bytes2);
            let roleContract = ink_core::env::call::FromAccountId::from_account_id(account);
            let regtrSSTContract = ink_core::env::call::FromAccountId::from_account_id(account2);

            self.env()    
                .emit_event(
                    EventInstSubContract {
                        isOk: 1,
                    }
                );
            self.roleContract.set(roleContract);
            self.regtrSSTContract.set(regtrSSTContract);
        }

        /// Create single class of SST and allocate amount to holding GWAL
        /// status code for create SST is 1
        #[ink(message)]
        fn create(&mut self, sstId: u128, class: u32, amount: String, holdingAddr: AccountId) -> bool {
            // Role validations
            let caller = self.env().caller();
            if self.roleContract.getRoleType(caller) != 1 {
                println("Create failed: only system account type can create new SST!");
                return false;
            }

            let (holding_role, _) = self.roleContract.getRole(holdingAddr);
            if self.roleContract.getRoleType(holdingAddr) != 2 || holding_role != 2002 {
                println("Created failed: holding address should be a GWAL type with holding role!");
                return false;
            }

            // Call regtrSSTContract contract to register new SST
            let coreAddr = self.env().account_id();
            let isRegister = self.regtrSSTContract.registerSST(sstId, coreAddr);
            if !isRegister { return false; }

            // Update gwal_sst_map
            let v = (class, amount.clone());
            self.symbol.set(sstId);
            self.holding_gwal.set(holdingAddr);
            match self.gwal_sst_map.get(&holdingAddr) {
                Some(_) => {
                    *self.gwal_sst_map.get_mut(&holdingAddr).unwrap() = v
                }
                None => {
                    self.gwal_sst_map.insert(holdingAddr, v);
                }
            };

            // Update status: new SST created, ready for distribution
            self.status.set(1);

            self.env()
                .emit_event(
                    CreateSSTEvent {
                        holding: holdingAddr,
                        classId: class,
                        sstAmount: amount,
                    }
                );
            
            true
        }

        /// Transfer by other(internal) core methods
        #[ink(message)]
        fn transfer(&mut self, from: AccountId, to: AccountId, amount: String, class: u32) -> bool {
            // Check if distribution is over
            if !*self.isDistributedOver.get() {
                println("Transfer failed: SST isn't distriuted over yet!");
            }

            // Update gwal_sst_map for both from & to address
            let is_success_from = self.update_gwal_sst_map(from, class, amount.clone(), 1);
            if is_success_from {
                let is_success_to = self.update_gwal_sst_map(to, class, amount.clone(), 2);
                self.env()
                    .emit_event(
                        TransferSSTEvent {
                            from: from,
                            to: to,
                            amount: amount,
                            class: class,
                        }
                    );

                is_success_to
            } else {
                println("Transfer failed: update gwal of from failed!");
                return false;
            }
        }

        /// Distribute SST to initial MWAL holder
        /// actual SST balance is still held by the CF GWAL 
        /// until it is transferred to another GWAL
        #[ink(message)]
        fn distribute(&mut self, subMWAL: AccountId, class: u32, amount: String) -> (u8, bool) {
            // SST registered validation 
            let sstId = *self.symbol.get();
            if !self.regtrSSTContract.isRegistered(sstId) {
                println("Failed: the SST hasn't registered yet!");
                return (1, false);
            }

            // Role validation
            let caller = self.env().caller();
            let (role, _) = self.roleContract.getRole(caller);
            if self.roleContract.getRoleType(caller) != 2 || role != 2006 {
                println("Failed: caller should be a GWAL type with distribute role!");
                return (1, false);
            }

            // Distribute SST
            let holdingAddr = *self.holding_gwal.get();
            if *self.isDistributedOver.get() {
                println("Failed: distribution is over!");
                return (1, false);
            } else {
                if let Some(_) = self.gwal_sst_map.get(&holdingAddr) {
                    let k = (subMWAL, class, amount.clone());
                    self.distributions.push(k);
                    let mut subTotal = 0f64;
                    for d in self.distributions.iter() {
                        let (_, _, amount) = d;
                        let amount_f64 = f64::from_str(&amount).unwrap();
                        subTotal += amount_f64;
                    }
                    // self.transfer(holdingAddr, subMWAL, amount, class);
                    let (_, amount_holding) = self.gwal_sst_map.get(&holdingAddr).unwrap();
                    let amount_holding_f64 = f64::from_str(&amount_holding).unwrap();

                    self.env()
                        .emit_event(
                            DistributeEvent {
                                subAddr: subMWAL,
                                amount: amount,
                                class: class,
                            }
                        );
                    
                    if amount_holding_f64 == subTotal {
                        // Set status as distribute over
                        self.status.set(4);
                        self.isDistributedOver.set(true);
                    }
                }
            }

            // return status
            return (*self.status.get(), true);
        }

        /// Execute trade by ATS
        #[ink(message)]
        fn executeTrade(&mut self, _sessionId: u128, _sstId: u128, _class: u32, _amount: String, _timestamp: u128, _tradeType: u8) -> bool {
            // Role validation
            let caller = self.env().caller();
            let (role, _) = self.roleContract.getRole(caller);
            if self.roleContract.getRoleType(caller) != 2 || role != 2004 {
                println("Executed trade failed: caller should be a GWAL type with execute trade role!");
                return false;
            }

            // Execute trades
            let executedTrade = self.executed_trades.get(&_sessionId);
            if let Some(_) = executedTrade {
                println("Failed: the session id already existed!");
                return false;
            } else {
                self.executed_trades.insert(_sessionId, (_sstId, _class, _amount.clone(), _timestamp, _tradeType));
            }

            self.env()
                .emit_event(
                    ExecuteTradeEvent {
                        sessionId: _sessionId,
                        class: _class,
                        amount: _amount,
                        timestamp: _timestamp,
                        tradeType: _tradeType,
                    }
                );

            true
        }

        /// Add settle item
        /// because of the Vec type input issue of UI test, separate function of adding settle items from settleMWALs()
        #[ink(message)]
        fn addSettleItem(&mut self, sub_mwal: AccountId, class: u32, amount: String, last: bool) {
            self.settle_items.push((sub_mwal, class, amount));
            if last {
                // Set status as ready for settlement
                self.status.set(2);
                self.readySettlement.set(true);
            }
        }

        /// Settle MWAL balances
        /// use for public verification of trades vs settlements and future auditability 
        #[ink(message)]
        fn settleMWALs(&mut self, _sessionId: u128) -> bool {
            // Role validation
            let caller = self.env().caller();
            let (role, _) = self.roleContract.getRole(caller);
            if self.roleContract.getRoleType(caller) != 2 || role != 2005 {
                println("Settle failed: caller should be a GWAL type with settle role!");
                return false;
            }

            // Check all settle items are ready
            if !*self.readySettlement.get() {
                println("Status not match: not ready for settlement!");
                return false;
            }

            // Start settlement
            if let Some(_) = self.settlements.get(&_sessionId) {
                println("Settlement failed: the session id is already existed!");
                return false;
            } else {
                let mut settleItems = Vec::new();
                let mut settleItems_copy = Vec::new();
                for item in self.settle_items.iter() {
                    let (sub_mwal, class, amount) = item;
                    settleItems.push((*sub_mwal, *class, amount.clone()));
                    settleItems_copy.push((*sub_mwal, *class, amount.clone()));
                }
                self.settlements.insert(_sessionId, settleItems);

                self.env()
                .emit_event(
                    SettleEvent {
                        sessionId: _sessionId,
                        settle_items: settleItems_copy,
                    }
                );

                // Set status as complete settlement
                self.status.set(3);
            }

            true
        }

        /// Start the multi-stage approval process
        #[ink(message)]
        fn transferToOther(&mut self, sstId: u128, class: u32, amount: String, to: AccountId, from: AccountId) -> u128 {
            let mut ptId = *self.pending_transfer_id.get();
            let pt = self.pending_transfer.get(&ptId);

            // SST registered validation
            if !self.regtrSSTContract.isRegistered(sstId) {
                println("Failed: the SST id isn't registered!");
                return ptId;
            }

            // Role validation
            let caller = self.env().caller();
            let (role, _) = self.roleContract.getRole(caller);
            if self.roleContract.getRoleType(caller) != 2 || role != 2008 {
                println("Failed: caller should be a GWAL type with canTransferToOther role!");
                return ptId;
            }

            // Update pending_transfer
            if let Some(_) = pt {
                println("Failed to start approval: pending_transfer_id already existed!");
            } else {
                self.pending_transfer.insert(ptId, (ptId, 0, class, amount, to, from));

                // Set status as start multi approval process
                self.status.set(5);
                self.env()
                    .emit_event(
                        StartApprovEvent {
                            id: ptId,
                        }
                    );

                ptId += 1;
                self.pending_transfer_id.set(ptId);
            }

            ptId
        }

        /// Cancel approval process
        #[ink(message)]
        fn cancelTransferToOther(&mut self, ptId: u128) -> bool {
            // Role validation
            let caller = self.env().caller();
            let (func, _) = self.roleContract.getRole(caller);
            if func != 2008 {
                println("Cancel failed: caller doesn't have role permission of canTransferToOther (2008)!");
                return false;
            }

            // Check if all pending confirmations are done; if all done, cannot cancel
            if self.checkPendingConfirmations(ptId) == 0 {
                println("Cancel failed: already confirmed all!");
                return false;
            }

            // Update pending_transfer & confirmations
            self.pending_transfer.remove(&ptId);
            self.confirmations.remove(&ptId);

            self.env()
                .emit_event(
                    CancelApprovEvent {
                        id: ptId,
                    }
                );
            
            true
        }

        /// Approve to next stage
        #[ink(message)]
        fn approveTransferToOther(&mut self, ptId: u128, func_name: u128, stage: u32) -> bool {
            // Role validation
            let caller = self.env().caller();
            let (func_caller, _) = self.roleContract.getRole(caller);
            if self.roleContract.getRoleType(caller) != 2 || func_caller != func_name {
                println("Approve failed: caller doesn't have correct role permission!");
                return false;
            }

            // Add confirmation
            let bytes: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            let emptyAddr = AccountId::from(bytes);
            let mut state = String::from(""); // for event data
            let isConfirmed = self.confirmations.get(&ptId);
            let (id, mut _stage, class, _, to , from) = *self.pending_transfer.get_mut(&ptId).unwrap();
            let amount = self.pending_transfer.get_mut(&ptId).unwrap().3.clone();
            let mut v = Vec::new();
            v.push((func_name, stage, caller));

            if let Some(_) = isConfirmed {
                for c in isConfirmed.unwrap().iter() {
                    v.push(*c);
                }
                *self.confirmations.get_mut(&ptId).unwrap() = v;
            } else {
                self.confirmations.insert(ptId, v);
                _stage += 1;
                state = String::from("ready_for_next");
            }

            // Check if all confirmations are done; if so, call transfer()
            if self.checkPendingConfirmations(ptId) == 0 {
                // Transfer gwal balance
                let is_success = self.transfer(from, to, amount, class);
                if !is_success {return false;}
                // Set status as complete aprroval process
                self.status.set(6);
                state = String::from("complete");
            }

            self.env()
                .emit_event(
                    TransferToOtherStatus {
                        id: ptId,
                        func: func_name,
                        stage: stage,
                        state: state,
                    }
                );

            true
        }

        /* Below are helper functions */

        /// Get distributions
        #[ink(message)]
        fn getDistribution(&mut self) -> Vec<(AccountId, u32, String)> {
            let arr = self.distributions.iter();
            let mut vec = Vec::new();
            for c in arr {
                let (sub_mwal, class, amount) = c;
                vec.push((*sub_mwal, *class, amount.clone()));
            }

            vec
        }

        /// Get settle items
        #[ink(message)]
        fn getSettleItems(&mut self) -> Vec<(AccountId, u32, String)> {
            let arr = self.settle_items.iter();
            let mut vec = Vec::new();
            for c in arr {
                let (sub_mwal, class, amount) = c;
                vec.push((*sub_mwal, *class, amount.clone()));
            }

            vec
        }

        /// Get settlements array
        #[ink(message)]
        fn getSettlements(&mut self, sessionId: u128) -> Vec<(AccountId, u32, String)> {
            let arr = self.settlements.get(&sessionId);
            let mut vec = Vec::new();

            if let Some(_) = arr {
                for c in arr.unwrap().iter() {
                    let (sub_mwal, class, amount) = c;
                    vec.push((*sub_mwal, *class, amount.clone()));
                }
            } else {
                println("Didn't find match settlements!");
            }

            vec
        }

        /// Get distributions length
        #[ink(message)]
        fn get_len(&self) -> u32 {
            self.distributions.len()
        }

        /// Get class & sst by gwal
        #[ink(message)]
        fn getGwal(&self, addr: AccountId) -> (u32, String) {
            let sst_map = self.gwal_sst_map.get(&addr);
            match sst_map {
                Some(_) => {
                    let (class, amount) = sst_map.unwrap();
                    (*class, amount.clone())
                }
                None => {
                    (0u32, String::from("0"))
                }
            }
            // *self.gwal_sst_map.get(&addr).unwrap_or(&(0u32, "0".to_string()))
        }

        /// Get pending transfer by ptId
        #[ink(message)]
        fn getPendingTransfer(&self, ptId: u128) -> (u128, u8, u32, String, AccountId, AccountId) {
            let pt = self.pending_transfer.get(&ptId);
            if let Some(_) = pt {
                let (ptId, stage, class, amount, to, from) = pt.unwrap();
                return (*ptId, *stage, *class, amount.clone(), *to, *from);
            } else {
                println("Didn't find match pending transfer!");
                let bytes: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
                let emptyAddr = AccountId::from(bytes);
                return (0, 0, 0, String::from("0"), emptyAddr, emptyAddr);
            }
        }

        /// Check how many confirmations pend for approval
        #[ink(message)]
        fn checkPendingConfirmations(&self, ptId: u128) -> u32 {
            let mut allChecks = BTreeMap::new();
            allChecks.insert((2008, 0), ());
            allChecks.insert((2009, 1), ());
            allChecks.insert((2010, 2), ());
            let arr = self.confirmations.get(&ptId);
            if let Some(_) = arr {
                for c in arr.unwrap().iter() {
                    let (func, stage, approver) = *c;
                    if allChecks.get(&(func, stage)).is_some() {
                        allChecks.remove(&(func, stage));
                    }
                }
            } else {
                println("No any confirmed yet!");
            }

            allChecks.len() as u32
        }

        /// Check confirmation
        #[ink(message)]
        fn checkConfirmation(&self, id: u128) -> Vec<(u128, u32, AccountId)> {
            let arr = self.confirmations.get(&id);
            let mut vec = Vec::new();

            if let Some(_) = arr {
                for c in arr.unwrap().iter() {
                    vec.push(*c);
                }
            } else {
                println("Didn't find match confirmation!");
            }

            vec
        }

        /// Check status & symbol
        #[ink(message)]
        fn checkState(&self) -> (u8, u128, u128) {
            let _status = *self.status.get();
            let _symbol = *self.symbol.get();
            let _ptId = *self.pending_transfer_id.get();

            (_status, _symbol, _ptId)
        }

        /// Reset for another new SST
        #[ink(message)]
        fn reset(&mut self) {
            let bytes: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            let emptyAddr = AccountId::from(bytes);
            self.holding_gwal.set(emptyAddr);
            self.status.set(0);
            self.isDistributedOver.set(false);
            self.readySettlement.set(false);
            self.symbol.set(0);
            for i in 0..self.settle_items.len() {
                self.settle_items.pop();
            }

            for j in 0..self.distributions.len() {
                self.distributions.pop();
            }
        }

        /// Update gwal_sst_map
        fn update_gwal_sst_map(&mut self, addr: AccountId, class: u32, amount: String, addrType: u8) -> bool {
            let gwal_addr = self.gwal_sst_map.get(&addr);
            match addrType {
                1 => {
                    if let Some(_) = gwal_addr {
                        let (class_addr, bal_addr_str) = gwal_addr.unwrap();
                        let bal_addr = f64::from_str(&bal_addr_str).unwrap();
                        let amount_f64 = f64::from_str(&amount).unwrap();

                        if class != *class_addr {
                            println("Transfer failed: class to be transferred doesn't match to token class of from address!");
                            return false;
                        }
                        if bal_addr < amount_f64 {
                            println("Transfer failed: balance of from isn't enough for transfer!");
                            return false;
                        }
                        self.gwal_sst_map.mutate_with(&addr, move |(c, b)| {
                            let mut b_f64 = f64::from_str(b).unwrap();
                            let amount_f64 = f64::from_str(&amount).unwrap();
                            b_f64 -= amount_f64;
                            // *b -= amount;
                        });
                    } else {
                        println("Transfer failed: cannot find match address of from in gwal_sst_map");
                        return false;
                    }
                },
                2 => {
                    if let Some(_) = gwal_addr {
                        // self.gwal_sst_map.mutate_with(&addr, |(c, b)| *b += amount);
                        self.gwal_sst_map.mutate_with(&addr, move |(c, b)| {
                            let mut b_f64 = f64::from_str(b).unwrap();
                            let amount_f64 = f64::from_str(&amount).unwrap();
                            b_f64 += amount_f64;
                            // *b += amount
                        });
                    } else {
                        self.gwal_sst_map.insert(addr, (class, amount));
                    }
                },
                _ => println("Transfer failed: account type is invalid!"),
            }

            true
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
        // fn test_distribute() {
        //     let mut coreContract = CoreContract::new();
        //     let bytes: [u8; 32] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        //     let holding = AccountId::from(bytes);
        //     coreContract.create(1, 2, 100, holding);
        //     let (class_holding, sst_holding) = coreContract.getGwal(holding);
        //     println!("After create, holding class: {x:?}, holding amount: {y:?}", x=class_holding, y=sst_holding);
        //     // let (status, symbol) = coreContract.checkState();
        //     // println!("Status: {x:?}, sstId: {y:?}", x=status, y=symbol);

        //     let bytes3: [u8; 32] = [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        //     let storage = AccountId::from(bytes3);
        //     let bytes4: [u8; 32] = [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        //     let sub1 = AccountId::from(bytes4);
        //     let bytes5: [u8; 32] = [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        //     let sub2 = AccountId::from(bytes5);
        //     let status1 = coreContract.distribute(sub1, 2, 40, storage);
        //     println!("Distribute status: {:?}", status1);

        //     let (class_holding_aft, sst_holding_aft) = coreContract.getGwal(holding);
        //     let (class_to_aft, sst_to_aft) = coreContract.getGwal(storage);
        //     println!("After 1st distribution, _holding class: {x:?}, _holding amount: {y:?}", x=class_holding_aft, y=sst_holding_aft);
        //     println!("After 1st distribution, _to class: {x:?}, _to amount: {y:?}", x=class_to_aft, y=sst_to_aft);

        //     let status2 = coreContract.distribute(sub2, 2, 60, storage);
        //     println!("Distribute status: {:?}", status2);

        //     let (class_holding_aft2, sst_holding_aft2) = coreContract.getGwal(holding);
        //     let (class_to_aft2, sst_to_aft2) = coreContract.getGwal(storage);
        //     println!("After 2nd distribution, _holding class: {x:?}, _holding amount: {y:?}", x=class_holding_aft2, y=sst_holding_aft2);
        //     println!("After 2nd distribution, _to class: {x:?}, _to amount: {y:?}", x=class_to_aft2, y=sst_to_aft2);

        //     // let bytes2: [u8; 32] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        //     // let submwal = AccountId::from(bytes2);
        //     // coreContract.distribute(submwal, 2);
        //     let len = coreContract.get_len();
        //     println!("Length of distributions is: {:?}", len);

        //     let (sub, c, num) = coreContract.getDistribution();
        //     println!("sub_mwal_1: {x:?}, class_sub1: {y:?}, amount_sub1: {z:?}", x=sub, y=c, z=num);

        //     let (sub2, c2, num2) = coreContract.getDistribution();
        //     println!("sub_mwal_2: {x:?}, class_sub2: {y:?}, amount_sub2: {z:?}", x=sub2, y=c2, z=num2);
            
        // }

        // #[test]
        // fn test_multi_approval() {
        //     let mut coreContract = CoreContract::new();
        //     let bytes: [u8; 32] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        //     let to_gwal = AccountId::from(bytes);
        //     let bytes2: [u8; 32] = [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        //     let from_gwal = AccountId::from(bytes2);
        //     let bytes3: [u8; 32] = [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        //     let holding = AccountId::from(bytes3);

        //     coreContract.create(1, 2, 100, holding);

        //     let bytes4: [u8; 32] = [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        //     let sub1 = AccountId::from(bytes4);
        //     let bytes5: [u8; 32] = [5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        //     let sub2 = AccountId::from(bytes5);
        //     let status1 = coreContract.distribute(sub1, 2, 40, from_gwal);
        //     let status2 = coreContract.distribute(sub2, 2, 60, from_gwal);

        //     let (class_holding, amount_holding) = coreContract.getGwal(holding);
        //     println!("After distribution over, holding class: {x:?}, holding amount: {y:?}", x=class_holding, y=amount_holding);

        //     let (status, sstId, ptId) = coreContract.checkState();

        //     let ptId_r = coreContract.transferToOther(sstId, 2, 100, to_gwal, from_gwal);
        //     let (ptId_aft, stage, class, amount, to, from) = coreContract.getPendingTransfer(ptId);
        //     println!("ptId: {a:?}, stage: {b:?}, class: {c:?}, amount: {d:?}, to: {e:?}, from: {f:?}", a=ptId_aft, b=stage, c=class, d=amount, e=to, f=from);
        //     println!("ptId: {:?}", ptId_r);

        //     let (status2, _, _) = coreContract.checkState();
        //     println!("status: {x:?}", x=status2);

        //     let len_btf = coreContract.checkPendingConfirmations(ptId_aft);
        //     println!("There are {:?} pending confirmation", len_btf);

        //     let isApproved = coreContract.approveTransferToOther(ptId_aft, 2008, 0);
        //     println!("Is approved by func_name: 2008 at stage: 0? {:?}", isApproved);

        //     let len1 = coreContract.checkPendingConfirmations(ptId_aft);
        //     println!("There are {:?} pending confirmation", len1);

        //     let isApproved2 = coreContract.approveTransferToOther(ptId_aft, 2009, 1);
        //     println!("Is approved by func_name: 2009 at stage: 1? {:?}", isApproved2);

        //     let len2 = coreContract.checkPendingConfirmations(ptId_aft);
        //     println!("There are {:?} pending confirmation", len2);

        //     let isApproved3 = coreContract.approveTransferToOther(ptId_aft, 2010, 2);
        //     println!("Is approved by func_name: 2010 at stage: 2? {:?}", isApproved3);

        //     let len3 = coreContract.checkPendingConfirmations(ptId_aft);
        //     println!("There are {:?} pending confirmation", len3);

        //     let confirms = coreContract.checkConfirmation(ptId_aft);
        //     for c in confirms {
        //         let (func, stage, approver) = c;
        //         println!("Already confirmed: ");
        //         println!("func_name: {x:?}, stage: {y:?}, approver: {z:?}", x=func, y=stage, z=approver);
        //     }

        //     let (status3, sstId3, ptId3) = coreContract.checkState();
        //     println!("new status: {:?}", status3);

        //     let (class_from, amount_from) = coreContract.getGwal(from_gwal);
        //     println!("After approval completed, from_gwal class: {x:?}, from_gwal amount: {y:?}", x=class_from, y=amount_from);
        //     let (class_to, amount_to) = coreContract.getGwal(to_gwal);
        //     println!("After approval completed, to_gwal class: {x:?}, to_gwal amount: {y:?}", x=class_to, y=amount_to);

        //     // let (class_from, amount_from) = coreContract.getGwal(from_gwal);
        //     // println!("After approval completed, from_gwal class: {x:?}, from_gwal amount: {y:?}", x=class_from, y=amount_from);
        //     // let (class_to, amount_to) = coreContract.getGwal(to_gwal);
        //     // println!("After approval completed, to_gwal class: {x:?}, to_gwal amount: {y:?}", x=class_to, y=amount_to);

        //     coreContract.cancelTransferToOther(ptId_aft);
        //     let (ptId_c, stage_c, class_c, amount_c, to_c, from_c) = coreContract.getPendingTransfer(ptId_aft);
        //     println!("ptId: {a:?}, stage: {b:?}, class: {c:?}, amount: {d:?}, to: {e:?}, from: {f:?}", a=ptId_c, b=stage_c, c=class_c, d=amount_c, e=to_c, f=from_c);
        //     let len4 = coreContract.checkPendingConfirmations(ptId_aft);
        // }
    }
}
