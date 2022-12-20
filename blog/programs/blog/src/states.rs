use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Blog {
    pub authority: Pubkey,
    pub latest: Pubkey,
    pub posts: u64,
}


#[account]
#[derive(Default)]
pub struct Post {
    pub title: String,
    pub content: String,
    pub timestamp: i64,
    pub blog: Pubkey,
    pub previous: Pubkey,
}

