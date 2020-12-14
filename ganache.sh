#!/bin/sh
# This script is intended to run in a container automatically.
# It would be useless to try launch it out of that context.

HOST=0.0.0.0 # listen all addresses to work in a container properly
PORT=8545

echo "Starting ganache..."
ganache-cli --host $HOST --port $PORT \
	    --account "0x0000000000000000000000000000000000000000000000000000000000000Ace,0" \
	    --account "0x0000000000000000000000000000000000000000000000000000000000Facade,1000000000000000000000"
