#!/usr/bin/env bash

#set -x
set -e

FLOOR=100
CEIL=200

RESULT=`shuf -i ${FLOOR}-${CEIL} -n 1`

echo $RESULT