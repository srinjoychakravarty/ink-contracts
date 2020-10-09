# Test Report for ink! contracts

## Summary
* Test individual function in roleContract, accContract, regtrSSTContract, coreContract
* Test write and get correct data via delegated way (call sub contracts by parent contract)
* Test process of new SST creation
* Test multiple rounds of new SST creation

Documentations: <br />
[Phase_1_SST_Specification.md](https://github.com/prometheumlabs/prometheum-blockchain-design/blob/master/requirements/Phase_1_SST_Specification.md)

[PEX03_settlement_data_flow.pdf](https://github.com/prometheumlabs/prometheum-blockchain-design/blob/master/examples/PEX03_settlement_data_flow.pdf)

## Features not implemented
* Settle all items at the same time
    - Due to the input issue in Polkadot UI, cannot put Vec type input ex. class[] or amount[] correctly
* Signature recover and verify in contract
    - Didn't find a function can do this feature in ink! 2.1
    - Implement multi-approval process based on [https://github.com/paritytech/ink/blob/master/examples/multisig_plain/lib.rs](examples/multisig-plain)

## <a name="ui"></a>Test flow via Polkadot UI
### Role types
    - 1001 Call a system contract
    - ----
    - 2001 Call a GWAL contract
    - 2002 Receive SST issuance holding
    - 2003 Receive settle from other CF (not compatible with 2002)
    - 2004 Call execute
    - 2005 Call settleMWALs
    - 2006 Call distribute 
    - 2008 canTransferToOther (CF operations/interface)
    - 2009 CF GWAL cold storage
    - 2010 CF GWAL operations

### Test steps
1. **Upload & deploy contracts in order** <br />
    a. roleContract <br />
    b. accContract <br />
    c. regtrSSTContract <br />
    d. coreContract <br />

2. **Set account type & role**
    >Bob is auto set as System type (1) after deploy roleContract. <br />
    
    *// call accContract to set* <br />
    * Alice -> holding_GWAL <br />
        - set type as 2 <br />
        - set role as 2002 <br />
    * Charlie -> distribute_GWAL <br />
        - set type as 2 <br />
        - set role as 2006 <br />
    * Eve -> canTransferToOther_GWAL <br />
        - set type as 2 <br />
        - set role as 2008 <br />
    * Ferdie -> coldStorage_GWAL <br />
        - set type as 2 <br />
        - set role as 2009 <br />
    * Cindy -> operation_GWAL (create additional account in UI) <br />
        - set type as 2  <br />
        - set role as 2010 <br />
    * Lucy -> settleMWALs_GWAL (create additional account in UI) <br />
        - set type as 2  <br />
        - set role as 2005 <br />
    * Jane -> executeTrade_GWAL (create additional account in UI) <br />
        - set type as 2 <br />
        - set role as 2004 <br />

    *// call roleContract to set* <br />
    * CoreContract <br />
        - set type as 1  <br />
    
    *// create two additional accounts in UI as subMWALs* <br />
    * Sub1 -> subMWAL 1 <br />
    * Sub2 -> subMWAL 2 <br />

    >Some accounts may not have balance yet. Transfer few balance before using the account as caller. 
    
3. **Call create() by Bob**
    * set sstId = 10
    * set class = 2
    * set amount = 1000000
    * set holdingAddr = Alice

Check: <br />
    - call getGwal(Alice) in coreContract, should get [2, 1000000] <br />
    - call checkState() in coreContract, should get [1, 10, 0] <br />
    - call listSSTs() in regtrSSTContract, should get [10] <br />

4. **Call addSettleItem() by Lucy**
    * set sub_mwal = Sub1
    * set class = 2
    * set amount = 400000
    * set last = false

Check: <br />
    - call getSettleItems() in coreContract, should get [[pubkeySub1, 2, 400000]] <br />
    - call checkState() in coreContract, should get [1, 10, 0] <br />

5. **Call addSettleItem() by Lucy**
    * set sub_mwal = Sub2
    * set class = 2
    * set amount = 600000
    * set last = true

Check: <br />
    - call getSettleItems() in coreContract, should get [[pubkeySub1, 2, 400000], [pubkeySub2, 2, 600000]] <br />
    - call checkState() in coreContract, should get [2, 10, 0] <br />

6. **Call settleMWALs() by Lucy**
    * set _sessionId = 1

Check: <br />
    - call getSettlements(1) in coreContract, should get [[pubkeySub1, 2, 400000], [pubkeySub2, 2, 600000]] <br />
    - call checkState() in coreContract, should get [3, 10, 0] <br />

7. **Call distribute() by Charlie**
    * set sub_mwal = Sub1
    * set class = 2
    * set amount = 400000

Check: <br />
    - call getDistribution() in coreContract, should get [[pubkeySub1, 2, 400000]] <br />

8. **Call distribute() by Charlie**
    * set sub_mwal = Sub2
    * set class = 2
    * set amount = 600000

Check: <br />
    - call getDistribution() in coreContract, should get [[pubkeySub1, 2, 400000], [pubkeySub2, 2, 600000]] <br />
    - call checkState() in coreContract, should get [4, 10, 0] <br />

9. **Call transferToOther() by Eve**
    * set sstId = 10
    * set class = 2
    * set amount = 400000
    * set to = Sub1
    * set from = Alice

Check:  <br />
    - call getPendingTransfer(0) in coreContract, should get [0, 0, 2, 400000, pubkeySub1, pubkeyAlice] <br />
    - call checkState() in coreContract, should get [5, 10, 1] <br />

10. **Call approveTransferToOther() by Eve**
    * set ptId = 0
    * set func_name = 2008
    * set stage = 0

Check: <br />
    - call checkPendingConfirmations(0) in coreContract, should get 2 <br />
    - call checkConfirmation(0) in coreContract, should get [[2008, 0, pubkeyEve]] <br />

11. **Call approveTransferToOther() by Ferdie**
    * set ptId = 0
    * set func_name = 2009
    * set stage = 1

Check: <br />
    - call checkPendingConfirmations(0) in coreContract, should get 1 <br />
    - call checkConfirmation(0) in coreContract, should get [[2009, 1, pubkeyFerdie], [2008, 0, pubkeyEve]] <br />

12. **Call approveTransferToOther() by Cindy**
    * set ptId = 0
    * set func_name = 2010
    * set stage = 2

Check: <br />
    - call checkPendingConfirmations(0) in coreContract, should get 0 <br />
    - call checkConfirmation(0) in coreContract, should get [[2010, 2, pubkeyCindy], [2009, 1, pubkeyFerdie], [2008, 0, pubkeyEve]] <br />
    - call checkState() in coreContract, should get [6, 10, 1] <br />
    - call getGwal(Sub1) in coreContract, should get [2, 400000] <br />
    - call getGwal(Alice) in coreContract, should get [2, 600000] <br />

13. **Call transferToOther() by Eve**
    * set sstId = 10
    * set class = 2
    * set amount = 600000
    * set to = Sub2
    * set from = Alice

Check:  <br />
    - call getPendingTransfer(1) in coreContract, should get [1, 0, 2, 600000, pubkeySub2, pubkeyAlice] <br />
    - call checkState() in coreContract, should get [5, 10, 2] <br />

10. **Call approveTransferToOther() by Eve**
    * set ptId = 0
    * set func_name = 2008
    * set stage = 0

Check: <br />
    - call checkPendingConfirmations(0) in coreContract, should get 2 <br />
    - call checkConfirmation(0) in coreContract, should get [[2008, 0, pubkeyEve]] <br />

11. **Call approveTransferToOther() by Ferdie**
    * set ptId = 0
    * set func_name = 2009
    * set stage = 1

Check: <br />
    - call checkPendingConfirmations(0) in coreContract, should get 1 <br />
    - call checkConfirmation(0) in coreContract, should get [[2009, 1, pubkeyFerdie], [2008, 0, pubkeyEve]] <br />

12. **Call approveTransferToOther() by Cindy**
    * set ptId = 0
    * set func_name = 2010
    * set stage = 2

Check: <br />
    - call checkPendingConfirmations(0) in coreContract, should get 0 <br />
    - call checkConfirmation(0) in coreContract, should get [[2010, 2, pubkeyCindy], [2009, 1, pubkeyFerdie], [2008, 0, pubkeyEve]] <br />
    - call checkState() in coreContract, should get [6, 10, 2] <br />
    - call getGwal(Sub2) in coreContract, should get [2, 600000] <br />
    - call getGwal(Alice) in coreContract, should get [2, 0] <br />

13. **Call executeTrade() by Jane**
    * set _sessionId = 1
    * set sstId = 10
    * set _class = 2
    * set _amount = 1000000
    * set timestampe = 1594997042
    * set tradeType = 0


#### 2nd round new SST creation
> Before 2nd round of new SST creation, call reset() in coreContract

Check: Call checkState() in coreContract, should get [0, 0, 2]

Follow the same process from **Step 3**

#### Cancel transfer
cancelTransferToOther() can be called before all confirmations are approved

ex. After transferToOther(),  <br />
* approveTransferToOther() -> cancelTransferToOther()
* approveTransferToOther() -> approveTransferToOther() -> cancelTransferToOther()
* approveTransferToOther() -> approveTransferToOther() -> approveTransferToOther() -> X (Cannot cancel)
