#!/usr/bin/env bash

#set -x
set -e

# Use funding_and_assign
source ./funding_and_assign.sh

logc "Neura holders staking with different amount in different validator"

resim set-default-account $VAL1_ACC $VAL1_PIV
resim run stake1

resim set-default-account $VAL2_ACC $VAL2_PIV
resim run stake2

resim set-default-account $VAL3_ACC $VAL3_PIV
resim run stake3

resim set-default-account $VAL4_ACC $VAL4_PIV
resim run stake4

resim set-default-account $VAL5_ACC $VAL5_PIV
resim run stake5

resim set-default-account $USER1_ACC $USER1_PIV
resim run stake6

resim set-default-account $USER2_ACC $USER2_PIV
resim run stake7

resim set-default-account $USER3_ACC $USER3_PIV
resim run stake8

resim set-default-account $USER4_ACC $USER4_PIV
resim run stake9

resim set-default-account $USER5_ACC $USER5_PIV
resim run stake10

logc "Some Neura holders addstake, unstake, try stop unstake and withdraw in unstaking period."

resim set-default-account $VAL4_ACC $VAL4_PIV
resim run add_stake

resim set-default-account $USER2_ACC $USER2_PIV
resim run unstake

resim set-default-account $USER4_ACC $USER4_PIV
resim run unstake2
resim run stop_unstake

resim set-default-account $USER5_ACC $USER5_PIV
resim run unstake3
resim run withdraw