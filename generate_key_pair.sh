#!/usr/bin/env bash
# Generate private key
openssl ecparam -genkey -name secp521r1 -noout -out ec512-private-key.pem
# Extract public key
openssl ec -in ec512-private-key.pem -pubout -out ec512-public-key.pem