use tokio_tungstenite::connect_async;
use futures_util::StreamExt;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct WebSocketMessage {
    pub merchant_id: Option<i32>,
    pub device_id: Option<String>,
    pub transaction_status: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct TransactionStatus {
    pub success: bool,
    pub message: String,
    pub qr_verified: Option<bool>,
    pub remarks1: Option<String>,
    pub remarks2: Option<String>,
    pub transaction_date: Option<String>,
    pub product_number: Option<String>,
    pub amount: Option<String>,
    pub commission_type: Option<String>,
    pub commission_amount: Option<f64>,
    pub total_calculated_amount: Option<f64>,
    pub payment_success: Option<bool>,
    pub trace_id: Option<i32>,
}

pub type WebSocketCallback = Arc<dyn Fn(WebSocketMessage) + Send + Sync>;

pub async fn listen_websocket_with_callback(
    ws_url: &str,
    callback: WebSocketCallback,
) -> Result<(), Box<dyn std::error::Error>> {
    let (ws_stream, _) = connect_async(ws_url).await?;
    println!("Connected to WebSocket: {}", ws_url);

    let (_, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                println!("Received WebSocket message: {}", text);
                
                // Parse the message
                if let Ok(ws_message) = serde_json::from_str::<WebSocketMessage>(&text) {
                    callback(ws_message);
                } else {
                    // Try to parse as transaction status directly
                    if serde_json::from_str::<TransactionStatus>(&text).is_ok() {
                        let ws_message = WebSocketMessage {
                            merchant_id: None,
                            device_id: None,
                            transaction_status: text,
                        };
                        callback(ws_message);
                    }
                }
            }
            Ok(tokio_tungstenite::tungstenite::Message::Binary(_)) => {
                println!("Received binary message");
            }
            Ok(tokio_tungstenite::tungstenite::Message::Ping(_)) => {
                println!("Received ping");
            }
            Ok(tokio_tungstenite::tungstenite::Message::Pong(_)) => {
                println!("Received pong");
            }
            Ok(tokio_tungstenite::tungstenite::Message::Close(_)) => {
                println!("WebSocket connection closed");
                break;
            }
            Ok(tokio_tungstenite::tungstenite::Message::Frame(_)) => {
                println!("Received frame message");
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

pub async fn listen_websocket(ws_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let callback = Arc::new(|message: WebSocketMessage| {
        println!("WebSocket Callback:");
        println!("  Merchant ID: {:?}", message.merchant_id);
        println!("  Device ID: {:?}", message.device_id);
        println!("  Transaction Status: {}", message.transaction_status);
        
        // Try to parse the transaction status
        if let Ok(status) = serde_json::from_str::<TransactionStatus>(&message.transaction_status) {
            println!("  Parsed Transaction Status:");
            println!("    Success: {}", status.success);
            println!("    Message: {}", status.message);
            if let Some(verified) = status.qr_verified {
                println!("    QR Verified: {}", verified);
            }
            if let Some(payment_success) = status.payment_success {
                println!("    Payment Success: {}", payment_success);
            }
            if let Some(amount) = status.amount {
                println!("    Amount: {}", amount);
            }
            if let Some(product_number) = status.product_number {
                println!("    Product Number: {}", product_number);
            }
            if let Some(transaction_date) = status.transaction_date {
                println!("    Transaction Date: {}", transaction_date);
            }
        }
    });

    listen_websocket_with_callback(ws_url, callback).await
}

