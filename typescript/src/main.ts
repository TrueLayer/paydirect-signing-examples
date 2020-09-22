import Yargs, { Options } from 'yargs';
import * as FS from 'fs';
import * as Path from 'path';
import * as JWS from 'jws';

interface SigningArgs {
    certificateId: string;
    payloadFilename: string;
    privateKeyFilename: string;
}

const getArgs = (): SigningArgs => {
    const stringOption: Options = {
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
        certificateId: argv["certificate-id"] as string,
        payloadFilename: argv["payload-filename"] as string,
        privateKeyFilename: argv["private-key-filename"] as string,
    }
}

const main = (): void => {
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


const createJws = (certificateId: string, payload: string, privateKey: string): string => JWS.sign({
    header: {
        alg: 'ES512',
        kid: certificateId,
    },
    payload,
    privateKey,
});

const detachContent = (jws: string): string => {
    const parts = jws.split('.');
    return `${parts[0]}..${parts[2]}`;
};

main();