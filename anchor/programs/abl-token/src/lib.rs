use anchor_lang::prelude::*;
use spl_transfer_hook_interface::instruction::ExecuteInstruction;
use spl_discriminator::SplDiscriminate;

pub mod instructions;
pub mod state;
pub mod errors;
pub use instructions::*;
pub use state::*;
pub use errors::*;


declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

#[program]
pub mod abl_token {

    use super::*;

    pub fn init_mint(ctx: Context<Initialize>, args: InitializeMintArgs) -> Result<()> {
        ctx.accounts.init_mint(args)
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn tx_hook(ctx: Context<TxHook>, amount: u64) -> Result<()> {
        ctx.accounts.tx_hook(amount)
    }

    pub fn init_wallet(ctx: Context<InitWallet>, args: InitWalletArgs) -> Result<()> {
        ctx.accounts.init_wallet(args)
    }

    pub fn remove_wallet(ctx: Context<RemoveWallet>) -> Result<()> {
        ctx.accounts.remove_wallet()
    }

    pub fn change_mode(ctx: Context<ChangeMode>, args: ChangeModeArgs) -> Result<()> {
        ctx.accounts.change_mode(args)
    }
}


