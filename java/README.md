# Prerequisites

- NPM (see [here](https://www.npmjs.com/get-npm) for instructions)

# Instructions

First install project dependencies:
```bash
npm install
```

The run help for instructions:
```bash
npm run signing -- --help
```

```text
Usage: signing [-hV] --certificate-id=<certificateId>
               --payload-filename=<payloadFileName>
               --private-key-filename=<privateKeyFilename>
A small command line interface to sign POST requests for Payouts API
      --certificate-id=<certificateId>
                  The certificate id associated to the public certificate you
                    uploaded in TrueLayer's Console. The certificate id can be
                    retrieved in the Payouts Setting section. It will be used
                    as the `kid` header in the JWS
  -h, --help      Show this help message and exit.
      --payload-filename=<payloadFileName>
                  The filename of the payload you want to sign, in JSON format
      --private-key-filename=<privateKeyFilename>
                  The filename of the Elliptic Curve private key used to sign,
                    in PEM format
  -V, --version   Print version information and exit.
```

# Example 

This assumes you have generated a key pair using the `generate_key_pair.sh` script in the root folder of the project.

```bash
# Assumes you are running from within the `javascript` directory
npm run signing -- --private-key-filename ../ec512-private-key.pem --payload-filename ../payload.json --certificate-id fa07d2bb-f25e-4805-b69c-211136c84d7b
```

The script returns both a full-blown [JWS](https://tools.ietf.org/html/rfc7515) and a [JWS with detached content](https://tools.ietf.org/html/rfc7515#appendix-F) (the one to be passed in the `X-Tl-Signature` header when making a request).