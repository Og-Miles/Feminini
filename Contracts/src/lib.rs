#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env, Symbol, Vec};

#[contract]
pub struct NFTContract;

#[contractimpl]
impl NFTContract {
    // Initialize the contract with an admin
    pub fn initialize(env: &Env, admin: Address) {
        env.storage().instance().set(&symbol_short!("admin"), &admin);
    }

    // Mint a new NFT
    pub fn mint_nft(env: &Env, to: Address, item_id: u32) {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin"))
            .expect("Contract not initialized");
        
        // Only admin can mint
        admin.require_auth();
        
        // Store NFT ownership
        env.storage().instance().set(&symbol_short!("owner"), &to);
        env.storage().instance().set(&symbol_short!("item_id"), &item_id);
        env.storage().instance().set(&symbol_short!("is_burned"), &false);
    }

    // Burn an NFT
    pub fn burn_nft(env: &Env, item_id: u32) {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin"))
            .expect("Contract not initialized");
        let owner: Address = env.storage().instance().get(&symbol_short!("owner"))
            .expect("No NFT exists");
        
        // Check if caller is either admin or owner
        if admin != env.invoker() && owner != env.invoker() {
            panic!("Only owner or admin can burn NFT");
        }

        // Verify the NFT exists and isn't already burned
        let stored_item_id: u32 = env.storage().instance().get(&symbol_short!("item_id"))
            .expect("No NFT exists");
        let is_burned: bool = env.storage().instance().get(&symbol_short!("is_burned"))
            .unwrap_or(false);

        if stored_item_id != item_id {
            panic!("NFT with this ID doesn't exist");
        }
        if is_burned {
            panic!("NFT is already burned");
        }

        // Mark NFT as burned
        env.storage().instance().set(&symbol_short!("is_burned"), &true);
    }

    // Check if an NFT is valid (not burned)
    pub fn is_valid_certificate(env: &Env, item_id: u32) -> bool {
        let stored_item_id: u32 = env.storage().instance().get(&symbol_short!("item_id"))
            .expect("No NFT exists");
        let is_burned: bool = env.storage().instance().get(&symbol_short!("is_burned"))
            .unwrap_or(false);

        stored_item_id == item_id && !is_burned
    }

    // Transfer NFT (only if not burned)
    pub fn transfer_nft(env: &Env, to: Address, item_id: u32) {
        let owner: Address = env.storage().instance().get(&symbol_short!("owner"))
            .expect("No NFT exists");
        
        // Check if caller is the owner
        if owner != env.invoker() {
            panic!("Only owner can transfer NFT");
        }

        // Verify NFT exists and isn't burned
        let stored_item_id: u32 = env.storage().instance().get(&symbol_short!("item_id"))
            .expect("No NFT exists");
        let is_burned: bool = env.storage().instance().get(&symbol_short!("is_burned"))
            .unwrap_or(false);

        if stored_item_id != item_id {
            panic!("NFT with this ID doesn't exist");
        }
        if is_burned {
            panic!("Cannot transfer burned NFT");
        }

        // Update ownership
        env.storage().instance().set(&symbol_short!("owner"), &to);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_nft_lifecycle() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let owner = Address::generate(&env);
        let new_owner = Address::generate(&env);
        let item_id = 1;

        // Initialize contract
        NFTContract::initialize(&env, admin.clone());

        // Test minting
        admin.as_contract(&env.current_contract(), || {
            NFTContract::mint_nft(&env, owner.clone(), item_id);
        });

        // Test transfer
        owner.as_contract(&env.current_contract(), || {
            NFTContract::transfer_nft(&env, new_owner.clone(), item_id);
        });

        // Test burning
        new_owner.as_contract(&env.current_contract(), || {
            NFTContract::burn_nft(&env, item_id);
        });

        // Verify NFT is burned
        assert!(!NFTContract::is_valid_certificate(&env, item_id));
    }

    #[test]
    #[should_panic(expected = "Only owner or admin can burn NFT")]
    fn test_burn_by_non_owner() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let owner = Address::generate(&env);
        let non_owner = Address::generate(&env);
        let item_id = 1;

        // Initialize and mint
        NFTContract::initialize(&env, admin.clone());
        admin.as_contract(&env.current_contract(), || {
            NFTContract::mint_nft(&env, owner.clone(), item_id);
        });

        // Attempt to burn as non-owner
        non_owner.as_contract(&env.current_contract(), || {
            NFTContract::burn_nft(&env, item_id);
        });
    }

    #[test]
    #[should_panic(expected = "Cannot transfer burned NFT")]
    fn test_transfer_burned_nft() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let owner = Address::generate(&env);
        let new_owner = Address::generate(&env);
        let item_id = 1;

        // Initialize, mint, and burn
        NFTContract::initialize(&env, admin.clone());
        admin.as_contract(&env.current_contract(), || {
            NFTContract::mint_nft(&env, owner.clone(), item_id);
        });
        owner.as_contract(&env.current_contract(), || {
            NFTContract::burn_nft(&env, item_id);
        });

        // Attempt to transfer burned NFT
        owner.as_contract(&env.current_contract(), || {
            NFTContract::transfer_nft(&env, new_owner.clone(), item_id);
        });
    }
}
