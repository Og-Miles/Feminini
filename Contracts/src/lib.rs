use soroban_sdk::{Env, Map, String, contract, contractimpl};

const mapper: Map<String, u32> = Map::new("JWL000", 12345);

#[contract]
pub struct FemininiContract;

#[contractimpl]
impl FemininiContract {
    pub fn link_jewelry(_jewelry_id: String, token_id: u32) -> Map<String, u32> {
        // Load or create the map from storage
        let mut map = Env
            .storage()
            .persistent()
            .get::<_, Map<String, u32>>(JEWELRY_MAP_KEY)
            .unwrap_or_else(|| Map::new(&Env));
        // Set the mapping
        map.set(jewelry_id, token_id);
        // Save the map back to storage
        Env.storage().persistent().set(JEWELRY_MAP_KEY, &map);
    }

    pub fn get_token_by_jewelry(_jewelry_id: String) -> u32 {
        let map = Env
            .storage()
            .persistent()
            .get::<_, Map<String, u32>>(JEWELRY_MAP_KEY)
            .unwrap_or_else(|| Map::new(&Env));
        map.get(jewelry_id)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{String, testutils::Env};
    #[test]

    fn test_link_jewelry() {
        //Arrange
        let env = Env::default();
        let jewelry_id = String::from_str(&env, "JWL001");
        let token_id = 12345u32;

        //Act
        let result = FemininiContract::link_jewelry(jewelry_id, token_id);

        //Assert
        assert_eq!(result.get(jewelry_id).unwrap(), token_id);
    }

    fn test_when_link_jewelry_assert_linked_only_once() {
        //Arrange
        let env = Env::default();
        let jewelry_id = String::from_str(&env, "JWL001");
        let token_id = 12345u32;
        let token_id2 = 67890u32;
        //Act
        let result = FemininiContract::link_jewelry(jewelry_id, token_id);
        let result = FemininiContract::link_jewelry(jewelry_id, token_id2);
        //Assert
        assert_eq!(result.get(jewelry_id).unwrap(), token_id);
        assert_ne!(result.get(jewelry_id).unwrap(), token_id2);
    }

    fn test_get_token_by_jewelry() {
        //Arrange
        let env = Env::default();
        let jewelry_id = String::from_str(&env, "JWL001");
        let token_id = 12345u32;

        //Act
        let result = FemininiContract::get_token_by_jewelry(token_id);

        //Assert
        assert_eq!(result.get(jewelry_id).unwrap(), token_id);
    }
}
