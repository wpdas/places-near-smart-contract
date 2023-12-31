/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use near_sdk::AccountId;

    use crate::{Address, Contract, PlaceInput};

    fn get_contract() -> Contract {
        let bob: AccountId = "bob.near".parse().unwrap();
        Contract::init(bob, None)
    }

    fn add_places_to_contract(contract: &mut Contract) {
        contract.add_place(PlaceInput {
            name: "Grumeti Gourmet".to_string(),
            address: Address {
                address: "Pampulha".to_string(),
                country: "Brazil".to_string(),
                state_or_province: "Minas Gerais".to_string(),
                city: "Belo Horizonte".to_string(),
            },
            place_type: "shopping".to_string(),
            description: "Um lugar legal".to_string(),
            pictures: vec![
                "https://lh5.googleusercontent.com/p/AF1QipMBMUOyXp7E1gZRB_KVeKLOLOpZv1bzZt-JxsAd=w408-h306-k-no".to_string(),
                "https://www.meioemensagem.com.br/wp-content/uploads/2019/05/Natura_NovaLoja_Fachada_Credito_IlanaBessler_575.jpg".to_string()
            ],
        });

        contract.add_place(PlaceInput {
            name: "Popurri Gourmet".to_string(),
            address: Address {
                address: "Lourdes".to_string(),
                country: "Brazil".to_string(),
                state_or_province: "Minas Gerais".to_string(),
                city: "Belo Horizonte".to_string(),
            },
            place_type: "shopping".to_string(),
            description: "A beautful place".to_string(),
            pictures: vec![],
        });
    }

    #[test]
    fn add_places_then_get_places() {
        let mut contract = get_contract();
        add_places_to_contract(&mut contract);

        let places = contract.get_places();
        let first_child = places.first().unwrap();

        assert_eq!(places.len(), 2);
        assert_eq!(first_child.id, 0);
    }

    #[test]
    fn get_place_by_id() {
        let mut contract = get_contract();
        add_places_to_contract(&mut contract);

        let place = contract.get_places_by_id(1).unwrap();

        assert_eq!(place.id, 1);
    }

    #[test]
    fn vote_then_get_avarage() {
        let mut contract = get_contract();
        add_places_to_contract(&mut contract);

        contract.vote(0, 5, Some("I've been asking for problems".to_string()));

        assert_eq!(contract.get_places_by_id(0).unwrap().avarage_votes, 5);
    }

    #[test]
    fn update_place_pictures() {
        let mut contract = get_contract();
        add_places_to_contract(&mut contract);

        let pic_0 = "https://lh5.googleusercontent.com/p/AF1QipMBMUOyXp7E1gZRB_KVeKLOLOpZv1bzZt-JxsAd=w408-h306-k-no".to_string();
        let pic_1 = "https://www.meioemensagem.com.br/wp-content/uploads/2019/05/Natura_NovaLoja_Fachada_Credito_IlanaBessler_575.jpg".to_string();
        let new_pictures = vec![pic_0.clone(), pic_1.clone()];

        contract.add_picture_to_place(1, new_pictures);

        let place_0 = contract.get_places_by_id(1).unwrap();

        assert_eq!(place_0.pictures[0], pic_0);
        assert_eq!(place_0.pictures[1], pic_1);
    }

    #[test]
    fn vote_twice_with_diff_values() {
        let mut contract = get_contract();
        add_places_to_contract(&mut contract);

        contract.vote(
            1,
            2,
            Some("Look me in the eyes, tell me what you see...".to_string()),
        );
        assert_eq!(contract.get_places_by_id(1).unwrap().avarage_votes, 2);
        contract.vote(1, 5, Some("Now you know, you free to go!".to_string()));
        assert_eq!(contract.get_places_by_id(1).unwrap().avarage_votes, 5);
    }

    #[test]
    fn remove_place() {
        let mut contract = get_contract();
        add_places_to_contract(&mut contract);

        contract.remove_place(1);

        assert_eq!(contract.get_places().len(), 1);
        assert_eq!(contract.get_places()[0].name, "Grumeti Gourmet".to_string());
    }
}
