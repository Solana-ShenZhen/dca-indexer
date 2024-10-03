//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Transfer {
      
              
          pub keeper: solana_program::pubkey::Pubkey,
          
              
          pub dca: solana_program::pubkey::Pubkey,
          
              
          pub user: solana_program::pubkey::Pubkey,
          
              
          pub output_mint: solana_program::pubkey::Pubkey,
          
              
          pub dca_out_ata: solana_program::pubkey::Pubkey,
          
              
          pub user_out_ata: Option<solana_program::pubkey::Pubkey>,
          
              
          pub intermediate_account: Option<solana_program::pubkey::Pubkey>,
          
              
          pub system_program: solana_program::pubkey::Pubkey,
          
              
          pub token_program: solana_program::pubkey::Pubkey,
          
              
          pub associated_token_program: solana_program::pubkey::Pubkey,
          
              
          pub event_authority: solana_program::pubkey::Pubkey,
          
              
          pub program: solana_program::pubkey::Pubkey,
      }

impl Transfer {
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(&[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(12 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            self.keeper,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.dca,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.user,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.output_mint,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.dca_out_ata,
            false
          ));
                                                      if let Some(user_out_ata) = self.user_out_ata {
              accounts.push(solana_program::instruction::AccountMeta::new(
                user_out_ata,
                false,
              ));
            } else {
              accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::DCA_ID,
                false,
              ));
            }
                                                                if let Some(intermediate_account) = self.intermediate_account {
              accounts.push(solana_program::instruction::AccountMeta::new(
                intermediate_account,
                false,
              ));
            } else {
              accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::DCA_ID,
                false,
              ));
            }
                                                    accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.associated_token_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.event_authority,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.program,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let data = TransferInstructionData::new().try_to_vec().unwrap();
    
    solana_program::instruction::Instruction {
      program_id: crate::DCA_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TransferInstructionData {
            discriminator: [u8; 8],
      }

impl TransferInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [163, 52, 200, 231, 140, 3, 69, 186],
                  }
  }
}

impl Default for TransferInstructionData {
  fn default() -> Self {
    Self::new()
  }
}



/// Instruction builder for `Transfer`.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` keeper
                ///   1. `[writable]` dca
                ///   2. `[writable]` user
          ///   3. `[]` output_mint
                ///   4. `[writable]` dca_out_ata
                      ///   5. `[writable, optional]` user_out_ata
                      ///   6. `[writable, optional]` intermediate_account
          ///   7. `[]` system_program
          ///   8. `[]` token_program
          ///   9. `[]` associated_token_program
          ///   10. `[]` event_authority
          ///   11. `[]` program
