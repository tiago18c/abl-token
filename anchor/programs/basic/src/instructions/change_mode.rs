use anchor_lang::prelude::*;
use anchor_spl::{token_2022::{spl_token_2022::extension::{StateWithExtensions, BaseStateWithExtensions},  Token2022, spl_token_2022::state::Mint}, token_interface::{spl_token_metadata_interface::state::Field, token_metadata_update_field,  TokenMetadataUpdateField}};
use anchor_spl::token_interface::spl_token_metadata_interface::state::TokenMetadata;


use crate::{Config, Mode};

#[derive(Accounts)]
pub struct ChangeMode<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
        has_one = authority,
    )]
    pub config: Box<Account<'info, Config>>,

    pub mint: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token2022>,
}


#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ChangeModeArgs {
    pub mode: Mode,
    pub threshold: u64,
}

impl<'info> ChangeMode<'info> {
    pub fn change_mode(&mut self, args: ChangeModeArgs) -> Result<()> {
        
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

        if args.mode == Mode::Mixed || self.has_threshold()? {
            let threshold = if args.mode == Mode::Mixed {
                args.threshold
            } else {
                0
            };
            
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
                threshold.to_string(),
            )?;
        }

        Ok(())
    }

    fn has_threshold(&self) -> Result<bool> {
        let mint_info = self.mint.to_account_info();
        let mint_data = mint_info.data.borrow();
        let mint = StateWithExtensions::<Mint>::unpack(&mint_data)?;
        let metadata = mint.get_variable_len_extension::<TokenMetadata>();
        Ok(metadata.is_ok() && metadata.unwrap().additional_metadata.iter().any(|(key, _)| key == "threshold"))
    }
}

