use crate::Contract;
use crate::ContractExt;

use near_sdk::serde::Serialize;
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::{ env, log, near_bindgen, AccountId, Promise, Balance };
use near_sdk::json_types::U128;

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000; // 0.001 N

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Donation {
    pub account_id: AccountId,
    pub total_amount: U128
}

#[near_bindgen]
impl Contract {
    #[payable] // Public - People can attach money
    pub fn donate(&mut self) -> U128 {
        // Get who is calling the method and how much $NEAR they attched
        let donor = env::predecessor_account_id();
        let donation_amount: Balance = env::attached_deposit();

        let mut donated_so_far = self.donations.get(&donor).unwrap_or(0);

        let to_transfer: Balance = if donated_so_far == 0 {
            // This is the user's first donation, lets register it, which increase storage
            assert!(donation_amount > STORAGE_COST, "Attach at least {} yoctoNEAR", STORAGE_COST);

            // Substract the storage cost to the amount to transfer
            donation_amount - STORAGE_COST
        } else {
            donation_amount
        };

        // Persist in storage the amount donated so far
        donated_so_far += donation_amount;
        let mut total = u128::from(self.total);
        total += donation_amount;
        self.total = U128(total);
        self.donations.insert(&donor, &donated_so_far);

        log!("Thank you {} for donating {}! You donated a total of {}", donor.clone(), donation_amount.clone(), donated_so_far.clone());

        // Send the money to the beneficiary
        Promise::new(self.beneficiary.clone()).transfer(to_transfer);

        // Return the total amount donated so far
        U128(donated_so_far)
    }

    // Public - get donation by account ID
    pub fn get_donation_by_account(&self, account_id: AccountId) -> Donation {
        Donation { 
            account_id: account_id.clone(), 
            total_amount: U128(self.donations.get(&account_id).unwrap_or(0)) 
        }
    }

    // Public  - get total number of donors
    pub fn number_of_donors(&self) -> u64 {
        self.donations.len()
    }

    // Public  - get total NEAR of donations
    pub fn total_donate(&self) -> U128 {
        self.total
    }

    // Public - pagination through all donations on the contract
    pub fn get_donations(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Donation> {
        // where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        // iterate through donation
        self.donations.iter()
                      .skip(start as usize)
                      .take(limit.unwrap_or(50) as usize)
                      .map(|(account, _)| self.get_donation_by_account(account))
                      .collect()
    }
}


