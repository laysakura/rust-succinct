#!/usr/bin/env bash
set -eux

SCRIPT_DIR=$(cd $(dirname $0); pwd)
. $SCRIPT_DIR/functions.sh

rustup component add rustfmt

if [ `git_branch` = 'feature/criterion-html-publish-w-gnuplot' ]; then
    is_osx && brew install gnuplot
    is_linux && sudo apt-get install -y gnuplot-nox
fi
