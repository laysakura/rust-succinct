downcase() { tr "[:upper:]" "[:lower:]"; }
upcase() { tr "[:lower:]" "[:upper:]"; }

ostype() { uname| downcase; }
is_linux() { [[ `ostype` == linux* ]]; }
is_osx() { [[ `ostype` == darwin* ]]; }

git_branch() {
    if [ "$TRAVIS_PULL_REQUEST" == "false" ]; then
        echo -n $TRAVIS_BRANCH;
    else
        echo -n $TRAVIS_PULL_REQUEST_BRANCH;
    fi
}
