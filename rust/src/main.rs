//! Cryptographic helpers functions (signing and signature verification).
use anyhow::Context;
use base64::URL_SAFE_NO_PAD;
use clap::Clap;
use openssl::ec::EcKey;
use openssl::ecdsa::EcdsaSig;
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::Private;
use serde_json::{json, Value};
use std::path::PathBuf;
use uuid::Uuid;

/// A small command line interface to sign POST requests for Payouts API.
#[derive(Clap)]
#[clap(version = "1.0", author = "TrueLayer")]
struct Command {
    /// The filename of the payload you want to sign, in JSON format.
    #[clap(long)]
    payload_filename: PathBuf,
    /// The filename of the Elliptic Curve private key used to sign, in PEM format.
    #[clap(long)]
    private_key_filename: PathBuf,
    /// The certificate id associated to the public certificate you uploaded in TrueLayer's Console.
    /// The certificate id can be retrieved in the Payouts Setting section.
    /// It will be used as the `kid` header in the JWS.
    #[clap(long)]
    certificate_id: Uuid,
}

impl Command {
    /// Parse the JSON payload from the specified file.
    pub fn payload(&self) -> Result<Value, anyhow::Error> {
        let raw_payload = std::fs::read(&self.payload_filename)
            .context("Failed to read the request payload file.")?;
        let payload: Value = serde_json::from_slice(&raw_payload)
            .context("Failed to parse the request payload as JSON.")?;
        Ok(payload)
    }

    /// Parse the EC private key from the specified file.
    pub fn private_key(&self) -> Result<EcKey<Private>, anyhow::Error> {
        let raw_private_key = std::fs::read(&self.private_key_filename)
            .context("Failed to read the private key file.")?;
        let private_key = openssl::pkey::PKey::private_key_from_pem(&raw_private_key)
            .context("Failed to parse the private key as PEM.")?
            .ec_key()
            .context("The private key must be an Elliptic Curve key.")?;
        private_key.check_key().context("Key verification failed")?;
        Ok(private_key)
    }
}

#[derive(serde::Serialize)]
pub struct JwsPayload {
    #[serde(rename = "Content-Type")]
    content_type: String,
    body: Value,
}

pub fn main() -> Result<(), anyhow::Error> {
    let options = Command::parse();

    let jws_header = json!({
        "alg": "ES512",
        "kid": options.certificate_id.to_string()
    });
    let jws_payload = options.payload()?;
    let jws_payload = serde_json::to_string(&jws_payload)?;
    let private_key = options.private_key()?;

    let jws = get_jws(&jws_header, &jws_payload, private_key)?;
    println!("JWS:\n{}\n", jws);

    let parts = jws.split(".").collect::<Vec<_>>();
    // Omit the payload for a JWS with detached payload
    println!("JWS with detached content:\n{}..{}", parts[0], parts[2]);
    Ok(())
}

/// Get a JWS using the ES512 signing scheme.
///
/// Check section A.4 of RFC7515 for the details: https://www.rfc-editor.org/rfc/rfc7515.txt
pub fn get_jws(
    jws_header: &Value,
    jws_payload: &str,
    pkey: EcKey<Private>,
) -> Result<String, anyhow::Error> {
    let to_be_signed = format!(
        "{}.{}",
        base64_encode(serde_json::to_string(&jws_header)?.as_bytes()),
        base64_encode(jws_payload.as_bytes()),
    );
    let signature = sign_es512(to_be_signed.as_bytes(), pkey)?;

    let jws = format!(
        "{}.{}.{}",
        base64_encode(serde_json::to_string(&jws_header)?.as_bytes()),
        base64_encode(jws_payload.as_bytes()),
        signature
    );
    Ok(jws)
}

/// Sign a payload using the provided private key and return the signature as a base64 encoded string.
///
/// Check section A.4 of RFC7515 for the details: https://www.rfc-editor.org/rfc/rfc7515.txt
pub fn sign_es512(payload: &[u8], pkey: EcKey<Private>) -> Result<String, anyhow::Error> {
    if pkey.group().curve_name() != Some(Nid::SECP521R1) {
        return Err(anyhow::anyhow!(
            "The underlying elliptic curve must be P-521 to sign using ES512."
        ));
    }
    let hash = openssl::hash::hash(MessageDigest::sha512(), &payload)?;
    let structured_signature = EcdsaSig::sign(&hash, &pkey)?;

    let r = structured_signature.r().to_vec();
    let s = structured_signature.s().to_vec();
    let mut signature_bytes: Vec<u8> = Vec::new();
    // Padding to fixed length
    signature_bytes.extend(std::iter::repeat(0x00).take(66 - r.len()));
    signature_bytes.extend(r);
    // Padding to fixed length
    signature_bytes.extend(std::iter::repeat(0x00).take(66 - s.len()));
    signature_bytes.extend(s);

    Ok(base64_encode(&signature_bytes))
}

/// Base64 encoding according to RFC7515 - see `Base64url` in section 2.
pub fn base64_encode(payload: &[u8]) -> String {
    base64::encode_config(payload, URL_SAFE_NO_PAD)
}
