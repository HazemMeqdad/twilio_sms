use std::env;
use twilio::{Client, OutboundMessage, OutboundMessageParams};

#[tokio::main]
async fn main() {
    // Get Twilio credentials from environment variables
    let account_sid = env::var("TWILIO_ACCOUNT_SID").expect("TWILIO_ACCOUNT_SID not set");
    let auth_token = env::var("TWILIO_AUTH_TOKEN").expect("TWILIO_AUTH_TOKEN not set");
    let twilio_number = env::var("TWILIO_NUMBER").expect("TWILIO_NUMBER not set");

    // Initialize the Twilio client
    let client = Client::new(&account_sid, &auth_token);

    // Set up a loop to continuously check for incoming messages
    loop {
        // Fetch messages
        let messages = client.get_messages().await.unwrap();

        // Process each message
        for message in messages {
            println!("Received message: {:?}", message);
            // You can implement your logic to handle the incoming message here
        }

        // Delay before checking for messages again
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
