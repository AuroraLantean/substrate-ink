#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
mod staking_contract {
    use openbrush::contracts::psp22::*;

    #[ink(storage)]
    pub struct StakingContract {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl StakingContract {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
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

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
    }
}
