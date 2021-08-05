# Instructions
```bash
cargo run -q -- --help
```

```text
verify-webhooks
Example command line interface to verify webhook payloads + signatures

USAGE:
    verify-webhooks --webhook-body <webhook-body> --tl-signature <tl-signature>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --tl-signature <tl-signature>    The `X-TL-Signature` webhook POST header
        --webhook-body <webhook-body>    The unmodified webhook POST body
```

# Example 
```sh
# Assumes you are running from within the `rust/verify` directory
cargo run -- \
    --webhook-body "{\"event_type\":\"withdrawal_settled\",\"event_id\":\"de5940e7-d695-4dc4-b2ed-4cd2b8fa720d\",\"event_schema_version\":1,\"event_body\":{\"client_id\":\"sandbox-stefanpdeure2e-f7f4e2\",\"transaction_id\":\"d1c60b2c-c4a1-4888-9126-cc07e24528e5\",\"settled_at\":\"2021-08-03T13:13:53.910Z\"}}" \
    --tl-signature "eyJhbGciOiJSUzUxMiIsImtpZCI6ImM5MDM0YzBmLWJkMDYtNGRkMS05OGZlLWY2N2E1YTFhMDYwMSIsImprdSI6Imh0dHBzOi8vd2ViaG9va3MudHJ1ZWxheWVyLXNhbmRib3guY29tLy53ZWxsLWtub3duL2p3a3MifQ..Z1vvjNAk83eRE-KWkH9omhd6QkdeBog0YaqfhVOYKU9r8eWVYd2gsej_fcfV1jEKKL-iOt7a9qF5Htw1CJ08P7AqlWy4_-QjtNEkJ7uFEsHRNTAunROfha_Xrc7e0fNrdkyEq3WHJT5KmBXcz2xJILAleclKptAMPX_QEKoIdi6OHRM1yRKE1_0cbqlCynQdDMYIBn1PLSXN4OOUPUhQHEvc6UIRUMH0MRrW1XUesE0IWXmC-zB4Y6BD6zRu7Q0gZPlmEphS3GbzB-LQxHt45dvZXaSS45RPUBMT5PsqwKvWr0bHgF7SvAiDHusO14U2Q-9_Vgli-oqQdrG5VJgiUA"
```
