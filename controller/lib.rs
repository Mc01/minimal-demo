// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
pub mod demo_controller {
    /// @dev Imports
    use ink::prelude::vec::Vec;
    use openbrush::traits::Storage;

    /// @dev Custom types
    pub type SelectorBytes = [u8; 4];
    pub type EthAddress = [u8; 20];
    pub type StringBytes = Vec<u8>;

    /// @dev Custom errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ControllerError {
        NotUsed,
    }

    /// @dev Custom structs
    #[derive(scale::Encode, scale::Decode, Clone, Debug)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct TestStruct {
        pub test_id: StringBytes,
        pub az_address: AccountId,
        pub az_o_address: Option<AccountId>,
        pub eth_address: EthAddress,
        pub eth_o_address: Option<EthAddress>,
    }

    /// @dev Data
    #[derive(Default, Debug)]
    #[openbrush::storage_item]
    pub struct ControllerData {
        pub data: bool,
    }

    /// @dev Storage
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct DemoController {
        // not used field
        #[storage_field]
        controller_data: ControllerData,
    }

    /// @dev Implementation
    impl DemoController {
        // -----------------------------------------------------------------------
        //                              Constructor
        // -----------------------------------------------------------------------

        #[ink(constructor)]
        pub fn new() -> Self {
            // return
            Self::default()
        }

        // -----------------------------------------------------------------------
        //                              Raise
        // -----------------------------------------------------------------------

        #[ink(message)]
        pub fn get_selector_invoke_add_test(&self) -> SelectorBytes {
            ink::selector_bytes!("invoke_add_test")
        }

        #[ink(message)]
        pub fn invoke_add_test(
            &mut self,
            _storage: AccountId,
            _test: TestStruct,
        ) -> Result<(), ControllerError> {
            // parameters
            let add_test_selector_ =
                ink::env::call::Selector::new(ink::selector_bytes!("add_test"));

            // add raise to storage
            ink::env::call::build_call::<ink::env::DefaultEnvironment>()
                .call(_storage) // storage address
                .gas_limit(0) // no gas limit
                .transferred_value(0) // no value
                .exec_input(
                    ink::env::call::ExecutionInput::new(add_test_selector_).push_arg(&_test),
                )
                .call_flags(ink::env::CallFlags::default())
                .returns::<()>()
                .invoke();

            // return
            Ok(())
        }
    }
}
