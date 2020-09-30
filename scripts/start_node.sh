#!/bin/bash
# input: $1 account name: alice/bob
# $2 index: starting with 0, 0 is seed node 
# $3 peerId of seed node, used to bootstrap, not needed for seed node

parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$parent_path"

#replace it to your program location, e.g. ../target/release/e2-chain
program=../target/debug/e2-chain

port=$((30333+$2))
ws_port=$((9944+$2))
rpc_port=$((9933+$2))

# start at genesis block with fresh database
rm /tmp/$1 2&>/dev/null || true

if [ ! -z "$3" ]
then
    echo "starting node $1 ..."
    $program  --base-path /tmp/$1 \
    --chain local \
    --$1 \
    --port $port \
    --ws-port $ws_port \
    --rpc-port $rpc_port \
    --validator \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$3
    
else
    echo "starting seed node $1 ..."
    $program  --base-path /tmp/$1 \
    --chain local \
    --$1 \
    --port $port \
    --ws-port $ws_port \
    --rpc-port $rpc_port \
    --validator 
    
fi

