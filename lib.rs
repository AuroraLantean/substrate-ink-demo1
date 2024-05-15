#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod flipper {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    //use ink::env::account_id;
    //use ink::primitives::AccountId;
    pub enum Status {
        NotStarted,
        Open,
        Cancelled,
    }
    pub struct Auction {
        name: String,
        subject: Hash,
        status: Status,
        finalized: bool,
        vector: Vec<u8>,
    }
    #[ink(event)]
    pub struct Transferred {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        amount: Balance,
    }
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
    }
    #[ink(storage)]
    pub struct Flipper {
        boo: bool,
        num: u32,
        name: String,
        //account: AccountId,
        balance: Balance,
        //auctions: Vec<Auction>,
    }
    impl Flipper {
        #[ink(constructor)]
        pub fn new(boo: bool, num: u32, name: String, balance: Balance) -> Self {
            let caller = Self::env().caller();
            ink::env::debug_println!("new(): caller = {:?}", caller);
            // Self::env().emit_event(Transferred {
            //     from: None,
            //     to: Some(caller),
            //     amount: num.into()
            // });
            Self { boo, num, name, balance}
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(false, 0, "name1".to_string(), 0)
        }//Default::default()

        #[ink(message)]
        pub fn flip(&mut self) {
            ink::env::debug_println!("flip00");
            //https://docs.rs/ink_env/5.0.0/ink_env/#functions
            let caller = self.env().caller();
            let account = self.env().account_id();
            let balance = self.env().balance();
            ink::env::debug_println!("flip01");

            let block_number = self.env().block_number();
            ink::env::debug_println!("flip02");
            let time_stamp = self.env().block_timestamp();
            ink::env::debug_println!("flip03");

            ink::env::debug_println!("caller: {:?}, account: {:?}", caller, account);
            ink::env::debug_println!("balance: {:?}", balance);
            ink::env::debug_println!("block_number: {:?}, time_stamp: {:?}", block_number, time_stamp);
            
            ink::env::debug_println!("magic bool: {}", self.boo);
            self.boo = !self.boo;
        }
        #[ink(message)]
        pub fn set_num(&mut self, new_value: u32) {
            ink::env::debug_println!("magic number: {}", self.num);
            self.num = new_value;
        }

        #[ink(message)]
        pub fn get_bool(&self) -> bool {
            ink::env::debug_println!("get_bool: {}", self.boo);
            self.boo
        }
        #[ink(message)]
        pub fn get_all(&self) -> (bool, u32, String) {
            (self.boo, self.num, self.name.clone())
        }
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: Balance) -> Result<(),()> {
            let is_caller_origin = self.env().caller_is_origin();
            ink::env::debug_println!("is_caller_origin: {:?}",  is_caller_origin);
            let from = self.env().caller();
            // implementation hidden
            self.env().emit_event(Transferred {
                from: Some(from),
                to: Some(to),
                amount
            });
            Ok(())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get_bool(), false);
            assert_eq!(flipper.get_all(), (false, 0, "name1".to_string()));
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false, 0, "name1".to_string(), 0);
            assert_eq!(flipper.get_bool(), false);
            flipper.flip();
            assert_eq!(flipper.get_bool(), true);
        }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::ContractsBackend;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = FlipperRef::default();

            // When
            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<Flipper>();

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a boo from the on-chain contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = FlipperRef::new(false);
            let contract = client
                .instantiate("flipper", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Flipper>();

            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = call_builder.flip();
            let _flip_result = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
