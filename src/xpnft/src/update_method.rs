use candid::candid_method;
use ic_cdk_macros::update;

use crate::{
    errors::{ApprovalError, TransferError},
    state::{Token, COLLECTION},
    types::{ApprovalArgs, MintArgs, TransferArgs},
    utils::account_transformer,
};

#[update]
#[candid_method(update)]
pub fn icrc7_transfer(arg: TransferArgs) -> Result<u128, TransferError> {
    let caller = ic_cdk::caller();
    COLLECTION.with(|c| {
        let mut c = c.borrow_mut();
        c.transfer(&caller, arg)
    })
}

#[update]
#[candid_method(update)]
pub fn icrc7_approve(arg: ApprovalArgs) -> Result<u128, ApprovalError> {
    let caller = ic_cdk::caller();
    COLLECTION.with(|c| {
        let mut c = c.borrow_mut();
        c.approve(&caller, arg)
    })
}

#[update]
#[candid_method(update)]
pub fn icrc7_mint(arg: MintArgs) -> u128 {
    let caller = ic_cdk::caller();
    let owner = account_transformer(arg.to);
    COLLECTION.with(|c| {
        let mut c = c.borrow_mut();
        let token = Token {
            id: arg.id,
            name: arg.name,
            description: arg.description,
            image: arg.image,
            owner,
            approvals: Vec::new(),
            meta: arg.xp_meta,
        };
        c.mint(&caller, token)
    })
}


#[update]
#[candid_method(update)]
pub fn icrc7_burn(token_id: u128) -> u128 {
    let caller = ic_cdk::caller();

    COLLECTION.with(|c| {
        let mut c = c.borrow_mut();
        c.burn(&caller, &token_id)
    })
}
