use std::time::Duration;

use dotenv;
use tokio::task;
use tokio::time::{self};

static KEY_MAKE_REQUEST_EVERY_X_SECONDS: &str = "MAKE_REQUEST_EVERY_X_SECONDS";
static KEY_URI: &str = "URI";

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let task = task::spawn(async {
        let uri = std::env::var(KEY_URI).unwrap();
        loop {
            println!("making ping request...");
            let client = reqwest::Client::new();
            let result = client
                .post(&uri)
                // .body("the exact body that is sent")
                .send()
                .await;

            if result.is_err() {
                println!("error in making request.")
            }
            time::sleep(Duration::from_secs(
                std::env::var(KEY_MAKE_REQUEST_EVERY_X_SECONDS)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
            ))
            .await;
        }
    });

    let result = task.await;
    if result.is_err() {
        println!("ERROR")
    }
}
