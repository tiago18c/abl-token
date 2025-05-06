use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface::{spl_token_metadata_interface::state::Field, token_metadata_update_field, Mint, TokenMetadataUpdateField}};

use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed,
};

use crate::{Config, Mode};

#[derive(Accounts)]
#[instruction(args: InitializeMintArgs)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        signer,
        payer = payer,
        mint::token_program = token_program,
        mint::decimals = args.decimals,
        mint::authority = args.mint_authority,
        mint::freeze_authority = args.freeze_authority,
        extensions::permanent_delegate::delegate = args.permanent_delegate,
        extensions::transfer_hook::authority = config.key(),
        extensions::transfer_hook::program_id = crate::id(),
        extensions::metadata_pointer::authority = config.key(),
        extensions::metadata_pointer::metadata_address = mint.key(),
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = payer,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token2022>,
}

impl<'info> Initialize<'info> {
    pub fn init_mint(&mut self, args: InitializeMintArgs) -> Result<()> {

        let config = &mut self.config;
        config.authority = self.mint.key();

        let cpi_accounts = TokenMetadataUpdateField {
            metadata: self.mint.to_account_info(),
            update_authority: self.config.to_account_info(),
            program_id: self.token_program.to_account_info(),
        };
        let seeds = &[b"config".as_ref(), &[self.config.bump]];
        let signer_seeds = &[&seeds[..]];
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        token_metadata_update_field(
            cpi_ctx,
            Field::Key("AB".to_string()),
            args.mode.to_string(),
        )?;

        if args.mode == Mode::Mixed {
            
            let cpi_accounts = TokenMetadataUpdateField {
                metadata: self.mint.to_account_info(),
                update_authority: self.config.to_account_info(),
                program_id: self.token_program.to_account_info(),
            };
            let cpi_program = self.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            
            token_metadata_update_field(
                cpi_ctx,
                Field::Key("threshold".to_string()),
                args.threshold.to_string(),
            )?;
        }

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeMintArgs {
    pub decimals: u8,
    pub mint_authority: Pubkey,
    pub freeze_authority: Pubkey,
    pub permanent_delegate: Pubkey,
    pub mode: Mode,
    pub threshold: u64,
}


pub fn get_extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
    Ok(vec![
        // [5] ab_wallet for destination token account wallet
        ExtraAccountMeta::new_with_seeds(&[
            Seed::Literal { bytes: "ab_wallet".as_bytes().to_vec() },
            Seed::AccountData { account_index: 2, data_index: 32, length: 32 }], false, true)?, // [2] destination token account
        
    ])
}
