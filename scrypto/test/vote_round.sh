#!/usr/bin/env bash

#set -x
set -e

source ./update_data.sh

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

output=`resim run ./transaction_manifest/start_round |  awk '/HashMap<String, String>/ {print $FS}'`

output2=`echo $output | cut -d "(" -f2 | cut -d ")" -f1 | sed 's/"//g'`

IFS=', '

read -a RESULTSVOTE <<< "$output2"

export API21=${RESULTSVOTE[0]}
export RESULT21=${RESULTSVOTE[1]}
export API22=${RESULTSVOTE[2]}
export RESULT22=${RESULTSVOTE[3]}
export API23=${RESULTSVOTE[4]}
export RESULT23=${RESULTSVOTE[5]}
export API24=${RESULTSVOTE[6]}
export RESULT24=${RESULTSVOTE[7]}
export API25=${RESULTSVOTE[8]}
export RESULT25=${RESULTSVOTE[9]}

declare -A hm1
declare -A hm2

hm1["$API1"]="$RESULT1"
hm1["$API2"]="$RESULT2"
hm1["$API3"]="$RESULT3"
hm1["$API4"]="$RESULT4"
hm1["$API5"]="$RESULT5"

hm2["$API21"]="$RESULT21"
hm2["$API22"]="$RESULT22"
hm2["$API23"]="$RESULT23"
hm2["$API24"]="$RESULT24"
hm2["$API25"]="$RESULT25"

for k in ${!hm1[@]}

do
    if [ "${hm1[$k]}" == "${hm2[$k]}" ]
    then VOTE=true && continue
    else VOTE=false && break
    fi
done

logc "$VOTE"

export VOTE

if $VOTE
then export VOTEM=false
else export VOTEM=true
fi

logc "Validators vote on results"

resim set-default-account $VAL1_ACC $VAL1_PIV
export VALVO_ADDRESS=$VAL1_ADDRESS
export VALVO_ACC=$VAL1_ACC
resim run ./transaction_manifest/vote

resim set-default-account $VAL2_ACC $VAL2_PIV
export VALVO_ADDRESS=$VAL2_ADDRESS
export VALVO_ACC=$VAL2_ACC
resim run ./transaction_manifest/vote

logy "Let validator 3 to forgot vote"


resim set-default-account $VAL4_ACC $VAL4_PIV
export VALVO_ADDRESS=$VAL4_ADDRESS
export VALVO_ACC=$VAL4_ACC
resim run ./transaction_manifest/vote

logy "Let one validator to vote untruthful"
resim set-default-account $VAL5_ACC $VAL5_PIV
export VALVO_ADDRESS=$VAL5_ADDRESS
export VALVO_ACC=$VAL5_ACC
resim run ./transaction_manifest/vote_malicious

logc "Admin begin conclude the voting round"
resim set-default-account $ADMIN_ACC $ADMIN_PIV
resim run ./transaction_manifest/end_round

logc "Check user's staked amount again, this should show user2, user5 got reward, user4 got no reward and user3 lose some NAR token"
resim set-default-account $USER2_ACC $USER2_PIV
resim call-method $VAL1_ADDRESS show_my_stake_amount 1,$STAKER_VAL1_BADGE

resim set-default-account $USER5_ACC $USER5_PIV
resim call-method $VAL4_ADDRESS show_my_stake_amount 1,$STAKER_VAL4_BADGE

resim set-default-account $USER4_ACC $USER4_PIV
resim call-method $VAL3_ADDRESS show_my_stake_amount 1,$STAKER_VAL3_BADGE

resim set-default-account $USER3_ACC $USER3_PIV
resim call-method $VAL5_ADDRESS show_my_stake_amount 1,$STAKER_VAL5_BADGE

completed