#![cfg_attr(not(feature = "std"), no_std)]

pub use self::contract_report::ContractReportRef;

#[ink::contract]
mod contract_report {
    
    use ink::env::call::{Selector, ExecutionInput};
    use ink::storage::Mapping;
    use ink::prelude::string::String;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Return if the balance cannot fulfill a request.
        InsufficientBalance,
        AlreadyOnList,
        TransferError,
        TransferErrorToOwner,
        TransferErrorToReporter,
        NotOnList,
        OwnerCantInteract,
    }

    /// Specify the ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    /// Create storage for a simple ERC-20 contract.
    #[ink(storage)]
    pub struct ContractReport {

        ///Arrangement of users who bought the song
        punishment_balances: Mapping<AccountId,Balance>,

        ///Owner address
        owner: AccountId,

        ///Account id that refers to the song contract
        buy_contract_address: AccountId,

        ///Song Name
        song_name: String,
    }

    impl ContractReport {

        //------------------------------CONSTRUCTOR------------------------------

        /// Crea una contrato de reporte cuando se publica una cancion
        #[ink(constructor)]
        pub fn new(owner: AccountId, buy_contract_address:AccountId, song_name: String) -> Self {

            let punishment_balances = Mapping::default();

            Self{
                punishment_balances,
                owner,
                buy_contract_address,
                song_name
            }

        }

        //Messages

        //------------------------------GETTERS------------------------------

        #[ink(message,payable,selector = 0xA1B2C3D4)]
        pub fn add_user_punishment(&mut self, buyer_address: AccountId) -> Result<(String,Balance)>{

            if self.env().caller() == self.owner {
                return Err(Error::OwnerCantInteract)
            }

            if self.env().transferred_value() <= 0 {
                return Err(Error::InsufficientBalance)
            }

            if self.is_user_in_list(buyer_address){
                return Err(Error::AlreadyOnList)
            }

            self.punishment_balances.insert(buyer_address,&self.env().transferred_value());

            Ok((String::from("Deposito satisfactorio"),self.punishment_balances.get(buyer_address).unwrap()))

        }

        #[ink(message)]
        pub fn is_user_in_list(&self, address:AccountId) -> bool{

            if self.punishment_balances.contains(address){
                return true
            }
            return false;

            /*let x = ink::env::call::build_call::<Environment>()
                .call(self.buy_contract_address)
                .gas_limit(0)
                .transferred_value(0)
                .exec_input(
                    ExecutionInput::new(Selector::new([0x12,0x34,0x56,0x78]))
                    .push_arg(address)
                )
                .returns::<bool>()
                .invoke();

            return x*/

        }

        #[ink(message)]
        pub fn get_user_punishment(&self, address:AccountId) -> Balance{

            if self.punishment_balances.contains(address){
                return self.punishment_balances.get(address).unwrap()
            }
            return 0

        }



        #[ink(message)]
        pub fn recover_image(&self) -> String{

            let x = ink::env::call::build_call::<Environment>()
                .call(self.buy_contract_address)
                .gas_limit(0)
                .transferred_value(0)
                .exec_input(
                    ExecutionInput::new(Selector::new([0xAB,0xCD,0x12,0x34]))
                )
                .returns::<String>()
                .invoke();

            return x

        }

        #[ink(message)]
        pub fn pay_reporter_and_owner(&mut self, reward: Balance, song_distributor: AccountId) -> Result<(String,Balance,Balance)>{

            if self.env().caller() == self.owner {
                return Err(Error::OwnerCantInteract)
            }

            if !self.is_user_in_list(song_distributor){
                return Err(Error::NotOnList)
            }

            let mut balance_to_reporter = Balance::default();
            let mut balance_to_owner = Balance::default();
            let punishment_deposit = self.punishment_balances.get(song_distributor).unwrap();

            if punishment_deposit < reward {

                balance_to_reporter = punishment_deposit / 2;
                balance_to_owner = punishment_deposit / 2;

            }else if reward == 0{

                balance_to_reporter = 0;
                balance_to_owner = punishment_deposit;

            }            
            
            else{

                if punishment_deposit - reward < punishment_deposit / 2{

                    balance_to_reporter = punishment_deposit / 2;
                    balance_to_owner = punishment_deposit / 2;

                }else{

                    balance_to_owner = punishment_deposit - reward;
                    balance_to_reporter = 0;

                }
            }

            if self.env().transfer(self.owner, balance_to_owner).is_err(){
                return Err(Error::TransferError)
            }

            if self.env().transfer(self.env().caller(), balance_to_reporter).is_err(){
                return Err(Error::TransferError)
            }

            self.punishment_balances.remove(song_distributor);

            Ok((String::from("Reporte verificado, gracias por tu reporte"),balance_to_owner,balance_to_reporter))

        }
        
        //------------------------------SETTERS------------------------------


        //------------------------------HELPERS------------------------------
    }

    //------------------------------TESTS------------------------------
    /*
    #[cfg(test)]
    mod tests {
        use ink::{primitives::AccountId};

        use super::*;

        // We define some helper Accounts to make our tests more readable
        fn default_accounts() -> ink::env::test::DefaultAccounts<Environment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn alice() -> AccountId {
            default_accounts().alice
        }

        fn bob() -> AccountId {
            default_accounts().bob
        }

        fn charlie() -> AccountId {
            default_accounts().charlie
        }

        fn django() -> AccountId {
            default_accounts().django
        }

        #[ink::test]
        fn publish_works(){
            let contract = ContractPublish::new_publish(
            "La bebe - ringtone".to_string(), 
            1, 
            "QmZ41fazG24A6H4bicrM2cTPjLWxxsX8tQkrAPzCu2e8AB".to_string(),
            "QmZ2Fg6zDt8p7SLsuVAL2spGAAY2rPp7JShAY3Xk6Ndt8o".to_string());
            assert_eq!(contract.recover_hash_address(),"QmZ41fazG24A6H4bicrM2cTPjLWxxsX8tQkrAPzCu2e8AB");
            assert_eq!(contract.recover_image_address(),"QmZ2Fg6zDt8p7SLsuVAL2spGAAY2rPp7JShAY3Xk6Ndt8o");
            assert_eq!(contract.recover_song_name(),"La bebe - ringtone");
            assert_eq!(contract.recover_song_price(),1);
        }
    }*/
}
