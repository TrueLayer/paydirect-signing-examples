using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Security.Cryptography;
using System.Text;
using CommandLine;
using Jose;

namespace signing
{
    partial class Program
    {
        static void Main(string[] args)
        {
            Parser.Default.ParseArguments<Options>(args)
                .WithParsed(options =>
                {
                    var token = Sign(options.Kid, options.KeyFile, options.PayloadFile, options.Debug);
                    Console.Error.WriteLine("\nDetached payload signature");
                    Console.Write(token);
                });
        }

        static string Sign(string kid, string keyFile, string payloadFile, bool debug)
        {
            var file = File.ReadAllText(keyFile).Trim();
            var key = ECDsa.Create();
            key?.ImportFromPem(file);

            // Dangling control chars (like newline) are a pernicious cause of signature validation errors.
            // Ensure there are none in the source file.
            var bodyText = File.ReadAllText(payloadFile, Encoding.UTF8);

            if (debug)
            {
                var bitString = BitConverter.ToString(Encoding.UTF8.GetBytes(bodyText));
                Console.Error.WriteLine($"Payload bits\n{bitString}");
            }

            var headers = new Dictionary<string, object> { {"alg", "ES512"}, {"kid", kid} };

            return JWT.Encode(bodyText, key, JwsAlgorithm.ES512, headers, options: new JwtOptions{ DetachPayload = true });
        }
    }
}