#[derive(Clone, Debug, Default)]
pub struct TransferBuilder {
            keeper: Option<solana_program::pubkey::Pubkey>,
                dca: Option<solana_program::pubkey::Pubkey>,
                user: Option<solana_program::pubkey::Pubkey>,
                output_mint: Option<solana_program::pubkey::Pubkey>,
                dca_out_ata: Option<solana_program::pubkey::Pubkey>,
                user_out_ata: Option<solana_program::pubkey::Pubkey>,
                intermediate_account: Option<solana_program::pubkey::Pubkey>,
                system_program: Option<solana_program::pubkey::Pubkey>,
                token_program: Option<solana_program::pubkey::Pubkey>,
                associated_token_program: Option<solana_program::pubkey::Pubkey>,
                event_authority: Option<solana_program::pubkey::Pubkey>,
                program: Option<solana_program::pubkey::Pubkey>,
                __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl TransferBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            #[inline(always)]
    pub fn keeper(&mut self, keeper: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.keeper = Some(keeper);
                    self
    }
            #[inline(always)]
    pub fn dca(&mut self, dca: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.dca = Some(dca);
                    self
    }
            #[inline(always)]
    pub fn user(&mut self, user: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.user = Some(user);
                    self
    }
            #[inline(always)]
    pub fn output_mint(&mut self, output_mint: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.output_mint = Some(output_mint);
                    self
    }
            #[inline(always)]
    pub fn dca_out_ata(&mut self, dca_out_ata: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.dca_out_ata = Some(dca_out_ata);
                    self
    }
            /// `[optional account]`
#[inline(always)]
    pub fn user_out_ata(&mut self, user_out_ata: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
                        self.user_out_ata = user_out_ata;
                    self
    }
            /// `[optional account]`
#[inline(always)]
    pub fn intermediate_account(&mut self, intermediate_account: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
                        self.intermediate_account = intermediate_account;
                    self
    }
            #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.system_program = Some(system_program);
                    self
    }
            #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_program = Some(token_program);
                    self
    }
            #[inline(always)]
    pub fn associated_token_program(&mut self, associated_token_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.associated_token_program = Some(associated_token_program);
                    self
    }
            #[inline(always)]
    pub fn event_authority(&mut self, event_authority: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.event_authority = Some(event_authority);
                    self
    }
            #[inline(always)]
    pub fn program(&mut self, program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.program = Some(program);
                    self
    }
            /// Add an aditional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: solana_program::instruction::AccountMeta) -> &mut Self {
    self.__remaining_accounts.push(account);
    self
  }
  /// Add additional accounts to the instruction.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[solana_program::instruction::AccountMeta]) -> &mut Self {
    self.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[allow(clippy::clone_on_copy)]
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    let accounts = Transfer {
                              keeper: self.keeper.expect("keeper is not set"),
                                        dca: self.dca.expect("dca is not set"),
                                        user: self.user.expect("user is not set"),
                                        output_mint: self.output_mint.expect("output_mint is not set"),
                                        dca_out_ata: self.dca_out_ata.expect("dca_out_ata is not set"),
                                        user_out_ata: self.user_out_ata,
                                        intermediate_account: self.intermediate_account,
                                        system_program: self.system_program.expect("system_program is not set"),
                                        token_program: self.token_program.expect("token_program is not set"),
                                        associated_token_program: self.associated_token_program.expect("associated_token_program is not set"),
                                        event_authority: self.event_authority.expect("event_authority is not set"),
                                        program: self.program.expect("program is not set"),
                      };
    
    accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
  }
}

  /// `transfer` CPI accounts.
  pub struct TransferCpiAccounts<'a, 'b> {
          
                    
              pub keeper: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub dca: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub user: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub output_mint: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub dca_out_ata: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub user_out_ata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                
                    
              pub intermediate_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                
                    
              pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub program: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `transfer` CPI instruction.
pub struct TransferCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub keeper: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub dca: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub user: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub output_mint: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub dca_out_ata: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub user_out_ata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
          
              
          pub intermediate_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
          
              
          pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub program: &'b solana_program::account_info::AccountInfo<'a>,
        }

