#!/usr/bin/env bash

#set -x
set -e

# Use init
source ./init.sh

logc "Change into Admin account to instantiate NeuRacle component and become NeuRacle Admin"
resim set-default-account $ADMIN_ACC $ADMIN_PIV

logy "Set input parameters to NeuRacle component as:"

logp "Validator cap: 100"
logp "Round length: 1"
logp "Pay rate: 1"
logp "Fee stablecoin: 0.3"
logp "Unstake delay: 500"
logp "Reward rate: 0.0015"
logp "Punishment: 10"

logy "Check doc to study about these parameters"

resim run instantiate
ADMIN_BADGE=03c279f7c2829a0e3c3ddfb25ae87df22c57db9afaf53c9d0de998
VALIDATOR_BADGE=035114d229cd9261483401938a8cfe6ce6ffc7afdea4dfd073d8c6
USER_BADGE=039bbede8652d2ad940205970bb9723aeabb695c69f799cc235545
NEURA=0351045a85127b83f17fcd5ab751ef85ded4144cf900d98be1041a
COMP=02b0c6688cb87e4164d9cce01f112028c458c7e248ccb6d9e3a55a

logc "Distribute 1000 NAR to each validators and users"
resim run transfer

logc "Mint 5 validator badges with different name, location, staking fee and distribute to validators"
resim run validator
VAL1_ADDRESS=021ffdc25fe0dbdc375c7ba873b7cc16b5071d281f0dec99a06be7
STAKER_VAL1_BADGE=03f9818f8ca61679c456cdfeea99e83ef1e5b8f5a1be603740a425
VAL2_ADDRESS=028b512dd9eb28731c5bfdb7d67a94bb52d9a676939bc99f3e9dcc
VAL3_ADDRESS=02b478f168948c9b69887281650c3de8340e992c348bde24b1c312
VAL4_ADDRESS=026201c9f6846b63bc6117705da430954068fbce67a4aec4640336
VAL5_ADDRESS=0210e264838177a3e47dee61b21a3f6603baf7a13999688fe99095

compeleted
