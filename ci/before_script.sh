#!/usr/bin/env bash
set -eux

SCRIPT_DIR=$(cd $(dirname $0); pwd)
. $SCRIPT_DIR/functions.sh

rustup component add rustfmt
