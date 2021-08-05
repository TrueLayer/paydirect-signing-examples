import Yargs, { Options } from 'yargs';
import * as Jws from 'jws';
import * as Base64 from 'js-base64';
import JwksClient from 'jwks-rsa';

interface Args {
    webhookBody: string;
    tlSignature: string;
}

const getArgs = (): Args => {
    const requiredString: Options = {
        type: 'string',
        requiresArg: true,
        demandOption: true,
    };
    const argv = Yargs.options({
        "webhook-body": { description: 'The unmodified webhook POST body', ...requiredString },
        "tl-signature": { description: 'The `X-TL-Signature` webhook POST header', ...requiredString },
    }).argv;

    return {
        webhookBody: argv["webhook-body"] as string,
        tlSignature: argv["tl-signature"] as string,
    };
}

const main = async (): Promise<void> => {
    const { webhookBody, tlSignature } = getArgs();

    try {
        await verifyTruelayerWebhook(webhookBody, tlSignature);
        console.log("Webhook verified ✓");
    } catch (e) {
        console.error("Webhook verification failed:", e.message);
    }
};

// Verifies a truelayer webhook `body` & `tlSignature`.
//
// Throws an exception for invalid body + signatures.
const verifyTruelayerWebhook = async (body: string, tlSignature: string): Promise<void> => {
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
const fetchJwksPublicKey = async (jwksUri: string, kid: string): Promise<string> => {
    // Note: jku/jwks url should be expected truelayer url(s)
    if (jwksUri !== "https://webhooks.truelayer.com/.well-known/jwks"
        && jwksUri !== "https://webhooks.truelayer-sandbox.com/.well-known/jwks") {
        throw new Error(`Invalid jku (jwks url): ${jwksUri}`);
    }

    const key = await JwksClient({ jwksUri }).getSigningKey(kid);
    return key.getPublicKey();
};

main();
