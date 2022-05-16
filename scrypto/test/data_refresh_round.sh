#!/usr/bin/env bash

#set -x
set -e

logc "Check user's staked amount"
resim set-default-account $USER2_ACC $USER2_PIV
resim call-method $VAL1_ADDRESS show_my_stake_amount 1,$STAKER_VAL1_BADGE

resim set-default-account $USER5_ACC $USER5_PIV
resim call-method $VAL4_ADDRESS show_my_stake_amount 1,$STAKER_VAL4_BADGE

resim set-default-account $USER4_ACC $USER4_PIV
resim call-method $VAL3_ADDRESS show_my_stake_amount 1,$STAKER_VAL3_BADGE

resim set-default-account $USER3_ACC $USER3_PIV
resim call-method $VAL5_ADDRESS show_my_stake_amount 1,$STAKER_VAL5_BADGE

logc "NeuRacle Admin start a round"

resim set-default-account $ADMIN_ACC $ADMIN_PIV

resim run ./transaction_manifest/start_round 

source ./update_data.sh

logc "Admin begin conclude the round"
resim set-default-account $ADMIN_ACC $ADMIN_PIV
resim run ./transaction_manifest/end_round

logc "Check user's staked amount again, this should show user2, user3 got reward, user4 got no reward and user5 lose some NAR token"
resim set-default-account $USER2_ACC $USER2_PIV
resim call-method $VAL1_ADDRESS show_my_stake_amount 1,$STAKER_VAL1_BADGE

resim set-default-account $USER5_ACC $USER5_PIV
resim call-method $VAL4_ADDRESS show_my_stake_amount 1,$STAKER_VAL4_BADGE

resim set-default-account $USER4_ACC $USER4_PIV
resim call-method $VAL3_ADDRESS show_my_stake_amount 1,$STAKER_VAL3_BADGE

resim set-default-account $USER3_ACC $USER3_PIV
resim call-method $VAL5_ADDRESS show_my_stake_amount 1,$STAKER_VAL5_BADGE

completed