impl<'a, 'b> TransferCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: TransferCpiAccounts<'a, 'b>,
          ) -> Self {
    Self {
      __program: program,
              keeper: accounts.keeper,
              dca: accounts.dca,
              user: accounts.user,
              output_mint: accounts.output_mint,
              dca_out_ata: accounts.dca_out_ata,
              user_out_ata: accounts.user_out_ata,
              intermediate_account: accounts.intermediate_account,
              system_program: accounts.system_program,
              token_program: accounts.token_program,
              associated_token_program: accounts.associated_token_program,
              event_authority: accounts.event_authority,
              program: accounts.program,
                }
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], &[])
  }
  #[inline(always)]
  pub fn invoke_with_remaining_accounts(&self, remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
  }
  #[inline(always)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed_with_remaining_accounts(
    &self,
    signers_seeds: &[&[&[u8]]],
    remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]
  ) -> solana_program::entrypoint::ProgramResult {
    let mut accounts = Vec::with_capacity(12 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            *self.keeper.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.dca.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.output_mint.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.dca_out_ata.key,
            false
          ));
                                          if let Some(user_out_ata) = self.user_out_ata {
            accounts.push(solana_program::instruction::AccountMeta::new(
              *user_out_ata.key,
              false,
            ));
          } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
              crate::DCA_ID,
              false,
            ));
          }
                                          if let Some(intermediate_account) = self.intermediate_account {
            accounts.push(solana_program::instruction::AccountMeta::new(
              *intermediate_account.key,
              false,
            ));
          } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
              crate::DCA_ID,
              false,
            ));
          }
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.associated_token_program.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.event_authority.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.program.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let data = TransferInstructionData::new().try_to_vec().unwrap();
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::DCA_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(12 + 1 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.keeper.clone());
                        account_infos.push(self.dca.clone());
                        account_infos.push(self.user.clone());
                        account_infos.push(self.output_mint.clone());
                        account_infos.push(self.dca_out_ata.clone());
                        if let Some(user_out_ata) = self.user_out_ata {
          account_infos.push(user_out_ata.clone());
        }
                        if let Some(intermediate_account) = self.intermediate_account {
          account_infos.push(intermediate_account.clone());
        }
                        account_infos.push(self.system_program.clone());
                        account_infos.push(self.token_program.clone());
                        account_infos.push(self.associated_token_program.clone());
                        account_infos.push(self.event_authority.clone());
                        account_infos.push(self.program.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `Transfer` via CPI.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` keeper
                ///   1. `[writable]` dca
                ///   2. `[writable]` user
          ///   3. `[]` output_mint
                ///   4. `[writable]` dca_out_ata
                      ///   5. `[writable, optional]` user_out_ata
                      ///   6. `[writable, optional]` intermediate_account
          ///   7. `[]` system_program
          ///   8. `[]` token_program
          ///   9. `[]` associated_token_program
          ///   10. `[]` event_authority
          ///   11. `[]` program
#[derive(Clone, Debug)]
pub struct TransferCpiBuilder<'a, 'b> {
  instruction: Box<TransferCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> TransferCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(TransferCpiBuilderInstruction {
      __program: program,
              keeper: None,
              dca: None,
              user: None,
              output_mint: None,
              dca_out_ata: None,
              user_out_ata: None,
              intermediate_account: None,
              system_program: None,
              token_program: None,
              associated_token_program: None,
              event_authority: None,
              program: None,
                                __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn keeper(&mut self, keeper: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.keeper = Some(keeper);
                    self
    }
      #[inline(always)]
    pub fn dca(&mut self, dca: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.dca = Some(dca);
                    self
    }
      #[inline(always)]
    pub fn user(&mut self, user: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.user = Some(user);
                    self
    }
      #[inline(always)]
    pub fn output_mint(&mut self, output_mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.output_mint = Some(output_mint);
                    self
    }
      #[inline(always)]
    pub fn dca_out_ata(&mut self, dca_out_ata: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.dca_out_ata = Some(dca_out_ata);
                    self
    }
      /// `[optional account]`
#[inline(always)]
    pub fn user_out_ata(&mut self, user_out_ata: Option<&'b solana_program::account_info::AccountInfo<'a>>) -> &mut Self {
                        self.instruction.user_out_ata = user_out_ata;
                    self
    }
      /// `[optional account]`
#[inline(always)]
    pub fn intermediate_account(&mut self, intermediate_account: Option<&'b solana_program::account_info::AccountInfo<'a>>) -> &mut Self {
                        self.instruction.intermediate_account = intermediate_account;
                    self
    }
      #[inline(always)]
    pub fn system_program(&mut self, system_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.system_program = Some(system_program);
                    self
    }
      #[inline(always)]
    pub fn token_program(&mut self, token_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_program = Some(token_program);
                    self
    }
      #[inline(always)]
    pub fn associated_token_program(&mut self, associated_token_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.associated_token_program = Some(associated_token_program);
                    self
    }
      #[inline(always)]
    pub fn event_authority(&mut self, event_authority: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.event_authority = Some(event_authority);
                    self
    }
      #[inline(always)]
    pub fn program(&mut self, program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.program = Some(program);
                    self
    }
            /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: &'b solana_program::account_info::AccountInfo<'a>, is_writable: bool, is_signer: bool) -> &mut Self {
    self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
    self
  }
  /// Add additional accounts to the instruction.
  ///
  /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
  /// and a `bool` indicating whether the account is a signer or not.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> &mut Self {
    self.instruction.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed(&[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
        let instruction = TransferCpi {
        __program: self.instruction.__program,
                  
          keeper: self.instruction.keeper.expect("keeper is not set"),
                  
          dca: self.instruction.dca.expect("dca is not set"),
                  
          user: self.instruction.user.expect("user is not set"),
                  
          output_mint: self.instruction.output_mint.expect("output_mint is not set"),
                  
          dca_out_ata: self.instruction.dca_out_ata.expect("dca_out_ata is not set"),
                  
          user_out_ata: self.instruction.user_out_ata,
                  
          intermediate_account: self.instruction.intermediate_account,
                  
          system_program: self.instruction.system_program.expect("system_program is not set"),
                  
          token_program: self.instruction.token_program.expect("token_program is not set"),
                  
          associated_token_program: self.instruction.associated_token_program.expect("associated_token_program is not set"),
                  
          event_authority: self.instruction.event_authority.expect("event_authority is not set"),
                  
          program: self.instruction.program.expect("program is not set"),
                    };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct TransferCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            keeper: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                dca: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                output_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                dca_out_ata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                user_out_ata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                intermediate_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                event_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

