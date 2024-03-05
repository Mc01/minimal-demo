// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
pub mod demo_storage {
    /// @dev Imports
    use ink::prelude::{string::String, vec::Vec};
    use openbrush::{storage::Mapping, traits::Storage};

    /// @dev Custom types
    pub type EthAddress = [u8; 20];
    pub type StringBytes = Vec<u8>;

    /// @dev Custom errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum StorageError {
        TestAlreadyExists,
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
    pub struct StorageData {
        pub nested_mapping: Mapping<StringBytes, Option<TestStruct>>,
    }

    /// @dev Storage
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct DemoStorage {
        // data used in demo
        #[storage_field]
        storage_data: StorageData,
    }

    /// @dev Implementation
    impl DemoStorage {
        // -----------------------------------------------------------------------
        //                              Constructor
        // -----------------------------------------------------------------------

        #[ink(constructor)]
        pub fn new() -> Self {
            // get default constructor
            let mut instance_ = Self::default();

            // init mappings
            instance_.storage_data.nested_mapping = Mapping::default();

            // return
            instance_
        }

        // -----------------------------------------------------------------------
        //                              Test
        // -----------------------------------------------------------------------

        #[ink(message)]
        pub fn add_test(&mut self, _test: TestStruct) -> Result<(), StorageError> {
            // get byte-string
            let test_id_: StringBytes = _test.test_id.clone();

            // verify if test does not exists
            if !self
                .storage_data
                .nested_mapping
                .get(&test_id_.clone())
                .unwrap_or(None)
                .is_none()
            {
                return Err(StorageError::TestAlreadyExists);
            }

            // save test into mapping
            self.storage_data
                .nested_mapping
                .insert(&test_id_, &Some(_test));

            // return
            Ok(())
        }

        #[ink(message)]
        pub fn check_test(&self, _test_id: String) -> bool {
            // return
            !self.get_test_data(_test_id).is_none()
        }

        #[ink(message)]
        pub fn get_test_data(&self, _test_id: String) -> Option<TestStruct> {
            // return
            self.storage_data
                .nested_mapping
                .get(&_test_id.into_bytes())
                .unwrap_or(None)
        }
    }
}
