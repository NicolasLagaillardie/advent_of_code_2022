#!/bin/bash

set -e


ts=$(date +%s)
cargo test
echo $(($(date +%s) - $ts))
    