using CommandLine;

namespace verify
{
    partial class Program
    {
        public class Options
        {
            [Option("webhook-body", Required = true, HelpText = "The unmodified webhook POST body")]
            public string WebhookBody { get; set; }

            [Option("tl-signature", Required = true, HelpText = "The `X-TL-Signature` webhook POST header")]
            public string TlSignature { get; set; }
        }
    }
}
