package signing;

import com.nimbusds.jose.*;
import com.nimbusds.jose.crypto.ECDSASigner;
import org.bouncycastle.jce.provider.BouncyCastleProvider;
import org.bouncycastle.openssl.PEMKeyPair;
import org.bouncycastle.openssl.PEMParser;
import org.bouncycastle.openssl.jcajce.JcaPEMKeyConverter;
import picocli.CommandLine;
import picocli.CommandLine.Command;
import picocli.CommandLine.Option;

import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.KeyPair;
import java.security.Security;
import java.security.interfaces.ECPrivateKey;
import java.util.concurrent.Callable;

@Command(
        name = "signing",
        description = "A small command line interface to sign POST requests for Payouts API",
        mixinStandardHelpOptions = true,
        version = "signing 1.0")
public class RequestSigner implements Callable<Integer> {

    @Option(
            names = {"--certificate-id"},
            required = true,
            description = "The certificate id associated to the public certificate you uploaded in TrueLayer's Console. The certificate id can be retrieved in the Payouts Setting section. It will be used as the `kid` header in the JWS")
    private String certificateId;

    @Option(
            names = {"--payload-filename"},
            required = true,
            description = "The filename of the payload you want to sign, in JSON format"
    )
    private String payloadFileName;

    @Option(
            names = {"--private-key-filename"},
            required = true,
            description = "The filename of the Elliptic Curve private key used to sign, in PEM format"
    )
    private String privateKeyFilename;

    @Override
    public Integer call() throws IOException, JOSEException {
        String payload = Files.readString(Path.of(payloadFileName).toAbsolutePath());
        String signature = createJwsSignature(payload);
        var signatureParts = signature.split("\\.");
        String detachedSignature = String.format("%s..%s", signatureParts[0], signatureParts[2]);

        System.out.printf("Request payload:\n%s" +
                "\n\nJWS:\n%s\n\nJWS with detached content:\n%s" +
                "\n\n", payload, signature, detachedSignature);

        return 0;
    }

    private String createJwsSignature(String payload) throws IOException, JOSEException {
        Security.addProvider(new BouncyCastleProvider());

        PEMParser pemParser = new PEMParser(new InputStreamReader(new FileInputStream(Path.of(privateKeyFilename).toAbsolutePath().toString())));
        PEMKeyPair pemKeyPair = (PEMKeyPair) pemParser.readObject();

        // Convert to Java (JCA) format
        JcaPEMKeyConverter converter = new JcaPEMKeyConverter();
        KeyPair keyPair = converter.getKeyPair(pemKeyPair);
        pemParser.close();

        // Get private EC key
        ECPrivateKey privateKey = (ECPrivateKey) keyPair.getPrivate();

        JWSObject jwsObject = new JWSObject(
                new JWSHeader.Builder(JWSAlgorithm.ES512).keyID(certificateId).build(),
                new Payload(payload));
        jwsObject.sign(new ECDSASigner(privateKey));

        // Serialise
        String compactJWS = jwsObject.serialize();

        return compactJWS;
    }
}
