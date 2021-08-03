use anyhow::{ensure, Context};
use picky::{
    jose::{
        jwk::JwkSet,
        jws::{Jws, JwsHeader},
    },
    key::PublicKey,
};

/// Verifies a truelayer webhook `body` & `tl_signature` (`X-TL-Signature` header value).
pub fn verify_truelayer_webhook(body: &[u8], tl_signature: &str) -> anyhow::Result<()> {
    // Download the public key as indicated in the signature header
    let jws_header = Jws::decode_without_validation(tl_signature)?.header;
    let public_key = fetch_jwks_public_key(jws_header)?;

    // Construct a full jwt using the `body` & detached jws
    let jws = {
        let mut jwt_parts = tl_signature.split('.');
        let header = jwt_parts.next().unwrap();
        let signature = jwt_parts.last().unwrap();
        let body_base64 = base64::encode_config(body, base64::URL_SAFE_NO_PAD);

        format!("{}.{}.{}", header, body_base64, signature)
    };

    // Verify jws with the public key
    Jws::decode(&jws, &public_key)?;

    Ok(())
}

/// Using jws header info download the /jwks public key by `kid` lookup.
fn fetch_jwks_public_key(jws_header: JwsHeader) -> anyhow::Result<PublicKey> {
    // the jws header contains the /jwks url & a `kid` where we can find the public key.
    let jwks_url = jws_header.jku.context("missing jws_header.jku")?;
    let kid = jws_header.kid.context("missing jws_header.kid")?;

    // jwks_url should be expected truelayer url(s)
    // Note: There are other valid urls not stated here in use by other systems.
    ensure!(
        jwks_url == "https://webhooks.truelayer.com/.well-known/jwks"
            || jwks_url == "https://webhooks.truelayer-sandbox.com/.well-known/jwks",
        "invalid jwks_url: {}",
        jwks_url,
    );

    // download the jwks
    // Note: blocking for simplicity, this fetch is generally cachable, see `cache-control`.
    let jwks: JwkSet = reqwest::blocking::get(jwks_url)?.json()?;

    // pick out the kid public key
    Ok(jwks
        .keys
        .iter()
        .find(|k| k.kid.as_ref() == Some(&kid))
        .context("no key with kid found")?
        .to_public_key()?)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Example of a webhook produced in truelayer-sandbox environment.
    const EXAMPLE_BODY: &str = "{\"event_type\":\"withdrawal_settled\",\"event_id\":\"de5940e7-d695-4dc4-b2ed-4cd2b8fa720d\",\"event_schema_version\":1,\"event_body\":{\"client_id\":\"sandbox-stefanpdeure2e-f7f4e2\",\"transaction_id\":\"d1c60b2c-c4a1-4888-9126-cc07e24528e5\",\"settled_at\":\"2021-08-03T13:13:53.910Z\"}}";
    /// Signature for `EXAMPLE_BODY` webhook body.
    const EXAMPLE_SIGNATURE: &str = "eyJhbGciOiJSUzUxMiIsImtpZCI6ImM5MDM0YzBmLWJkMDYtNGRkMS05OGZlLWY2N2E1YTFhMDYwMSIsImprdSI6Imh0dHBzOi8vd2ViaG9va3MudHJ1ZWxheWVyLXNhbmRib3guY29tLy53ZWxsLWtub3duL2p3a3MifQ..Z1vvjNAk83eRE-KWkH9omhd6QkdeBog0YaqfhVOYKU9r8eWVYd2gsej_fcfV1jEKKL-iOt7a9qF5Htw1CJ08P7AqlWy4_-QjtNEkJ7uFEsHRNTAunROfha_Xrc7e0fNrdkyEq3WHJT5KmBXcz2xJILAleclKptAMPX_QEKoIdi6OHRM1yRKE1_0cbqlCynQdDMYIBn1PLSXN4OOUPUhQHEvc6UIRUMH0MRrW1XUesE0IWXmC-zB4Y6BD6zRu7Q0gZPlmEphS3GbzB-LQxHt45dvZXaSS45RPUBMT5PsqwKvWr0bHgF7SvAiDHusO14U2Q-9_Vgli-oqQdrG5VJgiUA";

    #[test]
    fn verify_example() {
        let verify = verify_truelayer_webhook(EXAMPLE_BODY.as_bytes(), EXAMPLE_SIGNATURE);
        assert!(verify.is_ok(), "{:?}", verify);
    }

    #[test]
    fn verify_example_bad_signature() {
        const INCORRECT_SIGNATURE: &str = "eyJhbGciOiJSUzUxMiIsImtpZCI6ImM5MDM0YzBmLWJkMDYtNGRkMS05OGZlLWY2N2E1YTFhMDYwMSIsImprdSI6Imh0dHBzOi8vd2ViaG9va3MudHJ1ZWxheWVyLXNhbmRib3guY29tLy53ZWxsLWtub3duL2p3a3MifQ..GHvVb63MaxGB1NvXgJeL_NdymUj249TueFrKRh0kjOX6sAaskXMnmmFoziyZykInsSs7mKfRm02HkuVxQxBhDxuDbgfC2o4JkUsDiL5McreKb6ute8eGWsaaP0-LSkn0GxHUUQ7FfVY8G4Z--3uwoDuCq33ezbm7upWCOJeaAmRm3ry2Gbk-MQxzZOa1gutf7q975eZPkPoyAVRp9FiPNdhvrHwK3eI-U52v_V8qWOvm7MC0ApMPEgGf9-9zI9RAR9Vgd9MK23lbzWWevApzUQQWvFhpNR5Z5fkFJB-6zsyJMfv5dGFYf3aR5jRtMUBxvsK2LsdzvzfpVpBbLHFkGg";

        let verify_ex_bad = verify_truelayer_webhook(EXAMPLE_BODY.as_bytes(), INCORRECT_SIGNATURE);
        assert!(
            verify_ex_bad.is_err(),
            "bad body + example signature incorrectly passed verification"
        );

        let verify_bad_ex =
            verify_truelayer_webhook(r#"{"foo":123}"#.as_bytes(), EXAMPLE_SIGNATURE);
        assert!(
            verify_bad_ex.is_err(),
            "bad body + example signature incorrectly passed verification"
        );
    }
}
