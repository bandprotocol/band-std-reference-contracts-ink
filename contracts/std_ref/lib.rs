#![cfg_attr(not(feature = "std"), no_std, no_main)]
mod constant;
mod ref_data;
mod reference_data;

#[ink::contract]
mod std_ref {
    use ink::env::set_code_hash;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    use crate::constant::{E9, USD};
    use crate::ref_data::RefDatum;
    use crate::reference_data::ReferenceData;

    #[ink(storage)]
    pub struct StandardReference {
        /// Address of admin who can grant/revoke relayers
        admin: AccountId,
        /// Mapping of the granted relayers
        relayers: Mapping<AccountId, ()>,
        /// Mapping from symbol to reference datum
        ref_data: Mapping<String, RefDatum>,
    }

    /// Errors that can occur in the contract
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if the pair is invalid.
        PairDoesNotExist,
        /// Returned if the value is invalid.
        InvalidValue,
        /// Returned if unauthorized caller tries to call a function that requires authorization.
        Unauthorized,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl StandardReference {
        /// Creates a new StandardReference Contract
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            let mut relayers = Mapping::new();
            relayers.insert(admin, &());

            let ref_data = Mapping::new();

            Self {
                admin,
                ref_data,
                relayers,
            }
        }

        /// Upgrades the StandardReference contract
        #[ink(message)]
        pub fn upgrade(&mut self, code_hash: [u8; 32]) {
            if self.admin != self.env().caller() {
                panic!("Unauthorized");
            }

            set_code_hash(&code_hash)
                .unwrap_or_else(|err| panic!("Failed to set code hash due to {:?}", err));
        }

        /// Returns the account ID of the contract.
        #[ink(message)]
        pub fn contract_id(&self) -> AccountId {
            Self::env().account_id()
        }

        /// Returns the account ID of the current contract admin.
        #[ink(message)]
        pub fn current_admin(&self) -> AccountId {
            self.admin
        }

        /// Transfers the admin role to a new admin.
        #[ink(message)]
        pub fn transfer_admin(&mut self, new_admin: AccountId) -> Result<()> {
            if self.admin != self.env().caller() {
                return Err(Error::Unauthorized);
            }
            self.admin = new_admin;
            Ok(())
        }

        /// Checks if caller is relayer.
        #[ink(message)]
        pub fn is_relayer(&self, relayer: AccountId) -> bool {
            self.is_relayer_impl(&relayer)
        }

        /// Checks if caller is relayer.
        ///
        /// # Note
        ///
        /// Prefer to call this method over `is_relayer` since this
        /// works using references which are more efficient in Wasm.
        #[inline]
        fn is_relayer_impl(&self, relayer: &AccountId) -> bool {
            self.relayers.contains(relayer)
        }

        /// Adds relayers.
        #[ink(message)]
        pub fn add_relayers(&mut self, relayers: Vec<AccountId>) -> Result<()> {
            if self.admin != self.env().caller() {
                return Err(Error::Unauthorized);
            }
            for relayer in relayers {
                self.relayers.insert(relayer, &());
            }
            Ok(())
        }

        /// Removes relayers.
        #[ink(message)]
        pub fn remove_relayers(&mut self, relayers: Vec<AccountId>) -> Result<()> {
            if self.admin != self.env().caller() {
                return Err(Error::Unauthorized);
            }
            for relayer in relayers {
                self.relayers.take(relayer);
            }
            Ok(())
        }

        /// Returns the reference data for a given symbol
        #[ink(message)]
        pub fn get_reference_data(
            &mut self,
            symbol_pair: (String, String),
        ) -> Result<ReferenceData> {
            let base = self.get_ref_data(&symbol_pair.0)?;
            let quote = self.get_ref_data(&symbol_pair.1)?;

            ReferenceData::from_ref_data_pair(base, quote)
        }

        /// Returns the reference data for multiple bas/quote at once
        #[ink(message)]
        pub fn get_reference_data_bulk(
            &mut self,
            symbol_pair: Vec<(String, String)>,
        ) -> Vec<Result<ReferenceData>> {
            symbol_pair
                .into_iter()
                .map(|pair| self.get_reference_data(pair))
                .collect()
        }

