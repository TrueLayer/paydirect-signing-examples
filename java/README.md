# Prerequisites

- Java 11 or greater (Feel free to use [SdkMan!](https://sdkman.io/) for a painless setup)

Build the project with the Gradle wrapper included in this sample:
```bash
./gradlew build 
```

# Request signing instructions
The run help for instructions:
```bash
java -jar sign/build/libs/sign.jar -h
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
      --payload-filename=<payloadFileName>gs
                  The filename of the payload you want to sign, in JSON format
      --private-key-filename=<privateKeyFilename>
                  The filename of the Elliptic Curve private key used to sign,
                    in PEM format
  -V, --version   Print version information and exit.
```

## Example 

This assumes you have generated a key pair using the `generate_key_pair.sh` script in the root folder of the project.

```bash
# Assumes you are running from within the `java` directory
java -jar sign/build/libs/sign.jar --certificate-id fa07d2bb-f25e-4805-b69c-211136c84d7b --payload-filename ../payload.json --private-key-filename ../ec512-private-key.pem
```

The script returns both a full-blown [JWS](https://tools.ietf.org/html/rfc7515) and a [JWS with detached content](https://tools.ietf.org/html/rfc7515#appendix-F) (the one to be passed in the `X-TL-Signature` header when making a request).

# Webhook verification instructions
Webhook jws signature verification using _com.auth0:java-jwt_ & _com.auth0:jwks-rsa_.

```text
Usage: verify-webhook [-hV] --tl-signature=<tlSignature>
                      --webhook-body=<webhookBody>
Example cli to verify webhook payloads + signatures
  -h, --help      Show this help message and exit.
      --tl-signature=<tlSignature>
                  The `X-TL-Signature` webhook POST header
      --webhook-body=<webhookBody>
                  The unmodified webhook POST body
```

## Example 

This assumes you have generated a key pair using the `generate_key_pair.sh` script in the root folder of the project.

```bash
# Assumes you are running from within the `java` directory
java -jar verify/build/libs/verify.jar \
    --webhook-body "{\"event_type\":\"withdrawal_settled\",\"event_id\":\"de5940e7-d695-4dc4-b2ed-4cd2b8fa720d\",\"event_schema_version\":1,\"event_body\":{\"client_id\":\"sandbox-stefanpdeure2e-f7f4e2\",\"transaction_id\":\"d1c60b2c-c4a1-4888-9126-cc07e24528e5\",\"settled_at\":\"2021-08-03T13:13:53.910Z\"}}" \
    --tl-signature "eyJhbGciOiJSUzUxMiIsImtpZCI6ImM5MDM0YzBmLWJkMDYtNGRkMS05OGZlLWY2N2E1YTFhMDYwMSIsImprdSI6Imh0dHBzOi8vd2ViaG9va3MudHJ1ZWxheWVyLXNhbmRib3guY29tLy53ZWxsLWtub3duL2p3a3MifQ..Z1vvjNAk83eRE-KWkH9omhd6QkdeBog0YaqfhVOYKU9r8eWVYd2gsej_fcfV1jEKKL-iOt7a9qF5Htw1CJ08P7AqlWy4_-QjtNEkJ7uFEsHRNTAunROfha_Xrc7e0fNrdkyEq3WHJT5KmBXcz2xJILAleclKptAMPX_QEKoIdi6OHRM1yRKE1_0cbqlCynQdDMYIBn1PLSXN4OOUPUhQHEvc6UIRUMH0MRrW1XUesE0IWXmC-zB4Y6BD6zRu7Q0gZPlmEphS3GbzB-LQxHt45dvZXaSS45RPUBMT5PsqwKvWr0bHgF7SvAiDHusO14U2Q-9_Vgli-oqQdrG5VJgiUA"
```
