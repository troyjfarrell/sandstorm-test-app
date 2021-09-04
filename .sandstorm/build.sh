#!/bin/bash
set -euo pipefail

source /opt/app/.sandstorm/environment

export CARGO_HOME="$CARGO_HOME"
export RUSTUP_HOME="$RUSTUP_HOME"
source "$CARGO_HOME"/env

cd /opt/app && cargo build
