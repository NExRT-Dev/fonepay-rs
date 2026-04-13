//! Fonepay Dynamic QR Library
//! 
//! A Rust library for generating Fonepay Dynamic QR codes with proper HMAC validation
//! and API compliance.

pub mod hmac_utils;
pub mod qr_request;
pub mod status;
pub mod websocket;

pub use hmac_utils::generate_hmac;
pub use qr_request::{QrRequest, build_qr_request, send_qr_request};
pub use status::{StatusRequest, StatusResponse, check_status, build_status_request, download_qr};
pub use websocket::{
    listen_websocket, listen_websocket_with_callback, 
    WebSocketMessage, TransactionStatus, WebSocketCallback,
    parse_transaction_status, is_payment_success, is_qr_verified
};
