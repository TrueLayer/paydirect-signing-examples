#!/usr/bin/env sh
# Generate private key
DIR=${1:-./}
openssl ecparam -genkey -name secp521r1 -noout -out "${DIR}ec512-private-key.pem"
# Extract public key
openssl ec -in "${DIR}ec512-private-key.pem" -pubout -out "${DIR}ec512-public-key.pem"
