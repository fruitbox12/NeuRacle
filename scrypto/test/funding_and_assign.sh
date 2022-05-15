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

output=`resim run ./transaction_manifest/instantiate | awk '/Admin badge address: |Validator badge address: |User badge: |Neura: |Component: / {print $NF}'`
export ADMIN_BADGE=`echo $output | cut -d " " -f1`
export VALIDATOR_BADGE=`echo $output | cut -d " " -f2`
export USER_BADGE=`echo $output | cut -d " " -f3`
export NEURA=`echo $output | cut -d " " -f4`
export COMP=`echo $output | cut -d " " -f5`

logc "Distribute 1000 NAR to each validators and users"
resim run ./transaction_manifest/transfer

logc "Mint 5 validator badges with different name, location, staking fee and distribute to validators"
output=`resim run ./transaction_manifest/validator | awk '/val1 Validator Address: |val1 Staker Badge: |val2 Validator Address: |val2 Staker Badge: |val3 Validator Address: |val3 Staker Badge: |val4 Validator Address: |val4 Staker Badge: |val5 Validator Address: |val5 Staker Badge: / {print $NF}'`
export VAL1_ADDRESS=`echo $output | cut -d " " -f1`
export STAKER_VAL1_BADGE=`echo $output | cut -d " " -f2`
export VAL2_ADDRESS=`echo $output | cut -d " " -f3`
export STAKER_VAL2_BADGE=`echo $output | cut -d " " -f4`
export VAL3_ADDRESS=`echo $output | cut -d " " -f5`
export STAKER_VAL3_BADGE=`echo $output | cut -d " " -f6`
export VAL4_ADDRESS=`echo $output | cut -d " " -f7`
export STAKER_VAL4_BADGE=`echo $output | cut -d " " -f8`
export VAL5_ADDRESS=`echo $output | cut -d " " -f9`
export STAKER_VAL5_BADGE=`echo $output | cut -d " " -f10`

completed
