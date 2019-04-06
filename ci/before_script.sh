#!/bin/sh

. functions.sh

rustup component add rustfmt

if [ `git_branch` = 'master' ]; then
    is_osx && brew install gnuplot
    is_linux && sudo apt-get install -y gnuplot
fi
