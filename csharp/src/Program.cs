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
                    var token = Sign(options.Kid, options.KeyFile, options.PayloadFile);
                    Console.Write(token);
                });
        }

        static string Sign(string kid, string keyFile, string payloadFile)
        {
            // THe base64 encoded content of the ec private key (--BEGIN..--/--END..-- removed)
            var file = File.ReadLines(keyFile);
            var eccPem = file.Aggregate("", (l1, l2) =>
            {
                if (l2.Contains("--")) return l1;
                return l1 + l2;
            });

            var key = ECDsa.Create();
            key?.ImportECPrivateKey(Convert.FromBase64String(eccPem), out _);

            // Dangling control chars (like newline) are a pernicious cause of signature validation errors.
            // Ensure there are none in the source file.
            var bodyText = File.ReadAllText(payloadFile, Encoding.UTF8);

            // Uncomment to help diagnose dangling chars like carriage return '0A'
            // var bitString = BitConverter.ToString(Encoding.UTF8.GetBytes(bodyText));
            // Console.WriteLine($"Content bits {bitString}");

            var headers = new Dictionary<string, object> { {"alg", "ES512"}, {"kid", kid} };

            return JWT.Encode(bodyText, key, JwsAlgorithm.ES512, headers, options: new JwtOptions{ DetachPayload = true });
        }
    }
}