        /// Returns the ref data for a given symbol.
        #[inline]
        fn get_ref_data(&mut self, symbol: &str) -> Result<RefDatum> {
            if symbol == USD {
                return Ok(RefDatum::new(E9, Self::env().block_timestamp() / 1000, 0));
            }

            self.ref_data.get(symbol).ok_or(Error::PairDoesNotExist)
        }

        /// Relays the data to the contract
        #[ink(message)]
        pub fn relay(
            &mut self,
            symbol_rates: Vec<(String, u64)>,
            resolve_time: Timestamp,
            request_id: u64,
        ) -> Result<()> {
            if !self.is_relayer_impl(&self.env().caller()) {
                return Err(Error::Unauthorized);
            }

            for (symbol, rate) in symbol_rates {
                let ref_datum = match self.ref_data.get(&symbol) {
                    Some(mut ref_datum) => {
                        ref_datum.update(rate, resolve_time, request_id);
                        ref_datum
                    }
                    None => RefDatum::new(rate, resolve_time, request_id),
                };
                self.ref_data.insert(&symbol, &ref_datum);
            }

            Ok(())
        }

        /// Relays the data to the contract without checking timestamp
        #[ink(message)]
        pub fn force_relay(
            &mut self,
            symbol_rates: Vec<(String, u64)>,
            resolve_time: Timestamp,
            request_id: u64,
        ) -> Result<()> {
            if !self.is_relayer_impl(&self.env().caller()) {
                return Err(Error::Unauthorized);
            }

            for (symbol, rate) in symbol_rates {
                self.ref_data
                    .insert(symbol, &RefDatum::new(rate, resolve_time, request_id));
            }

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn setup(admin: AccountId, relayers: &Vec<AccountId>) -> StandardReference {
            let mut std_ref = StandardReference::new(admin);
            let _ = std_ref.add_relayers(relayers.clone());
            std_ref
        }

        #[ink::test]
        fn test_init() {
            let admin = AccountId::from([0x01; 32]);
            let std_ref = StandardReference::new(admin);
            assert_eq!(std_ref.current_admin(), admin);
        }

        #[ink::test]
        fn test_transfer_admin() {
            let admin = AccountId::from([0x01; 32]);
            let new_admin = AccountId::from([0x02; 32]);
            let relayer = AccountId::from([0x03; 32]);

            let mut std_ref = StandardReference::new(admin);
            let _ = std_ref.add_relayers(vec![relayer]);

            // Transfer admin role successfully
            let result = std_ref.transfer_admin(new_admin);
            assert_eq!(result, Ok(()));
            assert_eq!(std_ref.current_admin(), new_admin);
        }

        #[ink::test]
        fn test_add_relayers() {
            let admin = AccountId::from([0x01; 32]);
            let mut std_ref = StandardReference::new(admin);
            let relayers = vec![
                AccountId::from([0x02; 32]),
                AccountId::from([0x03; 32]),
                AccountId::from([0x04; 32]),
            ];
            assert_eq!(std_ref.add_relayers(relayers.clone()), Ok(()));
            for relayer in relayers.clone() {
                assert!(std_ref.is_relayer(relayer));
            }
        }

        #[ink::test]
        fn test_remove_relayers() {
            let admin = AccountId::from([0x01; 32]);
            let relayers = vec![
                AccountId::from([0x02; 32]),
                AccountId::from([0x03; 32]),
                AccountId::from([0x04; 32]),
            ];

            let mut std_ref = setup(admin, &relayers);

            assert_eq!(std_ref.remove_relayers(relayers.clone()), Ok(()));
            for relayer in relayers.clone() {
                assert!(!std_ref.is_relayer(relayer));
            }
        }

        #[ink::test]
        fn test_relay_success() {
            let relay_admin = AccountId::from([0x01; 32]);
            let mut std_ref = StandardReference::new(relay_admin);

            let symbol_rates = vec![
                ("BTC".to_string(), E9),
                ("ETH".to_string(), 2 * E9),
                ("BAND".to_string(), 3 * E9),
            ];

            let resolve_time = 1;
            let request_id = 1;

            let res = std_ref.relay(symbol_rates.clone(), resolve_time, request_id);
            assert_eq!(res, Ok(()));

            // check if the rates are updated
            let symbol_pairs: Vec<(String, String)> = symbol_rates
                .iter()
                .map(|(s, _)| (s.clone(), USD.to_string()))
                .collect();
            let rd = std_ref.get_reference_data_bulk(symbol_pairs);

            for ((_, o), r) in symbol_rates.iter().zip(rd) {
                assert_eq!((o * E9) as u128, r.unwrap().rate);
            }
        }

        #[ink::test]
        fn test_force_relay_success() {
            let admin = AccountId::from([0x01; 32]);
            let relayer = AccountId::from([0x02; 32]);

            let mut std_ref = StandardReference::new(admin);
            let _ = std_ref.add_relayers(vec![relayer]);

            // Force relay successfully
            let result = std_ref.force_relay(vec![("BTC".to_string(), E9)], 1, 1);
            assert_eq!(result, Ok(()));

            // Check if the rate is updated
            let r: core::prelude::v1::Result<ReferenceData, Error> =
                std_ref.get_reference_data(("BTC".to_string(), "USD".to_string()));

            assert_eq!((E9 * E9) as u128, r.unwrap().rate);
        }

        #[ink::test]
        fn test_successful_relay_overwrite() {
            let relay_admin = AccountId::from([0x01; 32]);
            let mut std_ref = StandardReference::new(relay_admin);

            let symbol_rates = vec![
                ("BTC".to_string(), E9),
                ("ETH".to_string(), 2 * E9),
                ("BAND".to_string(), 3 * E9),
            ];

            let res = std_ref.relay(symbol_rates.clone(), 1, 1);
            assert_eq!(res, Ok(()));

            let symbol_rates = vec![
                ("BTC".to_string(), 2 * E9),
                ("ETH".to_string(), 4 * E9),
                ("BAND".to_string(), 8 * E9),
            ];

            let res = std_ref.relay(symbol_rates.clone(), 2, 2);
            assert_eq!(res, Ok(()));

            // check if the rates are updated
            let symbol_pairs: Vec<(String, String)> = symbol_rates
                .iter()
                .map(|(s, _)| (s.clone(), USD.to_string()))
                .collect();
            let rd = std_ref.get_reference_data_bulk(symbol_pairs);

            for ((_, o), r) in symbol_rates.iter().zip(rd) {
                assert_eq!((o * E9) as u128, r.unwrap().rate);
            }
        }

        #[ink::test]
        fn test_stale_relay() {
            let relay_admin = AccountId::from([0x01; 32]);
            let mut std_ref = StandardReference::new(relay_admin);

            let symbol_rates = vec![
                ("BTC".to_string(), E9),
                ("ETH".to_string(), 2 * E9),
                ("BAND".to_string(), 3 * E9),
            ];

            let res = std_ref.relay(symbol_rates.clone(), 5, 5);
            assert_eq!(res, Ok(()));

            let stale_symbol_rates = vec![
                ("BTC".to_string(), 2 * E9),
                ("ETH".to_string(), 4 * E9),
                ("BAND".to_string(), 8 * E9),
            ];

            let res = std_ref.relay(stale_symbol_rates.clone(), 2, 2);
            assert_eq!(res, Ok(()));

            // check if the rates are updated
            let symbol_pairs: Vec<(String, String)> = symbol_rates
                .iter()
                .map(|(s, _)| (s.clone(), String::from(USD)))
                .collect();
            let rd = std_ref.get_reference_data_bulk(symbol_pairs);

            for ((_, o), r) in symbol_rates.iter().zip(rd) {
                assert_eq!((o * E9) as u128, r.unwrap().rate);
            }
        }
    }
}
