const Yargs = require('yargs');
const FS = require('fs');
const Path = require('path');
const JWS = require('jws');

const getArgs = () => {
    const stringOption = {
        type: 'string',
        requiresArg: true,
        demandOption: true,
    };
    const argv = Yargs.options({
        "certificate-id": { description: 'The certificate id associated to the public certificate you uploaded in TrueLayer\'s Console. The certificate id can be retrieved in the Payouts Setting section. It will be used as the `kid` header in the JWS', ...stringOption },
        "payload-filename": { description: 'The filename of the payload you want to sign, in JSON format', ...stringOption },
        "private-key-filename": { description: 'The filename of the Elliptic Curve private key used to sign, in PEM format', ...stringOption },
    }).argv;
    return {
        certificateId: argv["certificate-id"],
        payloadFilename: argv["payload-filename"],
        privateKeyFilename: argv["private-key-filename"],
    }
}

const main = () => {
    const {
        certificateId,
        payloadFilename,
        privateKeyFilename,
    } = getArgs();

    const payload = FS.readFileSync(Path.resolve(process.cwd(), payloadFilename), 'utf-8');
    const privateKey = FS.readFileSync(Path.resolve(process.cwd(), privateKeyFilename), 'utf-8');

    const jws = createJws(certificateId, payload, privateKey);
    console.log(`JWS:\n${jws}\n`);

    const jwsWithDetachedContent = detachContent(jws);
    console.log(`JWS with detached content:\n${jwsWithDetachedContent}\n`);
};


const createJws = (certificateId, payload, privateKey) => JWS.sign({
    header: {
        alg: 'ES512',
        kid: certificateId,
    },
    payload,
    privateKey,
});

const detachContent = (jws) => {
    const parts = jws.split('.');
    return `${parts[0]}..${parts[2]}`;
};

main();