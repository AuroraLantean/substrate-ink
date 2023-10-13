#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22)]
#[openbrush::contract]
mod staking_contract {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StakingContract {
        #[storage_field]
        psp22: psp22::Data,
        value: bool,
    }

    //impl PSP22 for StakingContract {}

    impl StakingContract {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut _instance = Self::default();
            psp22::Internal::_mint_to(&mut _instance, Self::env().caller(), initial_supply)
                .expect("Should mint");
            _instance
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use openbrush::contracts::psp22::PSP22Impl;
        use openbrush::test_utils::*;
        #[ink::test]
        fn constructor_works() {
            let accounts = accounts();
            let mint_amount = 1000_1000_1000;
            let staking_contract = StakingContract::new(mint_amount);
            let alice_balc = PSP22Impl::balance_of(&staking_contract, accounts.alice);
            assert_eq!(alice_balc, mint_amount);
        }

        #[ink::test]
        fn transfer_works() {
            let accounts = accounts();
            let mint_amount = 1000_1000_1000;
            let transfer_amount = 1_000;
            let mut staking_contract = StakingContract::new(mint_amount);

            PSP22Impl::transfer(
                &mut staking_contract,
                accounts.bob,
                transfer_amount,
                Vec::new(),
            )
            .expect("transfer");

            let alice_balc = PSP22Impl::balance_of(&staking_contract, accounts.alice);
            let bob_balc = PSP22Impl::balance_of(&staking_contract, accounts.bob);
            assert_eq!(alice_balc, mint_amount - transfer_amount);
            assert_eq!(bob_balc, transfer_amount);
        }
    }
}
