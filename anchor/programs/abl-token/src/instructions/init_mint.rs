use anchor_lang::{
    prelude::*, solana_program::program::invoke, solana_program::system_instruction::transfer,
};
use anchor_spl::{
    token_2022::Token2022,
    token_interface::{
        spl_token_metadata_interface::state::Field, token_metadata_initialize,
        token_metadata_update_field, Mint, TokenMetadataInitialize, TokenMetadataUpdateField,
    },
};

use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

use crate::{Config, Mode};

pub const META_LIST_ACCOUNT_SEED: &[u8] = b"extra-account-metas";

#[derive(Accounts)]
#[instruction(args: InitMintArgs)]
pub struct InitMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        signer,
        payer = payer,
        mint::token_program = token_program,
        mint::decimals = args.decimals,
        mint::authority = config.key(),
        mint::freeze_authority = config.key(),
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

    #[account(
        init,
        space = get_meta_list_size()?,
        seeds = [META_LIST_ACCOUNT_SEED, mint.key().as_ref()],
        bump,
        payer = payer,
    )]
    /// CHECK: extra metas account
    pub extra_metas_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token2022>,
}

impl InitMint<'_> {
    pub fn init_mint(&mut self, args: InitMintArgs, config_bump: u8) -> Result<()> {
        let config = &mut self.config;
        config.authority = self.mint.key();
        config.bump = config_bump;

        let seeds = &[b"config".as_ref(), &[self.config.bump]];
        let signer_seeds = &[&seeds[..]];
        //let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TokenMetadataInitialize {
            program_id: self.token_program.to_account_info(),
            mint: self.mint.to_account_info(),
            metadata: self.mint.to_account_info(), // metadata account is the mint, since data is stored in mint
            mint_authority: self.config.to_account_info(),
            update_authority: self.config.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        token_metadata_initialize(cpi_ctx, args.name, args.symbol, args.uri)?;

        let cpi_accounts = TokenMetadataUpdateField {
            metadata: self.mint.to_account_info(),
            update_authority: self.config.to_account_info(),
            program_id: self.token_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        token_metadata_update_field(cpi_ctx, Field::Key("AB".to_string()), args.mode.to_string())?;

        if args.mode == Mode::Mixed {
            let cpi_accounts = TokenMetadataUpdateField {
                metadata: self.mint.to_account_info(),
                update_authority: self.config.to_account_info(),
                program_id: self.token_program.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
            );

            token_metadata_update_field(
                cpi_ctx,
                Field::Key("threshold".to_string()),
                args.threshold.to_string(),
            )?;
        }

        let data = self.mint.to_account_info().data_len();
        let min_balance = Rent::get()?.minimum_balance(data);
        if min_balance > self.mint.to_account_info().get_lamports() {
            invoke(
                &transfer(
                    &self.payer.key(),
                    &self.mint.to_account_info().key(),
                    min_balance - self.mint.to_account_info().get_lamports(),
                ),
                &[
                    self.payer.to_account_info(),
                    self.mint.to_account_info(),
                    self.system_program.to_account_info(),
                ],
            )?;
        }

        // initialize the extra metas account
        let extra_metas_account = &self.extra_metas_account;
        let metas = get_extra_account_metas()?;
        let mut data = extra_metas_account.try_borrow_mut_data()?;
        ExtraAccountMetaList::init::<ExecuteInstruction>(&mut data, &metas)?;

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitMintArgs {
    pub decimals: u8,
    pub mint_authority: Pubkey,
    pub freeze_authority: Pubkey,
    pub permanent_delegate: Pubkey,
    pub mode: Mode,
    pub threshold: u64,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

fn get_meta_list_size() -> Result<usize> {
    Ok(ExtraAccountMetaList::size_of(1).unwrap())
}

fn get_extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
    Ok(vec![
        // [5] ab_wallet for destination token account wallet
        ExtraAccountMeta::new_with_seeds(
            &[
                Seed::Literal {
                    bytes: "ab_wallet".as_bytes().to_vec(),
                },
                Seed::AccountData {
                    account_index: 2,
                    data_index: 32,
                    length: 32,
                },
            ],
            false,
            true,
        )?, // [2] destination token account
    ])
}
