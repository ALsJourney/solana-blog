use anchor_lang::prelude::*;
use std::mem::size_of;

mod states;
use states::*;

mod constant;
use constant::BLOG_SEED;
use constant::POST_SEED;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod blog {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let blog = &mut ctx.accounts.blog;
        blog.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn create_post(ctx: Context<CreatePost>, title: String, content: String) -> Result<()> {
        let blog = &mut ctx.accounts.blog;
        let post = &mut ctx.accounts.post;

        post.previous = blog.latest;
        post.blog = blog.key();
        post.title = title;
        post.content = content;
        post.timestamp = Clock::get()?.unix_timestamp;

        blog.latest = post.key();
        blog.posts += 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, seeds = [BLOG_SEED, authority.key().as_ref()], bump, payer = authority, space = 8 + size_of::<Blog>())]
    pub blog: Account<'info, Blog>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String, content: String)]
pub struct CreatePost<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, has_one = authority)]
    pub blog: Account<'info, Blog>,

    #[account(init, seeds = [POST_SEED, blog.key().as_ref()], bump, payer = authority, space = 8 + size_of::<Post>() + title.as_bytes().len() + content.as_bytes().len() - 40)]
    pub post: Account<'info, Post>,
    pub system_program: Program<'info, System>,
}
