# Prerequisites
- OpenSSL (see [here](https://www.openssl.org/) for instructions)

# Instructions
```bash
cargo run -q -- --help
```

```text
signing 1.0
TrueLayer
A small command line interface to sign POST requests for Payouts API

USAGE:
    signing --payload-filename <payload-filename> --private-key-filename <private-key-filename> --certificate-id <certificate-id>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --certificate-id <certificate-id>
            The certificate id associated to the public certificate you uploaded in TrueLayer's Console. The certificate
            id can be retrieved in the Payouts Setting section. It will be used as the `kid` header in the JWS
        --payload-filename <payload-filename>            
            The filename of the payload you want to sign, in JSON format
        --private-key-filename <private-key-filename>
            The filename of the Elliptic Curve private key used to sign, in PEM format
```

# Example 

This assumes you have generated a key pair using the `generate_key_pair.sh` script in the root folder of the project.

```bash
# Assumes you are running from within the `rust/sign` directory
cargo run -q -- --private-key-filename ../../ec512-private-key.pem \
    --payload-filename ../../payload.json \
    --certificate-id fa07d2bb-f25e-4805-b69c-211136c84d7b
```

The script returns both a full-blown [JWS](https://tools.ietf.org/html/rfc7515) and a [JWS with detached content](https://tools.ietf.org/html/rfc7515#appendix-F) (the one to be passed in the `X-Tl-Signature` header when making a request).
