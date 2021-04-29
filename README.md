# Payouts API - Signing Examples

This repository provides a collection of code samples illustrating how to perform request signing for
[Payouts API](https://docs.truelayer.com/#payouts-api-v1) using different programming languages.

## Usage
Instructions on how to run each code sample are provided in the respective README.md files.

These code samples are provided as examples for you to understand how to perform request signing. They are not meant to be used to sign a request and then submit it with a different tool like Postman or cURL. The code or tool used to sign the request should also be used to POST the request, otherwise the encoding of the payload will no longer match the signature and your request will fail.

## Generating a key pair

All examples require an EC key pair to work correctly.
You can generate one using

```bash
./generate_key_pair.sh
```

The script requires [`openssl`](https://www.openssl.org/).

## Common Validation Errors

The payload parsed for signing *must* be bytewise equivialent to the payload sent to our API's. The most common cause of issues are errant control characters (often seen in cli implementations where a trailing newline '\n' is inserted/removed).
