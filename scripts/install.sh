#!/usr/bin/env bash

root=$(git rev-parse --show-toplevel 2> /dev/null)
gitExecPath=$(git --exec-path)
cmdPath=$root/target/release/git-ignore
target=$gitExecPath/git-ignore

# if target exists, remove it
if [ -f "$target" ]; then
    rm "$target"
fi

# Copy the binary to the git exec path
cp "$cmdPath" "$target"
