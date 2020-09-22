# Payouts API - Signing Examples
 
This repository provides a collection of code samples illustrating how to perform request signing for 
[Payouts API](https://docs.truelayer.com/#payouts-api-v1) using different programming languages.

Instructions on how to run each code sample are provided in the respective README.md files.

# Generating a key pair

All examples require an EC key pair to work correctly.
You can generate one using
```bash
./generate_key_pair.sh
```

The script requires [`openssl`](https://www.openssl.org/).