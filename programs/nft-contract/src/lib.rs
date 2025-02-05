use anchor_lang::prelude::*;

declare_id!("BTMq1LQVA8sDLgjngNkvE4SSNjLHUabX3rHznez5VDeJ");

#[program]
pub mod nft_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}






// use anchor_lang::prelude::*;  
// use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};  

// declare_id!("BTMq1LQVA8sDLgjngNkvE4SSNjLHUabX3rHznez5VDeJ");  

// #[program]  
// pub mod nft_project {  
//     use super::*;  

//     pub fn mint_nft(  
//         ctx: Context<MintNFT>,  
//         uri: String,  
//         title: String,  
//         symbol: String,  
//     ) -> Result<()> {  
//         // Mint the NFT  
//         let cpi_accounts = MintTo {  
//             mint: ctx.accounts.mint.to_account_info(),  
//             to: ctx.accounts.token_account.to_account_info(),  
//             authority: ctx.accounts.mint_authority.to_account_info(),  
//         };  
//         let cpi_program = ctx.accounts.token_program.to_account_info();  
//         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);  
//         token::mint_to(cpi_ctx, 1)?;  

//         // Create metadata for the NFT using Metaplex  
//         let metadata_accounts = mpl_token_metadata::instruction::create_metadata_accounts_v3(  
//             mpl_token_metadata::ID,  
//             ctx.accounts.metadata.key(),  
//             ctx.accounts.mint.key(),  
//             ctx.accounts.mint_authority.key(),  
//             ctx.accounts.payer.key(),  
//             ctx.accounts.mint_authority.key(),  
//             title,  
//             symbol,  
//             uri,  
//             None,  
//             1,  
//             true,  
//             false,  
//             None,  
//             None,  
//             None,  
//         );  

//         // Invoke the Metaplex instruction  
//         anchor_lang::solana_program::program::invoke(  
//             &metadata_accounts,  
//             &[  
//                 ctx.accounts.metadata.to_account_info(),  
//                 ctx.accounts.mint.to_account_info(),  
//                 ctx.accounts.mint_authority.to_account_info(),  
//                 ctx.accounts.payer.to_account_info(),  
//                 ctx.accounts.system_program.to_account_info(),  
//                 ctx.accounts.rent.to_account_info(),  
//             ],  
//         )?;  

//         Ok(())  
//     }  
// }  

// #[derive(Accounts)]  
// pub struct MintNFT<'info> {  
//     #[account(init, payer = payer, mint::decimals = 0, mint::authority = mint_authority)]  
//     pub mint: Account<'info, Mint>,  
//     #[account(init, payer = payer, token::mint = mint, token::authority = mint_authority)]  
//     pub token_account: Account<'info, TokenAccount>,  
//     /// CHECK: This is not dangerous because we don't read or write from this account  
//     pub metadata: UncheckedAccount<'info>,  
//     #[account(mut)]  
//     pub mint_authority: Signer<'info>,  
//     #[account(mut)]  
//     pub payer: Signer<'info>,  
//     pub token_program: Program<'info, Token>,  
//     pub system_program: Program<'info, System>,  
//     pub rent: Sysvar<'info, Rent>,  
// }