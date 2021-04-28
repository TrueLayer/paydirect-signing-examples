# Prerequisites

- jose-jwt
- commandlineparser

## Instructions

First install project dependencies:

```bash
dotnet restore
```

## Example

This assumes you have generated a key pair using the `generate_key_pair.sh` script in the root folder of the project.

```bash
# Assumes you are running from within the `csharp` directory
dotnet run --project src/signing.csproj -- --kid $kid --key-file ./ec512-private-key.pem --payload ./payload.json
```

The script returns a [JWS with detached content](https://tools.ietf.org/html/rfc7515#appendix-F) (the one to be passed in the `X-Tl-Signature` header when making a request)
