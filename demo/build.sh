
#!/usr/bin/env sh
#set -x
set -e

# pretty up the CLI output (from devmannic)
source ./logging.sh

log "start with fresh ledger state"
resim reset

XRD=030000000000000000000000000000000000000000000000000004

# initial
log "publish NeuRacle"

PACKAGE=01fa9b2019a241ee072d0dc381be47c9c44350f2464d2f24fbe052

log "simulate a reasonable non-zero epoch for testing"
# for consistency testing
resim set-current-epoch 10