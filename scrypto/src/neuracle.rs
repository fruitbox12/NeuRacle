use scrypto::prelude::*;
use crate::neura_stable_coin::NStableCoin;
use crate::validator::Validator;
use crate::utilities::*;

#[derive(NonFungibleData)]
pub struct ValidatorData {
    pub name: String,
    pub location: String,
    pub website: String,
    #[scrypto(mutable)]
    pub address: String

}

#[derive(NonFungibleData)]
pub struct UserData {
    #[scrypto(mutable)]
    pub end: u64,
    pub api: String,
}

blueprint! {

    struct NeuRacle {

        datas: HashMap<String, String>,
        non_validated_datas: HashMap<String, String>,
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
        mint_controller_badge: Vault,
        leader: Option<ComponentAddress>

    }

    impl NeuRacle {

        ///For easier understanding, I will just provide example of input parameters.
        ///validator_cap = 100 (same as Radix Olympia) > Data will only be choosen from top 100 validator.
        ///round_length = 1 > Data will be refreshed after 1 epoch. Current Scrypto version can only use this unit of timestamp. 
        ///Later NeuRacle will use transactions amount as unit of timestamp. 
        ///Or NeuRacle can even use time oracle service of others in the ecosystem.
        ///pay_rate = 1 > users must pay the ecosystem 1 neura fee per "round"
        ///fee_stablecoin = 0.3 > stable coin users must pay the ecosystem 0.3% fee.
        ///Same as the Radix Ecosystem, all the fee are burned!
        ///unstake_delay(epoch) = 500 (Same as Radix Olympia) > staker can only redeem their token after 500 epoch. 
        ///This is to ensure security of the ecosystem.
        ///reward_rate(%) = 0.0015 > stakers will earn 0.0015% of staked amount per round
        ///if round length = 1 epoch, estimated is 1 hour, that's about 13.14% APY.
        ///punishment = 5 > staked value will be slashed by 10 * reward rate on untruthful behavior.
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
                .updateable_non_fungible_data(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();
            
                info!("Validator badge address: {}", validator_badge);

            let user_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", "NeuRacle User Badge")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(controller_badge.resource_address())), LOCKED)
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
                datas: HashMap::new(),
                non_validated_datas: HashMap::new(),
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
                mint_controller_badge: Vault::with_bucket(mint_controller_badge),
                leader: None,
                }
                .instantiate()
                .add_access_check(rules)
                .globalize();
    
            info!(
                "Component: {}", component
            );

            return (component, admin_badge, neura)
        }

        ///At first, to prevent Sybil attack, NeuRacle also need to use DPoS mechanism and choose only Validators that has the basic requirement of network traffic.
        pub fn create_new_validator_node(&mut self, name: String, location: String, website: String, fee: Decimal) -> (ComponentAddress, Bucket) {

            assert!(self.stage == 1);

            let validator_id: NonFungibleId = NonFungibleId::random();

            let mut badge = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.validator_badge)
                .mint_non_fungible(&validator_id, ValidatorData{
                    name: name.clone(),
                    location: location,
                    website: website,
                    address: String::default()
                })
            });

            let validator_address = Validator::new(self.neura, badge.resource_address(), self.controller_badge.resource_address(), name, fee, self.unstake_delay);

            let mut data: ValidatorData = badge.non_fungible().data();

            data.address = validator_address.to_string();
            
            self.controller_badge
                .authorize(|| badge.non_fungible().update_data(data));

            self.validators.push((validator_address, Decimal::zero()));

            return (validator_address, badge)

        }

        ///After Xi'an, when the NeuRacle system is more decentralized, anyone can become validator.
        pub fn become_new_validator(&mut self, name: String, location: String, website: String, fee: Decimal) -> (ComponentAddress, Bucket) {

            assert!(self.stage == 2);

            let validator_id: NonFungibleId = NonFungibleId::random();
            
            let mut badge = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.validator_badge)
                .mint_non_fungible(&validator_id, ValidatorData{
                    name: name.clone(),
                    location: location,
                    website: website,
                    address: String::default()
                })
            });

            let validator_address = Validator::new(self.neura, self.controller_badge.resource_address(), badge.resource_address(), name, fee, self.unstake_delay);

            let mut data: ValidatorData = badge.non_fungible().data();

            data.address = validator_address.to_string();

            self.controller_badge
                .authorize(|| badge.non_fungible().update_data(data));

            self.validators.push((validator_address, Decimal::zero()));

            return (validator_address, badge)

        }

        pub fn become_new_user(&mut self, mut payment: Bucket, api: String) -> (Bucket, Bucket) {

            let amount = payment.amount();

            assert_resource(payment.resource_address(), self.neura, amount, self.pay_rate);

            let user_id: NonFungibleId = NonFungibleId::random();

            let length = (amount/self.pay_rate).floor();

            self.controller_badge.authorize(|| {
                payment.take(length * self.pay_rate).burn();
            });

            let current = Runtime::current_epoch();

            let end = current + length.to_string().parse::<u64>().unwrap() * self.round_length;

            let badge = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.user_badge)
                .mint_non_fungible(&user_id, UserData{
                    end: end,
                    api: api.clone()
                })
            });
            
            info!("You can access this data from now until epoch {}", end);

            if !self.datas.contains_key(&api) {

                self.datas.insert(api.clone(), String::default());
            
            }

            return (badge, payment)
            
        }

        pub fn refund_account(&mut self, mut identity: Bucket, mut payment: Bucket) -> (Bucket, Bucket) {

            let amount = payment.amount();

            assert_resource(identity.resource_address(), self.user_badge, identity.amount(), dec!("1"));
            assert_resource(payment.resource_address(), self.neura, amount, dec!("0"));

            let length = (amount/self.pay_rate).floor();

            self.controller_badge.authorize(|| {
                payment.take(length * self.pay_rate).burn();
            });

            let current = Runtime::current_epoch();

            let end = current + length.to_string().parse::<u64>().unwrap() * self.round_length;

            let mut data: UserData = identity.non_fungible().data();

            data.end = end;

            self.controller_badge
                .authorize(|| identity.non_fungible().update_data(data));
            
            info!("You can access your data until epoch {}", end);

            return (identity, payment)

        }

        pub fn get_data(&self, identity: Bucket) -> (Bucket, String) {

            assert_resource(identity.resource_address(), self.user_badge, identity.amount(), dec!("1"));
            
            let data = identity.non_fungible::<UserData>().data();

            assert!(
                (Runtime::current_epoch() <= data.end) || (data.end == 0),
                "Run out of time, you cannot access this data for now, please refund your account."
            );

            let my_data = self.datas.get(&data.api).unwrap().clone();

            return (identity, my_data)
        }

        pub fn new_round(&mut self) -> HashMap<String, String> {
            
            assert!(
                self.round_start == false,
                "Previous round haven't ended yet!"
            );

            let current = Runtime::current_epoch();

            assert!(
                current/self.round_length >= self.system_time,
                "Not time to start a new round yet!"
            );

            #[allow(unused_variables)]
            self.controller_badge.authorize(|| {

                self.validators.iter().for_each(|&(validator_address, mut weight)| {

                    let validator: Validator = validator_address.into();

                    weight = validator.get_current_staked_value();
                
                    validator.reset_status();

                });
            });

            info!("Start voting round number {} of NeuRacle", self.system_time);

            self.round_start = true;

            if self.stage == 1 {

                self.validators.sort_by_key(|a| a.1);
                self.validators.reverse();
                let try_get_cap = self.validators.get(0..self.validator_cap);

                match try_get_cap {
                    Some(x) => self.active_validators = x.iter().cloned().collect(),
                    None => self.active_validators = self.validators.iter().cloned().collect()
                }
                
                let random_leader_validator: (ComponentAddress, Decimal) = self.active_validators.clone().drain().next().unwrap();
                let validator: Validator = random_leader_validator.0.into();

                self.controller_badge.authorize(|| {
                    self.non_validated_datas = validator.get_data();
                });
                
                self.leader = Some(random_leader_validator.0);

                return validator.get_data()

            }

            else {
                
                self.active_validators = self.validators.iter().cloned().collect();

                let random_leader_validator: (ComponentAddress, Decimal) = self.active_validators.clone().drain().next().unwrap();

                let validator: Validator = random_leader_validator.0.into();

                self.controller_badge.authorize(|| {
                    self.non_validated_datas = validator.get_data();
                });
                
                self.leader = Some(random_leader_validator.0);

                return validator.get_data()
            }
        }

        pub fn end_round(&mut self) {
            
            assert!(
                self.round_start == true,
                "New round hasn't started yet!"
            );
        
            let mut val: HashMap<ComponentAddress, Decimal> = HashMap::new();

            self.active_validators.iter().for_each(|(&address, weight)| {
    
                let validator: Validator = address.into();
                    
                if validator.get_status() {
                    
                    val.insert(address, weight.clone());
                    
                }
            });
            
            assert!(
                val.len()*3 > self.active_validators.len()*2,
                "Not enough validator active yet!"
            );

            self.active_validators = val;

            let current = Runtime::current_epoch();

            let result = vote_for_data(self.active_validators.clone());

            match result {

            Some(true) => {

                self.datas = self.non_validated_datas.clone();

                    self.controller_badge.authorize(|| {

                    self.active_validators.iter().for_each(|(&address, _weight)| {
    
                        let validator: Validator = address.into();
                            
                        if validator.get_status() {
                            if validator.get_vote() {validator.mint(self.reward_rate)}
                            else {validator.burn(self.reward_rate * self.punishment)}
                            
                            }
                        });  
                    });
                }
            None => {}

            Some(false) => {

                let malicious_validator: Validator = self.leader.unwrap().into();
                malicious_validator.burn(self.reward_rate * self.punishment * dec!("5")); //malicious validator will be punished 5 times untruthful validator

                    self.controller_badge.authorize(|| {

                    self.active_validators.iter().for_each(|(&address, _weight)| {

                        let validator: Validator = address.into();
                            
                        if validator.get_status() {

                            if validator.get_vote() {validator.burn(self.reward_rate * self.punishment)}
                            else {validator.mint(self.reward_rate)}

                        }
                    }); 
                });
                    
                }
            }

            info!("End round {} of NeuRacle", self.system_time);

            self.system_time = current/self.round_length + 1;

            self.round_start = false
            
        }

        pub fn new_stable_coin_project(&mut self, pegged_to: String, api: String) -> ComponentAddress {

            if !self.datas.contains_key(&api) {
                
                self.datas.insert(api.clone(), String::default());
            
            };

            let neuracle: ComponentAddress = Runtime::actor().component_address().unwrap();

            let user_id: NonFungibleId = NonFungibleId::random();

            let data_badge = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.user_badge)
                .mint_non_fungible(&user_id, UserData{
                    end: 0,
                    api: api.clone()
                })
            });

            let controller_badge =self.mint_controller_badge.authorize(|| {
                borrow_resource_manager!(self.controller_badge.resource_address())
                .mint(dec!("1"))
            });

            let stable_coin_project_address = NStableCoin::new(self.neura, pegged_to.clone(), neuracle, controller_badge, data_badge, self.fee_stablecoin);

            self.stable_coins.insert(stable_coin_project_address, pegged_to + "NStable Coin");

            return stable_coin_project_address

        }

        pub fn show_validators(&self) {
            info!("Begin show data pools, format: (validator_id: staked_weight)|| {:?}", self.validators)
        }

        pub fn show_apis(&self) {
            info!("Begin show data apis, {:?}", self.datas.keys())
        }


        pub fn show_stable_coins(&self) {
            info!("Begin show stable coin projects, format: (project_address: project_name)|| {:?}", self.stable_coins)
        }

        pub fn get_apis(&self) -> Vec<String>{
            self.datas.keys().cloned().collect()
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