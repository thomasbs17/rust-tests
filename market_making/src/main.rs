// use serde_json::Value;
use std::error::Error;
use tokio_tungstenite::connect_async;
// use url::Url;
// use futures_util::sink::SinkExt; // For the `send` method
// use futures_util::stream::StreamExt; // For the `next` method

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
// async fn main() {
    // Define the Coinbase WebSocket URL
    // let url = String::from_str("wss://ws-feed.pro.coinbase.com").expect("Invalid URL");
    let url : &str = "wss://ws-feed.pro.coinbase.com";
    // Connect to the WebSocket
    let (mut socket, _) = connect_async(url).await?;
    println!("WebSocket connection established");

    // Subscribe to the BTC-USD order book
    let subscribe_message = r#"{
        "type": "subscribe",
        "channels": [
            {
                "name": "level2",
                "product_ids": ["BTC-USD"]
            }
        ]
    }"#;

    // // Send the subscription message
    // socket.send(tungstenite::Message::Text(subscribe_message.to_string())).await?;

    // // Listen for messages
    // while let Some(msg) = socket.next().await {
    //     match msg? {
    //         tungstenite::Message::Text(text) => {
    //             let order_book: Value = serde_json::from_str(&text)?;
    //             println!("Received: {}", order_book);
    //         }
    //         tungstenite::Message::Close(_) => {
    //             println!("Connection closed");
    //             break;
    //         }
    //         _ => {}
    //     }
    // }

    Ok(())
}
