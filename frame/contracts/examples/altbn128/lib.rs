#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod altbn128 {
    use ink_env::zk_snarks::AltBn128;
    use ink_prelude::string::String;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Altbn128 {
        /// Stores a single `String` value on the storage instead of using [u8; 64].
        value: String,
    }

    impl Altbn128 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: String) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new("hello, world".into())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn bn_256_add(&mut self) {
            let mut result = [0; 64];
            ink_env::inflect_add::<AltBn128>(&[], &[], &mut result);
            self.value = ink_prelude::format!("0x{:x?}", result);
        }

        // /// A message that can be called on instantiated contracts.
        // /// This one flips the value of the stored `bool` from `true`
        // /// to `false` and vice versa.
        // #[ink(message)]
        // pub fn flip(&mut self) {
        //     self.value = !self.value;
        // }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> String {
            ink_prelude::format!("{}", &self.value)
        }
    }

    // /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    // /// module and test functions are marked with a `#[test]` attribute.
    // /// The below code is technically just normal Rust code.
    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;
    //
    //     /// We test if the default constructor does its job.
    //     #[test]
    //     fn default_works() {
    //         let altbn128 = AltBn128::default();
    //         assert_eq!(altbn128.get(), false);
    //     }
    //
    //     /// We test a simple use case of our contract.
    //     #[test]
    //     fn it_works() {
    //         let mut altbn128 = AltBn128::new(false);
    //         assert_eq!(altbn128.get(), false);
    //         altbn128.flip();
    //         assert_eq!(altbn128.get(), true);
    //     }
    // }
}
