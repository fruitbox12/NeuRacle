# NeuRacle
Neuracle - Decentralized Trustless Oracle on Radix network

Oracle problem: https://encyclopedia.pub/entry/2959

solution: https://medium.com/@jameslee777/decentralized-trustless-oracles-dto-by-piggybacking-on-timestamp-consensus-rules-2adce34d67b6

more specified solution Komodo Trustless Oracles: https://komodoplatform.com/en/blog/the-promise-of-smart-contracts-and-the-oracle-problem/

WHY RADIX: https://www.radixdlt.com/post/cerberus-infographic-series-chapter-i

Basically:

Data Providers ~ Validator Nodes.
NeuRacle Ecosystem (Layer 2) ~ Shardspace.
lively data from an online source ~ Shard.
1 specific data in a particular time ~ Transaction.
Validated Data ~ Reaching Consensus.
Users = Users.
Components = Components.

Validators are the people that host NeuRacle Gateway off-chain and ensure the security, connectivity of the Gateway.

NeuRacle Gateway is an off-chain entity that will play role as a medium to fetch data source on-chain, use the source to fetch data off-chain, and automatize voting, feeding data on-chain on validator behalf.

NeuRacle will let users to choose data from any online source they want from, they can also choose on-chain aggregrated data but that will ofc more costly.

Users will have to take responsibility on the accessibility of sources. The data source can be public, or private data. User will have to provide an online and accessible API (and key, if that's a private data) to the NeuRacle Gateway. NeuRacle will help user to choose (or host) an API, but will not buy the data on user's behalf.

To help the Gateway feedback the precise data that users need, the data source API should return only that one specific data. It shouldn't be a chunk of datas.

After make a request, users can fetch on-chain data through NeuRacle component, if it deemed non-accessible, users will only receive blank data.

User make request for data from 1 source will have to provide NER token. The more they provide, the longer they can get validated data. All NER token used will be burn.

Why one source?

Aggregrate data on-chain will be much more computation costly.

Moreover, not every users will want aggregrated data.

Eg: Bob operating a USX stable coin project and using aggregrated "XRD/USD last price" data feed to the system, let user exchange XRD/USX on the feeded data. However, most of the time, there is 1 particular exchange that have it's XRD/USD price lower than the aggregrated data, and unfortunately most of your user use that exchange, so they complain about the data's authenticity. Now Bob have to use that exchange source data instead.

This won't just stay on crypto world, on real world too, different address, location, seller will provide different information. USA oil price will ofc different from Russia oil price. Pork from your locally will ofc different from the farm.

Seller, provider, manufacturer can also use NeuRacle service to validate their product's price, and give buyer a nft prove their possesion of the product without having to sell it in reality! Eg: real estate, automobile, oil, gold, even home grocery...

Off-chain identity can also do data aggregration and ensure some degree of decentralization (Eg: Flux). User can also buy that data and make a data feeding request on NeuRacle.

For simplication, this prototype will only serve non-aggregrated and assume that we already have the off-chain Gateway. Data will also be validated (Reaching Consensus) in 1 round of voting.