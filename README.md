# NeuRacle: Layer 2 Oracle solution on Radix Network

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

NeuRacle is a PoS Layer 2 solution built on Radix Ledger that provide decentralized, trustless data validation service to bring-in off-chain data.

## Oracle Trilemma:

Most traditional Oracle recent day come to the same [problem](https://encyclopedia.pub/entry/2959), that have to either compromised on trustless (using trusted identities to bring data on-chain, eg: [ChainLink](https://james-sangalli.medium.com/why-chainlink-is-not-the-oracle-of-the-future-8bb859a81947#:~:text=ChainLink%20does%20not%20have%20a,centralised%20verification%20and%20dispute%20resolution.), finality (like using optimistic oracle, bet to bring data on-chain, eg: [UMA](https://umaproject.org/products/optimistic-oracle)), or security. It's almost the same as the [blockchain trilemma](https://www.ledger.com/academy/what-is-the-blockchain-trilemma).

## From Oracle to Distributed Ledger Technology:

Because the Oracle trilemma is almost the same as blockchain trilemma, choose a blockchain solution as an [oracle solution](https://medium.com/@jameslee777/decentralized-trustless-oracles-dto-by-piggybacking-on-timestamp-consensus-rules-2adce34d67b6) will be an innovated approach. There already some successful Oracle that are using this approach to challenge the Oracle Trilemma, eg: [Komodo Trustless Oracles](https://komodoplatform.com/en/blog/the-promise-of-smart-contracts-and-the-oracle-problem/).

Though, blockchain can't solve it's own trilemma.

[Cerberus Concensus Model](https://assets.website-files.com/6053f7fca5bf627283b582c2/608811e3f5d21f235392fee1_Cerberus-Whitepaper-v1.01.pdf) is a DLT concensus model that (on theory and testnet) solved all these trilemma and maintain atomic composability at the same time. While inspired by Komodo to use Consensus Models for validating data off-chain and bring on-chain, NeuRacle will further advance the innovation in oracle space by building on the Radix Network and utilize Cerberus Concensus Model.

## NeuRacle Solution

As an utilization of Cerberus Concensus Model, NeuRacle will have some similar design, for short:

Data Providers ~ [Validator Nodes](https://www.radixdlt.com/post/cerberus-infographic-series-chapter-ii).
NeuRacle Ecosystem (Layer 2) ~ [Shardspace](https://www.radixdlt.com/post/cerberus-infographic-series-chapter-ii).
Lively data from an online source ~ [Shard](https://www.radixdlt.com/post/cerberus-infographic-series-chapter-iv).
1 specific data in a particular time ~ [Transaction](https://www.radixdlt.com/post/cerberus-infographic-series-chapter-iii).
Validated Data ~ [Reaching Consensus](https://www.radixdlt.com/post/cerberus-infographic-series-chapter-v).
Sybil Resistance by PoS = [Sybil Resistance by PoS](https://www.radixdlt.com/post/cerberus-infographic-series-chapter-vii).
Users = Users.
Components = Components.

## Quick Start:

## System Explaination

For a simple showcase, this prototype will be un-sharded, that mean each validators will validate all datas at the same time (Not divided into validator sets to bring more scalability or divided into data sources to bring more security). Datas will also be validated (Reaching Consensus) in 1 round of voting.

### NeuRacle ecosystem's entities:

There are 3 mains entites in NeuRacle ecosystem: **Users**, **Validators** and **NeuRacle Gateway**.

**Validators**, or Data Providers are the people that host NeuRacle Gateway off-chain and ensure the security, connectivity of the Gateway.

**NeuRacle Gateway** is an **off-chain entity** that will play role as a medium to automatically fetch data sources on-chain, use the source to fetch data off-chain, and feeding that data on-chain on validator behalf. To further prevent exploit, the key (or badge) will be provided to the Validator Local Gateway instead of the Validator himself. Change fee rate, collect validator fee also have to executed through NeuRacle Gateway.

NeuRacle will let users to choose data from any online source they want from, they can also choose on-chain aggregrated data but that will ofc more costly.

**Users** will have to take responsibility on the accessibility of sources. The data source can be public, or private data. User will have to provide an online and accessible API (and key, if that's a private data) to the NeuRacle Gateway. NeuRacle will also help user to choose (or host) an API that return the exact data user want, **but will not buy the data on user's behalf**.

To help the Gateway feedback the precise data that users need, the data source API should return only that one specific data. It shouldn't be a chunk of datas.

After make a request, users can fetch on-chain data through NeuRacle component, if it deemed non-accessible, users will only receive blank data.

User make request for data from 1 source will have to provide NER token. The more they provide, the longer they can get validated data. All NER token used will be burn.

### Why one source?

Aggregrate data on-chain will be much more computation costly.

Moreover, not every users will want aggregrated data.

Eg: Bob operating a USX stable coin project and using aggregrated "XRD/USD last price" data feed to the system, let user exchange XRD/USX on the feeded data. However, most of the time, there is 1 particular exchange that have it's XRD/USD price lower than the aggregrated data, and unfortunately most of your user use that exchange, so they complain about the data's authenticity. Now Bob have to use that exchange source data instead.

This won't just stay on crypto world, on real world too, different address, location, seller will provide different information. USA oil price will ofc different from Russia oil price. Pork from your locally will ofc different from the farm.

Seller, provider, manufacturer can also use NeuRacle service to validate their product's price, and give buyer a nft prove their possesion of the product **without having to know the customer identity!** Eg: real estate, automobile, oil, gold, even home grocery...

Off-chain identity can also do data aggregration and ensure some degree of decentralization (Eg: Flux, SurpraOracle). User can also buy that data and make a data feeding request on NeuRacle.

### Data refreshing round.

Anyone can choose a validator to stake, receive reward based on that validator contribution to the network. The Sybil Resistance mechanism worked the same as Radix Network.

After every round, data will be refreshed, NAR token will also be minted to reward (or burned to punish) validators.

The round call and call-off will run by first-come-first-serve mechanism. The individual call (or call-off) a round will receive a reasonable reward. This incentive is to ensure that data valitation round will happen and concluded right after they passed requirement.

Round call requirement is the round-length limited time.

Time unit of on-chain NeuRacle is transactions history or epoch length.

Round length is the limited time between each data validation round. Data can be validated every 10tx, 100tx or 1epoch,...

Because this time unit is unstable occasionally, the stability of data stream will have to depend on Admin monitor.

Beside data sources, NeuRacle Gateway also have to keep track of the Radix Ledger new history to see if the new NeuRacle round started or not.

NeuRacle Gateway will update data on Validators behalf right after round start. After update, the validator will deemed active in that round.

Round concluded requirement is >2/3 active validators.

Datas with >2/3 staked weight of that round will also be validated.

## What bad things might happend on NeuRacle?

**Vulnerability** (or break security): NeuRacle has the same Sybil Resistance as Radix Network, malicious entities will need >1/3 staked value  to break liveness, >2/3 staked value to really conduct a meaningful attack. Based on game-theory, that attack will really hard and costly. With sharded NeuRacle, the validator sets, as well as the data sources they may validate in the next round, will all be randomized, make the attack become almost impossible.
    
**Break liveness**: The data update will happen at the "almost same time" in every validators. Assume we have some validator
with low performance that will make the data "slightly different", that validator will right away deemed as "untruthful" and punished. This punish and reward mechanism will ensure all validator to host and ensure the Gateway in the best performance.


## Use case:

**Algorithmed stable coin**: Despite the current failure of Luna Ecosystem, I still have a high trust that **stable coin is a bridge between Crypto - Fiat worlds, so it should have traits of both**. The algorithmed stablecoin on DeFi world can be backed by algorithm **and** at the same time backed by fiat when traded in real world.

## Future work


*I'm still an amateur on cryptography and distributed technology, in this work there may still contain something wrong or haven't taken into account. I'm glad to have any contributor to this work.*