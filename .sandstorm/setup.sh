#!/bin/bash

set -euo pipefail

source /opt/app/.sandstorm/environment

export DEBIAN_FRONTEND=noninteractive

apt-get update && apt-get upgrade

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
