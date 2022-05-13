use scrypto::prelude::*;
use crate::data_api::DataApi;

blueprint! {
    struct NStableCoin {
        neura_vault: Vault,
        fee_vault: Vault,
        fee: Decimal,
        neura: ResourceAddress,
        symbol: String,
        name: String,
        stablecoin: ResourceAddress,
        controller_badge: Vault,
        data_api: DataApi,
        query: String
    }

    impl NStableCoin {
        
        pub fn new(medium_token: ResourceAddress, pegged_to: String, query: String, neura_price_data_api: ComponentAddress, controller_badge: Bucket, fee: Decimal) -> ComponentAddress {

            let symbol: String = borrow_resource_manager!(medium_token).metadata().get("symbol").unwrap().into();

            let name: String = pegged_to.clone() + "NStable Coin";

            info!("Using {} as {}NStableCoin medium token", symbol, pegged_to.clone());

            info!("Using NeuRacle's service to get price data");

            let data_api: DataApi = neura_price_data_api.into();

            let stablecoin = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", name.clone())
                .metadata("pegged_into", pegged_to.clone())
                .metadata("symbol", pegged_to.clone() + "N")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

                info!(
                    "{}N: {}", pegged_to, stablecoin
                );

            let component = Self {
                neura_vault: Vault::new(medium_token),
                fee_vault: Vault::new(medium_token),
                fee: fee,
                neura: medium_token,
                symbol: symbol,
                name: name,
                stablecoin: stablecoin,
                controller_badge: Vault::with_bucket(controller_badge),
                data_api: data_api,
                query: query
            }
            .instantiate()
            .globalize();

            info!(
                "Component: {}", component
            );

            return component
        }

        pub fn auto_swap(&mut self, token_bucket: Bucket) -> Bucket {

            let price = self.controller_badge.authorize(|| {self.data_api.get_data(self.query.clone())});

            info!("Current {} price is {} usd, begin auto swap", self.symbol, price);
            
            let token = token_bucket.resource_address();
            assert!(
                (token == self.stablecoin) || (token == self.neura) ,
                "Vault not contain this resource, cannot swap."
            );

            assert!(
                !((token == self.stablecoin) & (self.neura_vault.is_empty())),
                "Not enough {} to swap.", self.symbol 
            );

            if token == self.neura {

                let amount: Decimal = token_bucket.amount() * price;

                let usx_bucket = self.controller_badge.authorize(|| {
                    borrow_resource_manager!(self.stablecoin).mint(amount)
                });

                self.controller_badge.authorize(|| borrow_resource_manager!(self.neura).burn(token_bucket));

                return usx_bucket
            }

            else {

                let amount: Decimal = token_bucket.amount() / price;

                let medium_token_bucket = self.neura_vault.take(amount);

                self.controller_badge.authorize(|| {
                    token_bucket.burn()
                });

                medium_token_bucket
            }
        }
    }
}