use scrypto::prelude::*;
use crate::neuracle::NeuRacle;

blueprint! {
    struct NStableCoin {
        fee_vault: Vault,
        fee: Decimal,
        neura: ResourceAddress,
        symbol: String,
        pegged_to: String,
        stablecoin: ResourceAddress,
        controller_badge: Vault,
        data_badge: Vault,
        neuracle: ComponentAddress
    }

    impl NStableCoin {
        
        pub fn new(medium_token: ResourceAddress, pegged_to: String, neuracle: ComponentAddress, controller_badge: Bucket, data_badge: Bucket, fee: Decimal) -> ComponentAddress {

            let symbol: String = borrow_resource_manager!(medium_token).metadata().get("symbol").unwrap().into();

            let name: String = pegged_to.clone() + "NStable Coin";

            info!("Using {} as {}NStableCoin medium token", symbol, pegged_to.clone());

            info!("Using NeuRacle's service to get price data");

            let stablecoin = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", name.clone())
                .metadata("pegged_into", pegged_to.clone())
                .metadata("symbol", pegged_to.clone() + "N")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

                info!(
                    "{}N: {}", pegged_to.clone(), stablecoin
                );

            let component = Self {
                fee_vault: Vault::new(medium_token),
                fee: fee,
                neura: medium_token,
                symbol: symbol,
                pegged_to: pegged_to,
                stablecoin: stablecoin,
                controller_badge: Vault::with_bucket(controller_badge),
                data_badge: Vault::with_bucket(data_badge),
                neuracle: neuracle
            }
            .instantiate()
            .globalize();

            info!(
                "{} address: {}", name, component
            );

            return component
        }

        pub fn auto_swap(&mut self, token_bucket: Bucket) -> Bucket {

            let neuracle: NeuRacle = self.neuracle.into();

            let (data_badge, price) = neuracle.get_data(self.data_badge.take(dec!("1")));

            self.data_badge.put(data_badge);

            let price = Decimal::from(price);

            info!("Current {} price is {} {}, begin auto swap", self.symbol, price, self.pegged_to.clone());
            
            let token = token_bucket.resource_address();
            assert!(
                (token == self.stablecoin) || (token == self.neura) ,
                "Vault not contain this resource, cannot swap."
            );

            if token == self.neura {

                let amount: Decimal = token_bucket.amount() * price;

                let stable_coin_bucket = self.controller_badge.authorize(|| {
                    borrow_resource_manager!(self.stablecoin).mint(amount)
                });

                self.controller_badge.authorize(|| borrow_resource_manager!(self.neura).burn(token_bucket));

                info!("You got {} {} after using auto swap.", amount, self.pegged_to.clone() + "N");

                return stable_coin_bucket
            }

            else {

                let amount: Decimal = token_bucket.amount() / price;

                let medium_token_bucket = self.controller_badge.authorize(|| {
                    borrow_resource_manager!(self.stablecoin).mint(amount)
                });

                self.controller_badge.authorize(|| {
                    token_bucket.burn()
                });

                info!("You got {} {} after using auto swap.", amount, self.symbol);

                return medium_token_bucket
            }
        }
    }
}