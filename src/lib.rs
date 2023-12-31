// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedSet, Vector};
use near_sdk::env;
use near_sdk::env::log_str;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId};

pub mod internal;
pub mod utils;
pub use crate::utils::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VoteMeta {
    account_id: AccountId,
    vote_value: i8,
    feedback: Option<String>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Address {
    address: String,
    country: String,
    state_or_province: String,
    city: String,
}

#[near_bindgen]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PlaceInput {
    name: String,
    address: Address,
    description: String,
    pictures: Vec<String>,
    place_type: String, // Food, shopping, travel, study, university,...
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Place {
    id: u64,
    name: String,
    address: Address,
    description: String,
    place_type: String, // Food, shopping, travel, study, university,...
    avarage_votes: i8,
    votes_counter: i32,
    votes: Vec<VoteMeta>,
    pictures: Vec<String>,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner: AccountId,
    admins: UnorderedSet<AccountId>,
    places: Vector<Place>,
    last_id: u64,
}

// Default state to use if no initialize method is called.
// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            // b"v" é um prefixador que vai ser usado como chave no store do contrato
            owner: "wendersonpires.testnet".parse().unwrap(),
            admins: UnorderedSet::new(b"s"),
            places: Vector::new(b"v"),
            last_id: 0,
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner: AccountId, admins: Option<Vec<AccountId>>) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        Self {
            owner,
            admins: account_vec_to_set(
                if admins.is_some() {
                    admins.unwrap()
                } else {
                    vec![]
                },
                b"s",
            ),
            places: Vector::new(b"v"),
            last_id: 0,
        }
    }

    // free - Returns the places saved, defaulting to DEFAULT_GREETING
    pub fn get_places(&self) -> Vec<Place> {
        return self.places.to_vec();
    }

    // free - Get places by id
    pub fn get_places_by_id(&self, place_id: u64) -> Option<Place> {
        if let Some(index) = self.places.iter().position(|place| place.id == place_id) {
            self.places.get(index as u64)
        } else {
            None
        }
    }

    // payable - Accepts a place, and records it
    pub fn add_place(&mut self, place: PlaceInput) {
        let place_name = place.name.clone();
        log_str(&format!("Adding new place: {place_name}"));

        let new_place = Place {
            id: self.last_id,
            name: place.name,
            address: place.address,
            description: place.description,
            place_type: place.place_type,
            avarage_votes: 0,
            votes_counter: 0,
            votes: vec![],
            pictures: place.pictures,
        };

        self.places.push(&new_place);
        self.last_id += 1;
    }

    // payable - Vote
    pub fn vote(&mut self, place_id: u64, vote: i8, feedback: Option<String>) {
        if let Some(index) = self.places.iter().position(|place| place.id == place_id) {
            // Get the place by its index (id)
            let mut place = self.places.get(index as u64).unwrap() as Place;
            let place_name = place.name.clone();
            log_str(&format!("Processing vote for: {place_name} - {vote}"));

            // Check if user has voted on this place already
            let voter = env::predecessor_account_id();
            let previous_vote_index = place.votes.iter().position(|vote| vote.account_id == voter);

            // Register the new vote
            let new_vote = VoteMeta {
                account_id: voter,
                vote_value: vote,
                feedback,
            };

            // If user has voted already, just update its vote
            if previous_vote_index.is_some() {
                let vote_index = previous_vote_index.unwrap();
                place.votes[vote_index] = new_vote;
            } else {
                // Add a new vote
                place.votes.push(new_vote);
                // Update place's votes_counter
                place.votes_counter += 1;
            }

            // Update place's avarage of votes
            let votes_length = place.votes.len();
            let votes_sum = place
                .votes
                .iter()
                .map(|vote_data| vote_data.vote_value)
                .reduce(|value_a, value_b| value_a + value_b)
                .unwrap();
            place.avarage_votes = votes_sum / votes_length as i8;

            // Update the place inside the stored places
            self.places.replace(index as u64, &place);
        }
    }

    // payable - Add pictures to a place
    pub fn add_picture_to_place(&mut self, place_id: u64, pictures: Vec<String>) {
        assert!(
            self.is_owner_or_admin(),
            "Only the owner or admins can call this method"
        );

        if let Some(index) = self.places.iter().position(|place| place.id == place_id) {
            let mut place = self.places.get(index as u64).unwrap() as Place;
            let place_name = place.name.clone();
            log_str(&format!("Adding pictures to: {place_name}"));

            let updated_places = vec![place.pictures, pictures].concat();
            place.pictures = updated_places;

            // Update the place inside the stored places
            self.places.replace(index as u64, &place);
        }
    }

    // payable - Remove a place
    pub fn remove_place(&mut self, place_id: u64) {
        assert!(
            self.is_owner_or_admin(),
            "Only the owner or admins can call this method"
        );

        // NOTE: Is this a similar way for JS -> Array.filter?
        log_str(&format!("Removing place where place_id is: {place_id}"));
        if let Some(index) = self.places.iter().position(|place| place.id == place_id) {
            self.places.swap_remove(index as u64);
        }
    }

    // TODO: add and remove admin [admin can do it only]
}

// Tests in a separated file (see more here -> http://xion.io/post/code/rust-unit-test-placement.html)
#[cfg(test)]
#[path = "./tests.rs"]
mod tests;
