use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{log, near_bindgen, AccountId, env};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Side {
    Heads,
    Tails
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CoinFlip {
    points: UnorderedMap<AccountId, u64>
}

impl Default for CoinFlip {
    fn default() -> Self {
        Self { points: UnorderedMap::new(b'c') }
    }
}

 /*
    Flip a coin. Pass in the side (heads or tails) and a random number will be chosen
    indicating whether the flip was heads or tails. If you got it right, you get a point.
  */
#[near_bindgen]
impl CoinFlip {
    pub fn flip_coin(&mut self, player_guess: Side) -> Side{
        // Check who called the method
        let player = env::predecessor_account_id();
        log!("{} chose {:?}", player, player_guess);

        // Stimulate a coin flip
        let outcome = Self::simulate_coin_flip();

        // Get the current player points
        let mut player_points = self.points.get(&player).unwrap_or(0);

        // Check if their guess was right and modify the points accordingly
        if player_guess == outcome {
            log!("The result was {:?}, you get a point!", outcome);
            player_points += 1;
        } else {
            log!("The result was {:?}, you lost a point!", outcome);
            player_points = if player_points == 0 {0} else {player_points - 1};
        }

        // Store the new points
        self.points.insert(&player, &player_points);
        
        return outcome
    }

    pub fn get_points_by_account(&self, account: AccountId) -> u64 {
        self.points.get(&account).unwrap_or(0)
    }

    #[private]
    fn simulate_coin_flip() -> Side {
        // randomSeed creates a random string, learn more about it in the README 
        let random_seed = env::random_seed();

        // If the last charCode is even we choose heads, otherwise tails
        return if random_seed[0] % 2 == 0 {Side::Heads} else {Side::Tails} 
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let contract  = CoinFlip::default();
        assert_eq!(contract.points.len(), 0);
    }

    #[test]
    fn test_flip_coin() {
        let mut contract  = CoinFlip::default();

        let result = &contract.flip_coin(Side::Heads);

        let point = contract.get_points_by_account(env::predecessor_account_id());
        if *result == Side::Heads {
            assert_eq!(point, 1)
        } else {
            assert_eq!(point, 0)
        }
    }

}
