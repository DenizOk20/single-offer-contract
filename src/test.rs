#![cfg(test)]
extern crate std;

use crate::{token,SingleOfferClient};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal, Symbol,
};

fn create_token_contract<'a>(env:&Env,admin:&Address) -> (token::Client<'a>,token::StellarAssetClient<'a>){
    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(env, &sac.address()),
        token::StellarAssetClient::new(env, &sac.address()),
    )
}

fn create_single_offer_contract<'a>(
    env:&Env,
    seller:&Address,
    sell_token:&Address,
    buy_token:&Address,
    sell_price:u32,
    buy_price:u32,
    ) -> SingleOfferClient<'a>{
    let offer = SingleOfferClient::new(env, &env.register(crate::SingleOffer,()));
    offer.create(seller,sell_token,buy_token,&sell_price,&buy_price);

    assert_eq!(env.auths(),
        std::vec![
            (
                seller.clone(),
                AuthorizedInvocation{
                    function: AuthorizedFunction::Contract((
                        offer.address.clone(),
                        symbol_short!("create"),
                        (
                            seller.clone(),
                            sell_token.clone(),
                            buy_token.clone(),
                            sell_price,
                            buy_price
                        )
                        .into_val(env),
                    )),
                    sub_invocations: std::vec![]
                }
            )
        ]
    );
    offer
}


#[test]

fn test(){
    let env = Env::default();
    env.mock_all_auths();

    let token_admin = Address::generate(&env);
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);

    let sell_token = create_token_contract(&env, &token_admin);
    let sell_token_client = sell_token.0;
    let sell_token_admin_client = sell_token.1;

    let buy_token = create_token_contract(&env, &token_admin);
    let buy_token_client = buy_token.0;
    let buy_token_admin_client = buy_token.1;

    let offer = create_single_offer_contract(
        &env,
        &seller,
        &sell_token_client.address,
        &buy_token_client.address,
        1,
        2,
    );

    sell_token_admin_client.mint(&seller,&1000);
    buy_token_admin_client.mint(&buyer,&1000);
    sell_token_client.transfer(&seller,&offer.address,&100);

    assert!(offer.try_trade(
        &buyer,
        &20_i128,
        &11_i128,
    ).is_err());

    offer.trade(
        &buyer,
        &20_i128,
        &10_i128,
    );

    assert_eq!(
        env.auths(),
        std::vec![
            (
                buyer.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        offer.address.clone(),
                        symbol_short!("trade"),
                        (
                            &buyer,
                            20_i128,
                            10_i128
                        )
                        .into_val(&env),
                    )),
                    sub_invocations: std::vec![AuthorizedInvocation {
                        function:AuthorizedFunction::Contract((
                            buy_token_client.address.clone(),
                            symbol_short!("transfer"),
                            (
                                buyer.clone(),
                                &offer.address,
                                &20_i128
                            ).into_val(&env)
                        )),
                        sub_invocations: std::vec![]
                    }]
                }
            )
        ]
    );

    assert_eq!(sell_token_client.balance(&seller), 900);
    assert_eq!(sell_token_client.balance(&buyer), 10);
    assert_eq!(sell_token_client.balance(&offer.address), 90);
    assert_eq!(buy_token_client.balance(&seller), 20);
    assert_eq!(buy_token_client.balance(&buyer), 980);
    assert_eq!(buy_token_client.balance(&offer.address), 0);


    offer.withdraw(&sell_token_client.address, &70_i128);

    assert_eq!(
        env.auths(),
        std::vec![
            (
                seller.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        offer.address.clone(),
                        symbol_short!("withdraw"),
                        (
                            sell_token_client.address.clone(),
                            &70_i128
                        )
                        .into_val(&env),
                    )),
                    sub_invocations: std::vec![]
                }
            )]
        );

        assert_eq!(sell_token_client.balance(&seller), 970);
        assert_eq!(sell_token_client.balance(&offer.address), 20);

        offer.update_price(&1, &1);

        assert_eq!(
            env.auths(),
            std::vec![
                (
                    seller.clone(),
                    AuthorizedInvocation {
                        function: AuthorizedFunction::Contract((
                            offer.address.clone(),
                            Symbol::new(&env,"update_price"),
                            (
                                1_u32,
                                1_u32,
                            )
                            .into_val(&env),
                        )),
                        sub_invocations: std::vec![]
                    }
                )
            ]
        );


    offer.trade(&buyer, &10_i128, &9_i128);
    assert_eq!(sell_token_client.balance(&seller), 970);
    assert_eq!(sell_token_client.balance(&buyer), 20);
    assert_eq!(sell_token_client.balance(&offer.address), 10);
    assert_eq!(buy_token_client.balance(&seller), 30);
    assert_eq!(buy_token_client.balance(&buyer), 970);
    assert_eq!(buy_token_client.balance(&offer.address), 0);
}