// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::env;
use near_sdk::env::log_str;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VoteMeta {
    account_id: AccountId,
    vote_value: i8,
}

#[near_bindgen]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PlaceInput {
    name: String,
    address: String,
    description: String,
    pictures: Vec<String>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Place {
    id: u64,
    name: String,
    address: String,
    description: String,
    avarage_votes: i8,
    votes_counter: i32,
    votes: Vec<VoteMeta>,
    pictures: Vec<String>,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    places: Vector<Place>,
    last_id: u64,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            // b"v" é um prefixador que vai ser usado como chave no store do contrato
            places: Vector::new(b"v"),
            last_id: 0,
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // FREE - Public method - returns the places saved, defaulting to DEFAULT_GREETING
    pub fn get_places(&self) -> Vec<Place> {
        return self.places.to_vec();
    }

    // FREE - get places by id
    pub fn get_places_by_id(&self, place_id: u64) -> Option<Place> {
        if let Some(index) = self.places.iter().position(|place| place.id == place_id) {
            self.places.get(index as u64)
        } else {
            None
        }
    }

    // PAYED - Public method - accepts a place, and records it
    pub fn add_place(&mut self, place: PlaceInput) {
        let place_name = place.name.clone();
        log_str(&format!("Adding new place: {place_name}"));

        let new_place = Place {
            id: self.last_id,
            name: place.name,
            address: place.address,
            description: place.description,
            avarage_votes: 0,
            votes_counter: 0,
            votes: vec![],
            pictures: place.pictures,
        };

        self.places.push(&new_place);
        self.last_id += 1;
    }

    // PAYED - Public method - Vote
    pub fn vote(&mut self, place_id: u64, vote: i8) {
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

    // PAYED - Public method - Add pictures to a place
    pub fn add_picture_to_place(&mut self, place_id: u64, pictures: Vec<String>) {
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

    // PAYED - Public method - Remove a place
    pub fn remove_place(&mut self, place_id: u64) {
        // NOTE: Is this a similar way for JS -> Array.filter?
        log_str(&format!("Removing place where place_id is: {place_id}"));
        if let Some(index) = self.places.iter().position(|place| place.id == place_id) {
            self.places.swap_remove(index as u64);
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_places_then_get_places() {
        let mut contract = Contract::default();

        let new_place = PlaceInput {
            name: "Grumeti Gourmet".to_string(),
            address: "Pampulha".to_string(),
            description: "Um lugar legal".to_string(),
            pictures: vec![
                "https://lh5.googleusercontent.com/p/AF1QipMBMUOyXp7E1gZRB_KVeKLOLOpZv1bzZt-JxsAd=w408-h306-k-no".to_string(),
                "https://www.meioemensagem.com.br/wp-content/uploads/2019/05/Natura_NovaLoja_Fachada_Credito_IlanaBessler_575.jpg".to_string()
            ],
        };
        contract.add_place(new_place);

        contract.add_place(PlaceInput {
            name: "Popurri Gourmet".to_string(),
            address: "Lourdes".to_string(),
            description: "A beautful place".to_string(),
            pictures: vec![],
        });

        let places = contract.get_places();
        let first_child = places.first().unwrap();

        assert_eq!(places.len(), 2);
        assert_eq!(first_child.id, 0);
    }

    #[test]
    fn get_place_by_id() {
        let mut contract = Contract::default();

        let new_place = PlaceInput {
            name: "Grumeti Gourmet".to_string(),
            address: "Pampulha".to_string(),
            description: "Um lugar legal".to_string(),
            pictures: vec![
                "https://lh5.googleusercontent.com/p/AF1QipMBMUOyXp7E1gZRB_KVeKLOLOpZv1bzZt-JxsAd=w408-h306-k-no".to_string(),
                "https://www.meioemensagem.com.br/wp-content/uploads/2019/05/Natura_NovaLoja_Fachada_Credito_IlanaBessler_575.jpg".to_string()
            ],
        };
        contract.add_place(new_place);

        contract.add_place(PlaceInput {
            name: "Popurri Gourmet".to_string(),
            address: "Lourdes".to_string(),
            description: "A beautful place".to_string(),
            pictures: vec![],
        });

        let place = contract.get_places_by_id(1).unwrap();

        assert_eq!(place.id, 1);
    }

    #[test]
    fn vote_then_get_avarage() {
        let mut contract = Contract::default();

        contract.add_place(PlaceInput {
            name: "Grumeti Gourmet".to_string(),
            address: "Pampulha".to_string(),
            description: "Um lugar legal".to_string(),
            pictures: vec![],
        });

        contract.vote(0, 5);

        assert_eq!(contract.get_places_by_id(0).unwrap().avarage_votes, 5);
    }

    #[test]
    fn update_place_pictures() {
        let mut contract = Contract::default();

        contract.add_place(PlaceInput {
            name: "Grumeti Gourmet".to_string(),
            address: "Pampulha".to_string(),
            description: "Um lugar legal".to_string(),
            pictures: vec![],
        });

        contract.add_place(PlaceInput {
            name: "Popurri Gourmet".to_string(),
            address: "Lourdes".to_string(),
            description: "A beautful place".to_string(),
            pictures: vec![],
        });

        let pic_0 = "https://lh5.googleusercontent.com/p/AF1QipMBMUOyXp7E1gZRB_KVeKLOLOpZv1bzZt-JxsAd=w408-h306-k-no".to_string();
        let pic_1 = "https://www.meioemensagem.com.br/wp-content/uploads/2019/05/Natura_NovaLoja_Fachada_Credito_IlanaBessler_575.jpg".to_string();
        let new_pictures = vec![pic_0.clone(), pic_1.clone()];

        contract.add_picture_to_place(0, new_pictures);

        let place_0 = contract.get_places_by_id(0).unwrap();

        assert_eq!(place_0.pictures[0], pic_0);
        assert_eq!(place_0.pictures[1], pic_1);
    }

    #[test]
    fn remove_place_then_check_if_it_is_enabled() {
        let mut contract = Contract::default();

        contract.add_place(PlaceInput {
            name: "Grumeti Gourmet".to_string(),
            address: "Pampulha".to_string(),
            description: "Um lugar legal".to_string(),
            pictures: vec![],
        });

        contract.add_place(PlaceInput {
            name: "Popurri Gourmet".to_string(),
            address: "Lourdes".to_string(),
            description: "A beautful place".to_string(),
            pictures: vec![],
        });

        contract.remove_place(1);

        assert_eq!(contract.get_places().len(), 1);
        assert_eq!(contract.get_places()[0].name, "Grumeti Gourmet".to_string());
    }

    #[test]
    fn vote_twice_with_diff_values() {
        let mut contract = Contract::default();

        contract.add_place(PlaceInput {
            name: "Grumeti Gourmet".to_string(),
            address: "Pampulha".to_string(),
            description: "Um lugar legal".to_string(),
            pictures: vec![],
        });

        contract.add_place(PlaceInput {
            name: "Popurri Gourmet".to_string(),
            address: "Lourdes".to_string(),
            description: "A beautful place".to_string(),
            pictures: vec![],
        });

        contract.vote(1, 2);
        assert_eq!(contract.get_places_by_id(1).unwrap().avarage_votes, 2);
        contract.vote(1, 5);
        assert_eq!(contract.get_places_by_id(1).unwrap().avarage_votes, 5);
    }
}
