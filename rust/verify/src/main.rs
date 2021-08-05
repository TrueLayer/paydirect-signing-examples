mod verify_webhook;

use clap::Clap;

/// Example command line interface to verify webhook payloads + signatures.
#[derive(Clap)]
struct Command {
    /// The unmodified webhook POST body.
    #[clap(long)]
    webhook_body: String,
    /// The `X-TL-Signature` webhook POST header.
    #[clap(long)]
    tl_signature: String,
}

pub fn main() {
    let opt = Command::parse();

    println!();

    match verify_webhook::verify_truelayer_webhook(opt.webhook_body.as_bytes(), &opt.tl_signature) {
        Ok(_) => println!("Webhook verified ✓"),
        Err(e) => {
            println!("Webhook verification failed: {}", e);
            std::process::exit(1);
        }
    }
}
