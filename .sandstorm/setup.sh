#!/bin/bash

set -euo pipefail

source /opt/app/.sandstorm/environment

export DEBIAN_FRONTEND=noninteractive

apt-get update && apt-get upgrade

# Install the Rust toolchain
apt-get install --yes gcc-multilib

if [ ! -f "$RUSTUP_INIT" ] || [ ! "$(/usr/bin/sha256sum -c "$RUSTUP_INIT_SHA256")" ]; then
	$CURL -o "$RUSTUP_INIT" "$RUSTUP_INIT_URL"
	if [ ! "$(/usr/bin/sha256sum -c "$RUSTUP_INIT_SHA256")" ]; then
		echo "SHA-256 check of rustup-init failed."
		exit 1
	fi
fi
if [ ! -d "$RUSTUP_HOME" ] || [ ! -d "$CARGO_HOME" ]; then
	chmod 755 "$RUSTUP_INIT"
	RUSTUP_HOME="$RUSTUP_HOME" CARGO_HOME="$CARGO_HOME" $RUSTUP_INIT --default-host "$RUSTUP_INIT_HOST_TRIPLE" --default-toolchain stable --no-modify-path -y
fi

chown -R "$VAGRANT_USER":"$VAGRANT_GROUP" "$CARGO_HOME"

# Install Cap'n Proto
apt-get install --yes g++

if [ ! -f "/usr/local/bin/capnpc" ]; then
	$CURL -O "$CAPNPROTO_URL"
	if [ ! "$(/usr/bin/sha256sum -c "$CAPNPROTO_SHA256")" ]; then
		echo "SHA-256 check of $CAPNPROTO_TGZ failed."
		exit 1
	fi
	tar zxf "$CAPNPROTO_TGZ"
	cd "$CAPNPROTO_BASENAME"
	./configure && make -j6 check && make install
fi

exit 0
