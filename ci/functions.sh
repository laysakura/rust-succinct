function downcase() { tr "[:upper:]" "[:lower:]"; }
function upcase() { tr "[:lower:]" "[:upper:]"; }

function ostype() { uname| downcase; }
function is_linux() { [[ `ostype` == linux* ]]; }
function is_osx() { [[ `ostype` == darwin* ]]; }

function git_branch() { git symbolic-ref --short HEAD; }
