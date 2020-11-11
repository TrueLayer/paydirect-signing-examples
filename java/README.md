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
Options:
  --help|-h               Show help                                    [boolean]
  --version|-V            Show version number                          [boolean]
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