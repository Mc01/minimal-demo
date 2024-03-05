// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
pub mod demo_emitter {
    /// @dev Imports
    use ink::prelude::{string::String, vec::Vec};
    use openbrush::traits::Storage;

    /// @dev Custom types
    pub type SelectorBytes = [u8; 4];
    pub type StringBytes = Vec<u8>;

    /// @dev Custom wrappers for passing many Scale-concatenated args as single argument to call builder
    pub struct CallInput<'a>(pub &'a [u8]);
    impl<'a> scale::Encode for CallInput<'a> {
        fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
            dest.write(self.0);
        }
    }

    /// @dev Data
    #[derive(Default, Debug)]
    #[openbrush::storage_item]
    pub struct EmitterData {
        pub data: bool,
    }

    /// @dev Storage
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct DemoEmitter {
        // not used field
        #[storage_field]
        emitter_data: EmitterData,
    }

    /// @dev Implementation
    impl DemoEmitter {
        // -----------------------------------------------------------------------
        //                              Constructor
        // -----------------------------------------------------------------------

        #[ink(constructor)]
        pub fn new() -> Self {
            // return
            Self::default()
        }

        // -----------------------------------------------------------------------
        //                              Entrypoint
        // -----------------------------------------------------------------------

        #[ink(message)]
        pub fn entrypoint(
            &mut self,
            _controller: AccountId,   // deployed controller address
            _selector: SelectorBytes, // function selector to invoke
            _args: StringBytes,       // byte-encoded arguments for function
        ) -> Result<(), String> {
            // perform new call on controller
            ink::env::call::build_call::<ink::env::DefaultEnvironment>()
                .call(_controller)
                .gas_limit(0) // no gas limit
                .transferred_value(0) // no value
                .exec_input(
                    ink::env::call::ExecutionInput::new(ink::env::call::Selector::new(_selector)) // encoded selector
                        .push_arg(CallInput(&_args)), // encoded args
                )
                .call_flags(ink::env::CallFlags::default())
                .returns::<()>()
                .invoke();

            // return ok
            Ok(())
        }
    }
}
