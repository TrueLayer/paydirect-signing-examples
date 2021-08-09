package verify;

import com.auth0.jwk.UrlJwkProvider;
import com.auth0.jwt.JWT;
import com.auth0.jwt.algorithms.Algorithm;
import com.auth0.jwt.interfaces.DecodedJWT;
import picocli.CommandLine.Command;
import picocli.CommandLine.Option;

import java.io.UnsupportedEncodingException;
import java.net.URI;
import java.security.interfaces.RSAPublicKey;
import java.util.Base64;
import java.util.concurrent.Callable;

import static java.nio.charset.StandardCharsets.UTF_8;

@Command(
        name = "verify-webhook",
        description = "Example cli to verify webhook payloads + signatures",
        mixinStandardHelpOptions = true)
public class WebhookVerifier implements Callable<Integer> {


    @Option(
            names = {"--webhook-body"},
            required = true,
            description = "The unmodified webhook POST body")
    private String webhookBody;

    @Option(
            names = {"--tl-signature"},
            required = true,
            description = "The `X-TL-Signature` webhook POST header")
    private String tlSignature;

    @Override
    public Integer call() throws Exception {
        var jws = buildJws();

        var jwksUrl = jws.getHeaderClaim("jku").asString();

        // jwks_url should be expected truelayer url(s)
        if (!jwksUrl.equals("https://webhooks.truelayer.com/.well-known/jwks")
                && !jwksUrl.equals("https://webhooks.truelayer-sandbox.com/.well-known/jwks")) {
            throw new RuntimeException("Invalid signature jwksUrl: " + jwksUrl);
        }

        // fetch public key
        var jwk = new UrlJwkProvider(new URI(jwksUrl).toURL()).get(jws.getKeyId());
        var key = Algorithm.RSA512((RSAPublicKey) jwk.getPublicKey(), null);

        key.verify(jws);
        System.out.println("Webhook verified ✓");
        return 0;
    }

    // Combines body & signature into a JWS.
    private DecodedJWT buildJws() throws UnsupportedEncodingException {
        var bodyBase64 = Base64.getUrlEncoder()
                .withoutPadding()
                .encodeToString(webhookBody.getBytes(UTF_8.toString()));
        var jwsString = tlSignature.replaceFirst("\\.\\.", "." + bodyBase64 + ".");
        return JWT.decode(jwsString);
    }
}
