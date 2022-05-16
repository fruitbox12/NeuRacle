#!/bin/bash

#set -x
set -e

source ./users_and_apis.sh

logc "Admin instantiate new algorithm stable coin project, pegged in USD"
logy "Because NAR token is still haven't launched yet, let's use XRD instead"
logy "This prototype also haven't implement a fee mechanism yet"

resim set-default-account $ADMIN_ACC $ADMIN_PIV
output=`resim run ./transaction_manifest/stable_coin | awk '/USDN: |USDNStable Coin address: / {print $NF}'`
export USDN=`echo $output | cut -d " " -f1`
export SC_COMP=`echo $output | cut -d " " -f2`

logc "Run a data voting round to get newest data."
source ./data_refresh_round.sh

logc "Begin swap"

resim set-default-account $ADMIN_ACC $ADMIN_PIV
export NUM=15000 #You can edit this
export RS=$NEURA
resim run ./transaction_manifest/auto_swap

export NUM=120 #You can edit this
export RS=$USDN
resim run ./transaction_manifest/auto_swap

completed