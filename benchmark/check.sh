#!/bin/bash

BENCH_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
AXE="$BENCH_DIR/../target/release/axe"


if [ ! -f $AXE ]; then
  echo "please run 'cargo build --release'"
  exit 1
fi


case $1 in
  plain)
    $BENCH_DIR/logspam-plain | $AXE filter | pv --line-mode > /dev/null

    ;;

  password)
    $BENCH_DIR/logspam-password | $AXE filter | pv --line-mode > /dev/null

    ;;

  *)
    echo "Usage: $0 {plain|password}"

    ;;

esac

