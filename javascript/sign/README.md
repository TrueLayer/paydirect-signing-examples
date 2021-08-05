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
Options:
  --help                  Show help                                    [boolean]
  --version               Show version number                          [boolean]
  --certificate-id        The certificate id associated to the public
                          certificate you uploaded in TrueLayer's Console. The
                          certificate id can be retrieved in the Payouts Setting
                          section. It will be used as the `kid` header in the
                          JWS                                [string] [required]
  --payload-filename      The filename of the payload you want to sign, in JSON
                          format                             [string] [required]
  --private-key-filename  The filename of the Elliptic Curve private key used to
                          sign, in PEM format                [string] [required]
```

# Example 

This assumes you have generated a key pair using the `generate_key_pair.sh` script in the root folder of the project.

```bash
# Assumes you are running from within the `javascript/sign` directory
npm run signing -- --private-key-filename ../../ec512-private-key.pem --payload-filename ../../payload.json --certificate-id fa07d2bb-f25e-4805-b69c-211136c84d7b
```

The script returns both a full-blown [JWS](https://tools.ietf.org/html/rfc7515) and a [JWS with detached content](https://tools.ietf.org/html/rfc7515#appendix-F) (the one to be passed in the `X-Tl-Signature` header when making a request).
