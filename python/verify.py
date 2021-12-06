import argparse
import jwt
import sys
import json
import urllib
from base64 import urlsafe_b64encode


def build_jws(args):
    """Constructs a full jwt using the body & detached jws signature args."""
    body_base64 = str(urlsafe_b64encode(
        args.webhook_body.encode("utf-8")), "utf-8").rstrip("=")
    signature_parts = args.tl_signature.split(".")
    return f"{signature_parts[0]}.{body_base64}.{signature_parts[2]}"


def fetch_public_key(jws_header):
    """Downloads the jwks indicated in the jws header"""
    # jku/jwks_url should be expected truelayer url(s)
    assert jws_header["jku"] == "https://webhooks.truelayer.com/.well-known/jwks" \
        or jws_header["jku"] == "https://webhooks.truelayer-sandbox.com/.well-known/jwks"

    # note: header hack needed to bypass cloudflare 403
    req = urllib.request.Request(jws_header["jku"], headers={
                                 'User-Agent': "jwks-client"})
    with urllib.request.urlopen(req) as response:
        res = json.load(response)
        set = jwt.PyJWKSet.from_dict(res)
        return next(k for k in set.keys if k.key_id == jws_header["kid"])


parser = argparse.ArgumentParser(
    description='CLI to verify webhook payloads + signatures.')
parser.add_argument('--webhook-body', help='the unmodified webhook POST body')
parser.add_argument(
    '--tl-signature', help='the `X-TL-Signature` webhook POST header')

jws = build_jws(parser.parse_args())
jws_header = jwt.get_unverified_header(jws)
public_key = fetch_public_key(jws_header)

try:
    # Verify jws with the public key
    jwt.decode(jws, public_key.key, algorithms=[jws_header["alg"]])
    print("Webhook verified ✓")
except Exception as e:
    print(f"Webhook verification failed: {e}")
    sys.exit(1)
