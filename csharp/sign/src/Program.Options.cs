using CommandLine;

namespace signing
{
    partial class Program
    {
        public class Options
        {
            [Option("kid", Required = true, HelpText = "The key id as shown in Console")]
            public string Kid { get; set; }

            [Option("key-file", Required = true, HelpText = "PEM encoded file containing the private key")]
            public string KeyFile { get; set; }

            [Option("payload", Required = true, HelpText = "File containing the payload to sign")]
            public string PayloadFile { get; set; }

            [Option("debug", Default = false, HelpText = "Write payload bytes to stderr")]
            public bool Debug { get; set; }
        }
    }
}
