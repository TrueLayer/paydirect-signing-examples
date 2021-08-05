const Yargs = require('yargs');
const Jws = require('jws');
const { Base64 } = require('js-base64');
const JwksClient = require('jwks-rsa');

const getArgs = () => {
    const requiredString = {
        type: 'string',
        requiresArg: true,
        demandOption: true,
    };
    return Yargs.options({
        "webhook-body": { description: 'The unmodified webhook POST body', ...requiredString },
        "tl-signature": { description: 'The `X-TL-Signature` webhook POST header', ...requiredString },
    }).argv;
}

const main = async () => {
    const {
        "webhook-body": webhookBody,
        "tl-signature": tlSignature,
    } = getArgs();

    try {
        await verifyTruelayerWebhook(webhookBody, tlSignature);
        console.log("Webhook verified ✓");
    } catch (e) {
        console.error("Webhook verification failed:", e.message);
        process.exitCode = 1
    }
};

// Verifies a truelayer webhook `body` & `tlSignature`.
//
// Throws an exception for invalid body + signatures.
const verifyTruelayerWebhook = async (body, tlSignature) => {
    // Construct a full jws using the `body` & detached jws
    const signatureParts = tlSignature.split('.');
    const jws = `${signatureParts[0]}.${Base64.encode(body, true)}.${signatureParts[2]}`;

    const { header } = Jws.decode(jws);
    const publicKey = await fetchJwksPublicKey(header.jku, header.kid);

    // Verify using the public key
    if (!Jws.verify(jws, header.alg, publicKey)) {
        throw new Error("Invalid signature");
    }
};

// Using the jws header info download the /jwks public key by `kid` lookup.
const fetchJwksPublicKey = async (jwksUri, kid) => {
    // Note: jku/jwks url should be expected truelayer url(s)
    if (jwksUri !== "https://webhooks.truelayer.com/.well-known/jwks"
        && jwksUri !== "https://webhooks.truelayer-sandbox.com/.well-known/jwks") {
        throw new Error(`Invalid jku (jwks url): ${jwksUri}`);
    }

    const key = await JwksClient({ jwksUri }).getSigningKey(kid);
    return key.getPublicKey();
};

main();
