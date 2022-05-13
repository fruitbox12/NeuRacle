use scrypto::prelude::*;
use crate::data_api::DataApi;
use crate::neura_stable_coin::NStableCoin;
use crate::validator::Validator;
use crate::utilities::*;

#[derive(NonFungibleData)]
pub struct ValidatorData {
    pub name: String,
    pub location: String,
    pub website: String,
    pub address: String,
    pub staked: Decimal
}

#[derive(NonFungibleData)]
pub struct UserData {
    pub end: u64,
    pub api: String,
    pub query: String
}

blueprint! {

    struct NeuRacle {

        data_apis: HashMap<String, ComponentAddress>,
        non_validated_apis: HashMap<String, (u8, String)>,
        stable_coins: LazyMap<ComponentAddress, String>,
        validators: Vec<(ComponentAddress, Decimal)>,
        validator_cap: usize,
        neura_vault: Vault,
        controller_badge: Vault,
        neura: ResourceAddress,
        validator_badge: ResourceAddress,
        user_badge: ResourceAddress,
        pay_rate: Decimal,
        fee_stablecoin: Decimal,
        unstake_delay: u64,
        stage: u8,
        round_length: u64,
        reward_rate: Decimal,
        punishment: Decimal,
        system_time: u64,
        round_start: bool,
        active_validators: HashMap<ComponentAddress, Decimal>,
        mint_controller_badge: Vault

    }

    impl NeuRacle {

        //For easier understanding, I will just provide example of input parameters.
        //validator_cap = 100 (same as Radix Olympia) > Data will only be choosen from top 100 validator.
        //round_length = 1 > Data will be refreshed after 1 epoch. Current Scrypto version can only use this unit of timestamp. 
        //Later NeuRacle will use transactions amount as unit of timestamp. 
        //Or NeuRacle can even use time oracle service of others in the ecosystem.
        //pay_rate = 1 > users must pay the ecosystem 1 neura fee per "round"
        //fee_stablecoin = 0.3 > stable coin users must pay the ecosystem 0.3% fee.
        //Same as the Radix Ecosystem, all the fee are burned!
        //unstake_delay(epoch) = 500 (Same as Radix Olympia) > staker can only redeem their token after 500 epoch. 
        //This is to ensure security of the ecosystem.
        //reward_rate(%) = 0.0015 > stakers will earn 0.0015% of staked amount per round
        //if round length = 1 epoch, estimated is 1 hour, that's about 13.14% APY.
        //punishment = 5 > staked value will be slashed by 10 * reward rate on untruthful behavior.
        pub fn new(validator_cap: usize, round_length: u64, pay_rate: Decimal, fee_stablecoin: Decimal, unstake_delay: u64, reward_rate: Decimal, punishment: Decimal) -> (ComponentAddress, Bucket, Bucket) {

            let system_time = Runtime::current_epoch() / round_length;

            assert_fee(fee_stablecoin);

            let admin_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "NeuRacle Admin Badge")
                .initial_supply(dec!("1"));

                info!(
                    "Admin badge address: {}", admin_badge.resource_address()
                );

            let mint_controller_badge = ResourceBuilder::new_fungible()
                .metadata("name", "NeuRacle Controller Badge")
                .initial_supply(dec!("1"));

            let controller_badge = ResourceBuilder::new_fungible()
                .mintable(rule!(require(mint_controller_badge.resource_address())), LOCKED)
                .metadata("name", "NeuRacle Controller Badge")
                .initial_supply(dec!("1"));

             let validator_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", "NeuRacle Validator Badge")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();
            
                info!("Validator badge address: {}", validator_badge);

            let user_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", "NeuRacle User Badge")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

                info!("User badge: {}", user_badge);

            //We can have many strategy to decentralize the token like vesting, airdrop, locking,... 
            //but let's just focus on NeuRacle main system for now.
            let neura = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "NeuRacle")
                .metadata("symbol", "NAR")
                .updateable_metadata(rule!(require(admin_badge.resource_address())), MUTABLE(rule!(require(admin_badge.resource_address()))))
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .initial_supply(dec!("10000000"));

                info!("Neura: {}", neura.resource_address());

            let rules = AccessRules::new()
                .method("create_new_validator_node", rule!(require(admin_badge.resource_address())))
                .method("advance_stage", rule!(require(admin_badge.resource_address())))
                .method("set_unstake_delay", rule!(require(admin_badge.resource_address())))
                .method("new_round", rule!(require(admin_badge.resource_address())))
                .method("end_round", rule!(require(admin_badge.resource_address())))
                .method("new_stable_coin_project", rule!(require(admin_badge.resource_address())))
                .method("new_api", rule!(require(controller_badge.resource_address())))
                .default(rule!(allow_all));

            let component = Self {
                data_apis: HashMap::new(),
                non_validated_apis: HashMap::new(),
                stable_coins: LazyMap::new(),
                validators: Vec::new(),
                validator_cap: validator_cap,
                controller_badge: Vault::with_bucket(controller_badge),
                neura_vault: Vault::new(neura.resource_address()),
                neura: neura.resource_address(),
                validator_badge: validator_badge,
                user_badge: user_badge,
                pay_rate: pay_rate,
                fee_stablecoin: fee_stablecoin,
                unstake_delay: unstake_delay,
                stage: 1,
                round_length: round_length,
                reward_rate: reward_rate / dec!("100"),
                punishment: punishment,
                system_time: system_time,
                round_start: false,
                active_validators: HashMap::new(),
                mint_controller_badge: Vault::with_bucket(mint_controller_badge)
                }
                .instantiate()
                .add_access_check(rules)
                .globalize();
    
            info!(
                "Component: {}", component
            );

            return (component, admin_badge, neura)
        }

        //At first, to prevent Sybil attack, NeuRacle also need to use DPoS mechanism and choose only Validators that has the basic requirement of network traffic.
        pub fn create_new_validator_node(&mut self, name: String, location: String, website: String, fee: Decimal) -> (ComponentAddress, Bucket) {

            assert!(self.stage == 1);

            let validator_id: NonFungibleId = NonFungibleId::random();

            let badge = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.validator_badge)
                .mint_non_fungible(&validator_id, ValidatorData{
                    name: name.clone(),
                    location: location,
                    website: website,
                    address: String::default(),
                    staked: Decimal::zero()
                })
            });

            let validator_address = Validator::new(self.neura, self.controller_badge.resource_address(), badge.resource_address(), name, fee, self.unstake_delay);

            badge.non_fungible::<ValidatorData>().data().address = validator_address.to_string();

            info!("Validator address: {}", validator_address);

            self.validators.push((validator_address, Decimal::zero()));

            return (validator_address, badge)

        }

        //After Xi'an, when the NeuRacle system is more decentralized, anyone can become validator.
        pub fn become_new_validator(&mut self, name: String, location: String, website: String, fee: Decimal) -> (ComponentAddress, Bucket) {

            assert!(self.stage == 2);

            let validator_id: NonFungibleId = NonFungibleId::random();
            
            let badge = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.validator_badge)
                .mint_non_fungible(&validator_id, ValidatorData{
                    name: name.clone(),
                    location: location,
                    website: website,
                    address: String::default(),
                    staked: Decimal::zero()
                })
            });

            let validator_address = Validator::new(self.neura, self.controller_badge.resource_address(), badge.resource_address(), name, fee, self.unstake_delay);

            badge.non_fungible::<ValidatorData>().data().address = validator_address.to_string();

            info!("Validator address: {}", validator_address);

            self.validators.push((validator_address, Decimal::zero()));

            return (validator_address, badge)

        }

        pub fn become_new_user(&mut self, mut payment: Bucket, api: String, query: String) -> (Bucket, Bucket) {

            let amount = payment.amount();

            assert_resource(payment.resource_address(), self.neura, amount, self.pay_rate);

            let user_id: NonFungibleId = NonFungibleId::random();

            let length = (amount/self.pay_rate).floor();

            self.controller_badge.authorize(|| {
                payment.take(length * self.pay_rate).burn();
            });

            let current = Runtime::current_epoch();

            let end = 10*self.round_length + current + length.to_string().parse::<u64>().unwrap();

            let badge = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.user_badge)
                .mint_non_fungible(&user_id, UserData{
                    end: end,
                    api: api.clone(),
                    query: query.clone()
                })
            });
            
            info!("You can access this data from {} until {} epoch", 10*self.round_length + current, end);

            if self.data_apis.contains_key(&api) {

                let api: DataApi = self.data_apis[&api].into();
                api.new_query(query);
                
            }

            else {

                self.non_validated_apis.insert(api, (0, query));

            };

            return (badge, payment)
            
        }

        pub fn new_api(&mut self, api: String) {

            let api_address = DataApi::new(api.clone(), 
            self.controller_badge.resource_address(), 
            self.non_validated_apis.remove(&api).unwrap().1, 
            self.reward_rate, 
            self.punishment);

            self.data_apis.insert(api, api_address);

        }

        pub fn refund_account(&mut self, identity: Bucket, mut payment: Bucket) -> (Bucket, Bucket) {

            let amount = payment.amount();

            assert_resource(identity.resource_address(), self.user_badge, identity.amount(), dec!("1"));
            assert_resource(payment.resource_address(), self.neura, amount, dec!("0"));

            let length = (amount/self.pay_rate).floor();

            self.controller_badge.authorize(|| {
                payment.take(length * self.pay_rate).burn();
            });

            let current = Runtime::current_epoch();

            let end = current + length.to_string().parse::<u64>().unwrap();

            identity.non_fungible::<UserData>().data().end = end;
            
            info!("You can access your data from now until {} epoch", end);

            return (identity, payment)

        }

        pub fn get_data(&self, identity: Bucket) -> (Bucket, Decimal) {

            assert_resource(identity.resource_address(), self.user_badge, identity.amount(), dec!("1"));
            
            let data = identity.non_fungible::<UserData>().data();

            assert!(
                (Runtime::current_epoch() <= data.end) || (data.end == 0),
                "Run out of time, you cannot access this data for now, please refund your account."
            );

            let &address = self.data_apis.get(&data.api).unwrap();

            let data_api: DataApi = address.into();

            let my_data: Decimal = self.controller_badge.authorize(|| data_api.get_data(data.query));

            return (identity, my_data)
        }

        pub fn new_round(&mut self) {
            
            assert!(
                self.round_start == false,
                "Previous round haven't ended yet"
            );

            let current = Runtime::current_epoch();

            assert!(
                current/self.round_length >= self.system_time,
                "Not time to start a new round yet"
            );

            #[allow(unused_assignments)]
            for &(validator_address, mut weight) in &self.validators {

                let validator: Validator = validator_address.into();

                weight = validator.get_current_staked_value();

                validator.reset_status();

            }

            if self.stage == 1 {

                self.validators.sort_by_key(|a| a.1);
                self.validators.reverse();
                self.active_validators = self.validators.get(0..self.validator_cap).unwrap().to_vec().into_iter().collect();
                self.controller_badge.authorize(|| {
                for (_api, &address) in &self.data_apis {
                    
                        let data_api: DataApi = address.into();
                        data_api.new_round(self.active_validators.clone());

                    }
                });
            }

            else {

                self.controller_badge.authorize(|| {
                    for (_api, &address) in &self.data_apis {
                        let data_api: DataApi = address.into();
                        data_api.new_round(self.validators.clone().into_iter().collect());
                    }
                });
            }

            self.round_start = true

        }

        //I will put the requirement time unit here after we got smaller time unit like transactions history. Wait for 1 epoch to settle a price data is insane!
        pub fn end_round(&mut self) {
            
            assert!(
                self.round_start == true,
                "New round hasn't started yet"
            );
            
            let current = Runtime::current_epoch();

            if self.stage == 1 {

                self.validators.sort_by_key(|a| a.1);
                self.validators.reverse();
                self.active_validators = self.validators.get(0..self.validator_cap).unwrap().to_vec().into_iter().collect();
                
                self.controller_badge.authorize(|| {
                    for (_api, &address) in &self.data_apis {
                    
                        let data_api: DataApi = address.into();
                        data_api.end_round(self.active_validators.clone());
                    }
                });

                for (api, time) in &mut self.non_validated_apis.clone() {
                    
                    let result = vote_for_api(api.clone(), self.active_validators.clone());
                    match result {
                        Some(true) => {
                            self.new_api(api.clone());
                        }
                        None => {
                            self.non_validated_apis.get_mut(&api.clone()).unwrap().0 += 1;
                        }
                        Some(false) => {
                            if time.0 >= 9 {self.non_validated_apis.remove(api);}
                            else {
                            self.non_validated_apis.get_mut(&api.clone()).unwrap().0 += 1;}
                        }
                    }
                    
                }
                
            }

            else {

                self.controller_badge.authorize(|| {

                    for (_api, &address) in &self.data_apis {

                        let data_api: DataApi = address.into();
                        data_api.end_round(self.validators.clone().into_iter().collect());

                    }
                });

                for (api, time) in &mut self.non_validated_apis.clone() {
                    
                    let result = vote_for_api(api.clone(), self.active_validators.clone());
                    match result {
                        Some(true) => {
                            self.new_api(api.clone());
                        }
                        None => {
                            self.non_validated_apis.get_mut(&api.clone()).unwrap().0 += 1;
                        }
                        Some(false) => {
                            if time.0 >= 9 {self.non_validated_apis.remove(api);}
                            else {
                            self.non_validated_apis.get_mut(&api.clone()).unwrap().0 += 1;}
                        }
                    }
                    
                }
            }

            self.system_time = current/self.round_length + 1;

            self.round_start = false
            
        }

        pub fn new_stable_coin_project(&mut self, pegged_to: String, api: String) -> ComponentAddress {

            let mut query: String = "NAR ".to_owned();
            let pegged = pegged_to.clone();
            let peg = pegged.as_str();
            query.push_str(peg);
            query.push_str(" last price");

            if !self.data_apis.contains_key(&api) {

                let api_address = DataApi::new(api.clone(), 
                self.controller_badge.resource_address(),
                query.clone(),
                self.reward_rate, 
                self.punishment);
                self.data_apis.insert(api.clone(), api_address);
            
            };

            let api_address = self.data_apis[&api];

            let controller_badge = self.mint_controller_badge.authorize(|| borrow_resource_manager!(self.controller_badge.resource_address()).mint(dec!("1")));

            let stable_coin_project_address = NStableCoin::new(self.neura, pegged_to.clone(), query, api_address, controller_badge, self.fee_stablecoin);

            self.stable_coins.insert(stable_coin_project_address, pegged_to + " NStable Coin");

            return stable_coin_project_address

        }

        pub fn show_validators(&self) {
            info!("Begin show data pools, format: (validator_id: staked_weight)|| {:?}", self.validators)
        }

        pub fn show_data_apis(&self) {
            info!("Begin show data apis, {:?}", self.data_apis.keys())
        }


        pub fn show_stable_coins(&self) {
            info!("Begin show stable coin projects, format: (project_address: project_name)|| {:?}", self.stable_coins)
        }

        pub fn get_all_query_per_api(&self) -> LazyMap<String, Vec<String>> {
            let result = LazyMap::new();
            
            for (api, &address) in &self.data_apis.clone() {

                let data_api: DataApi = address.into();
                let query = data_api.get_query();
                result.insert(api.clone(), query)

            }
            
            return result
        }

        pub fn advance_stage(&mut self) {
            assert!(self.stage == 2);
            self.stage += 1
        }

        pub fn set_unstake_delay(&mut self, new_unstake_delay: u64) {
            self.unstake_delay = new_unstake_delay
        }

    }
}