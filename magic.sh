#!/usr/bin/env bash

for i in {1..6}; do
  echo "Automagically update src/lib.rs 🪄"
  mv "./demo/lib${i}.rs" ./src/lib.rs
  $SHELL
done

# This is a dumb convenient way to run the demo without copy-pasting each code
# example, just hit Ctrl+D instead ;)
