
// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::log_str;
use near_sdk::env;
// use near_sdk::{near_bindgen, serde_json, AccountId, serde};
use near_sdk::{near_bindgen, AccountId};
use near_sdk::serde::{Serialize, Deserialize};
// use near_sdk::collections::{Vector, LookupMap};
use near_sdk::collections::Vector;

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
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            places: Vector::new(b"v"),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // FREE - Public method - returns the places saved, defaulting to DEFAULT_GREETING
    pub fn get_places(&self) -> Vec<Place> {
        return self.places.to_vec()
    }

    // FREE - get places by id
    pub fn get_places_by_id(&self, place_id: u64) -> Option<Place> {
        return self.places.get(place_id)
    }

    // PAYED - Public method - accepts a place, and records it
    pub fn add_place(&mut self, place: PlaceInput) {
        let place_name = place.name.clone();
        
        log_str(&format!("Adding new place: {place_name}"));

        let new_place = Place {
            id: self.places.len(),
            name: place.name,
            address: place.address,
            description: place.description,
            avarage_votes: 0,
            votes_counter: 0,
            votes: vec![],
            pictures: place.pictures,
        };

        self.places.push(&new_place)
    }

    // PAYED - Public method - Vote
    pub fn vote(&mut self, place_id: u64, vote: i8) {
        if self.places.get(place_id).is_some() {

            // Get the place by its index (id)
            let mut place = self.places.get(place_id).unwrap() as Place;
            let place_name = place.name.clone();
            log_str(&format!("Processing vote for: {place_name} - {vote}"));

            // Register the new vote
            let voter = env::predecessor_account_id();
            let new_vote = VoteMeta {
                account_id: voter,
                vote_value: vote,
            };
            
            place.votes.push(new_vote);

            // Update place's avarage of votes
            let votes_length = place.votes.len();
            let votes_sum = place.votes.iter().map(|vote_data| vote_data.vote_value).reduce(|value_a, value_b| value_a + value_b).unwrap();
            place.avarage_votes = votes_sum / votes_length as i8;

            // Update place's votes_counter
            place.votes_counter += 1;

            // Update the place inside the stored places
            self.places.replace(place_id, &place);
        }
    }

    // PAYED - Public method - Add pictures to a place
    pub fn add_picture_to_place(&mut self, place_id: u64, pictures: Vec<String>) {
        let mut place = self.places.get(place_id).unwrap() as Place;
        let updated_places = vec![place.pictures, pictures].concat();
        place.pictures = updated_places;

        // Update the place inside the stored places
        self.places.replace(place_id, &place);
    }

    // PAYED - Public method - Remove a place
    pub fn remove_place(&mut self, place_id: u64) {
        // NOTE: Is this a similar way for JS -> Array.filter?
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
        contract.vote(0, 1);
        contract.vote(0, 3);
        contract.vote(0, 4);

        assert_eq!(
            contract.get_places_by_id(0).unwrap().avarage_votes,
            3
        );
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
}
