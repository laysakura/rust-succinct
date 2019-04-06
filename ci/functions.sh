downcase() { tr "[:upper:]" "[:lower:]"; }
upcase() { tr "[:lower:]" "[:upper:]"; }

ostype() { uname| downcase; }
is_linux() { [[ `ostype` == linux* ]]; }
is_osx() { [[ `ostype` == darwin* ]]; }

git_branch() { git symbolic-ref --short HEAD; }
