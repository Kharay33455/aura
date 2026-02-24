use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("BZKVHMBN2nvHfmpr3BC4ksQQBSf798Wyi26SaRrbBWME");
pub const PROG_OWNER: Pubkey = pubkey!("B2NrSbo6TCS3x5kxX5Jb1iNB3dyNzWPooR4gnE9342PB");

#[program]
pub mod aura {
    use super::*;
    pub fn do_stuff(ctx: Context<DoStuff>, amount: u64) -> Result<()> {
        let aura: &Account<'_, Aura> = &ctx.accounts.aura;
        let system_program: &Program<'_, System> = &ctx.accounts.system_program;
        let interactor: &Signer<'_> = &ctx.accounts.interactor;
        let prog_owner: &AccountInfo<'_> = &ctx.accounts.prog_owner;

        if aura.aura_value == 0 {
            transfer_sol(
                prog_owner.to_account_info(),
                interactor.to_account_info(),
                system_program.to_account_info(),
                amount,
            )?;
        } else {
            transfer_sol(
                interactor.to_account_info(),
                prog_owner.to_account_info(),
                system_program.to_account_info(),
                amount,
            )?;
        }

        Ok(())
    }
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let aura: &mut Account<'_, Aura> = &mut ctx.accounts.aura;
        aura.aura_value = 0;
        Ok(())
    }

    pub fn update_aura(ctx: Context<UpdateAura>, new_aura: u64)->Result<()>{
        let aura: &mut Account<'_, Aura> = &mut ctx.accounts.aura;
        aura.aura_value = new_aura;
        Ok(())
    }
}

pub fn transfer_sol<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    program_id: AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let cpi = CpiContext::new(program_id, Transfer { from, to });

    transfer(cpi, amount)?;
    Ok(())
}

#[account]
#[derive(InitSpace)]
pub struct Aura {
    pub aura_value: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, space = 8 + Aura::INIT_SPACE, payer = signer, seeds=[b"aura"], bump)]
    pub aura: Account<'info, Aura>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAura<'info> {
    pub aura: Account<'info, Aura>,
}

#[derive(Accounts)]
pub struct DoStuff<'info> {
    #[account(mut)]
    pub interactor: Signer<'info>,
    pub aura: Account<'info, Aura>,
    /// CHECK: we'll handle this check ourselves as struct doesn't exist in program
    #[account(mut)]
    pub prog_owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum AuraError {
    #[msg("Not auth")]
    NotAuth,
}
