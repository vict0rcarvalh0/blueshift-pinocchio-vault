use pinocchio::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::find_program_address,
};

pub struct WithdrawAccounts<'a> {
    pub owner: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub bump: u8,
}

impl<'a> TryFrom<&'a [AccountInfo]> for WithdrawAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [owner, vault, _system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Basic Accounts Checks
        if !owner.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if vault.owner() != &pinocchio_system::ID {
            return Err(ProgramError::IncorrectProgramId);
        }

        let (vault_key, bump) = find_program_address(&[b"vault", owner.key().as_ref()], &crate::ID);
        if vault.key() != &vault_key {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(Self {
            owner,
            vault,
            bump,
        })
    }
}

pub struct Withdraw<'a> {
    pub accounts: WithdrawAccounts<'a>,
}

impl<'a> TryFrom<&'a [AccountInfo]> for Withdraw<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = WithdrawAccounts::try_from(accounts)?;
        Ok(Self { accounts })
    }
}

impl<'a> Withdraw<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;

    pub fn process(&self) -> ProgramResult {
        let lamports = *self.accounts.vault.try_borrow_lamports()?;
        *self.accounts.vault.try_borrow_mut_lamports()? = 0;
        *self.accounts.owner.try_borrow_mut_lamports()? += lamports;
        Ok(())
    }
}