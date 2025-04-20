#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, token, unwrap::UnwrapOptimized, Address, Env,
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Offer,
}

#[derive(Clone)]
#[contracttype]
pub struct Offer {
    pub seller: Address,
    pub sell_token: Address,
    pub buy_token: Address,
    pub sell_price: u32,
    pub buy_price: u32,
}

#[contract]
pub struct SingleOffer;

#[contractimpl]
impl SingleOffer {
    pub fn create(
        env:Env,
        seller: Address,
        sell_token: Address,
        buy_token: Address,
        sell_price: u32,
        buy_price: u32,
    ){
        if env.storage().instance().has(&DataKey::Offer) {
            panic!("offer is already created");
        }
        if buy_price == 0 || sell_price == 0 {
            panic!("zero price is not allowed");
        }

        seller.require_auth();
        write_offer(
            &env,
            &Offer {
                seller,
                sell_token,
                buy_token,
                sell_price,
                buy_price,
            },
        );
    }

    pub fn trade(env:Env,buyer:Address, buy_token_amount: i128,min_sell_token_amount:i128){
        buyer.require_auth();

        let offer = load_offer(&env);
        let sell_token_client = token::Client::new(&env, &offer.sell_token);
        let buy_token_client = token::Client::new(&env, &offer.buy_token);

        let sell_token_amount = buy_token_amount
            .checked_mul(offer.sell_price as i128)
            .unwrap_optimized() / offer.buy_price as i128;

        if sell_token_amount < min_sell_token_amount {
            panic!("price is too low");
        }
        let contract = env.current_contract_address();


        buy_token_client.transfer(&buyer, &contract, &buy_token_amount);
        sell_token_client.transfer(&contract, &buyer, &sell_token_amount);
        buy_token_client.transfer(&contract,&offer.seller, &buy_token_amount);
    }

    pub fn withdraw(env:Env,token:Address, amount:i128){
        let offer = load_offer(&env);
        offer.seller.require_auth();
        token::Client::new(&env, &token)
            .transfer(&env.current_contract_address(), &offer.seller, &amount);
    }

    pub fn update_price(env:Env, sell_price: u32, buy_price: u32) {
        if sell_price == 0 || buy_price == 0 {
            panic!("zero price is not allowed");
        }
        
        let mut offer = load_offer(&env);
        offer.seller.require_auth();    
        
        offer.sell_price = sell_price;
        offer.buy_price = buy_price;
        write_offer(&env, &offer);
    }

    pub fn get_offer(env:Env) -> Offer {
        load_offer(&env)
    }
}

fn load_offer(env: &Env) -> Offer {
    env.storage().instance().get(&DataKey::Offer).unwrap_optimized()
}

fn write_offer(env: &Env, offer: &Offer) {
    env.storage().instance().set(&DataKey::Offer, offer);
}

mod test;