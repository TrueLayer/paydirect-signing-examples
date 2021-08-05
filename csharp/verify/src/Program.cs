using System;
using System.Linq;
using System.Net.Http;
using System.Security.Cryptography;
using System.Text;
using System.Threading.Tasks;
using CommandLine;
using Jose;
using Newtonsoft.Json.Linq;

namespace verify
{
    partial class Program
    {
        static void Main(string[] args)
        {
            Parser.Default.ParseArguments<Options>(args)
                .WithParsed(options =>
                {
                    VerifyTruelayerWebhook(options.WebhookBody, options.TlSignature).Wait();
                    Console.WriteLine("Webhook verified ✓");
                });
        }

        /// <summary>Verifies a truelayer webhook body + signature or throws.</summary>
        static async Task VerifyTruelayerWebhook(string body, string tlSignature)
        {
            var bodyBase64 = Base64Url.Encode(Encoding.UTF8.GetBytes(body));
            var jws = tlSignature.Replace("..", $".{bodyBase64}.");

            var headers = Jose.JWT.Headers(jws);

            var key = await FetchJwksPublicKey(headers["jku"].ToString(), headers["kid"].ToString());

            var payload = Jose.JWT.Decode(jws, key);
        }

        /// <summary>
        /// Using the jws header info download the /jwks public key by kid lookup.
        /// </summary>
        static async Task<RSACryptoServiceProvider> FetchJwksPublicKey(string jku, string kid)
        {
            // Note: jku/jwks url should be expected truelayer url(s)
            if (jku != "https://webhooks.truelayer.com/.well-known/jwks"
                && jku != "https://webhooks.truelayer-sandbox.com/.well-known/jwks")
            {
                throw new ArgumentException($"invalid signature jku (jwks url): {jku}");
            }

            JObject jwks;
            using (HttpClient http = new HttpClient())
            {
                var jwksString = await http.GetStringAsync(jku);
                jwks = JObject.Parse(jwksString);
            }

            var jwk = jwks["keys"].SingleOrDefault(k => (k["kid"].ToString()) == kid);
            if (jwk == null) throw new ArgumentException($"Unknown signature kid: {kid}");

            var key = new RSACryptoServiceProvider();
            key.ImportParameters(new RSAParameters
            {
                Modulus = Base64Url.Decode(jwk["n"].ToString()),
                Exponent = Base64Url.Decode(jwk["e"].ToString())
            });

            return key;
        }
    }
}
