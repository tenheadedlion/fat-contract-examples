#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use pink_extension as pink;

#[pink::contract(env=PinkEnvironment)]
mod logging {
    use super::pink;
    use pink::PinkEnvironment;

    #[ink(storage)]
    pub struct Logging {}

    impl Logging {
        #[ink(constructor)]
        pub fn default() -> Self {
            pink::error!("instantiated");
            Self {}
        }

        #[ink(message)]
        pub fn test(&self) {
            pink::error!("a test message received");
            pink::warn!("test end");
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn log_works() {
            env_logger::init();
            pink_extension_runtime::mock_ext::mock_all_ext();

            Logging::default().test()
        }
    }
}
