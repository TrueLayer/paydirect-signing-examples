# Prerequisites

- Java 11 or greater (Feel free to use [SdkMan!](https://sdkman.io/) for a painless setup)

# Instructions

Build the project with the Gradle wrapper included in this sample:
```bash
./gradlew build 
```

The run help for instructions:
```bash
java -jar build/libs/signing.jar -h
```
alternatively, with the Gradle wrapper:

```bash
./gradlew run --args='-h'
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

# Example 

This assumes you have generated a key pair using the `generate_key_pair.sh` script in the root folder of the project.

```bash
# Run with java command.
# Assumes you are running from within the `java` directory
java -jar build/libs/signing.jar --certificate-id fa07d2bb-f25e-4805-b69c-211136c84d7b --payload-filename ../payload.json --private-key-filename ../ec512-private-key.pem
```

Alternatively

```bash
# Run with the gradle wrapper.
# Assumes you are running from within the `java` directory
./gradlew run --args='--certificate-id fa07d2bb-f25e-4805-b69c-211136c84d7b --payload-filename ../payload.json --private-key-filename ../ec512-private-key.pem'
```

The script returns both a full-blown [JWS](https://tools.ietf.org/html/rfc7515) and a [JWS with detached content](https://tools.ietf.org/html/rfc7515#appendix-F) (the one to be passed in the `X-TL-Signature` header when making a request).