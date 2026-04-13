# Dynamic QR - Rust

Version: 1.1
Date: July 2024

## Use This First

1. Call `post_qr_request` to create QR.
2. Call `post_qr_request_status` with same PRN to check payment status.
3. Call `get_qr_response` with websocket URL from QR response if you want real-time updates.

## Minimal Backend Example

```rust
use fonepay_dynamic_qr::{post_qr_request, post_qr_request_status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = std::env::var("FONEPAY_BASE_URL")?;
    let prn = "unique-prn-001";

    let qr_response = post_qr_request(
        &base_url,
        "50",
        prn,
        "fonepay123",
        "test1",
        "test2",
        "bijayk",
        "password",
        "fonepay",
    ).await?;

    println!("QR Response: {}", qr_response);

    let status_response = post_qr_request_status(
        &base_url,
        prn,
        "fonepay123",
        "bijayk",
        "password",
        "fonepay",
    ).await?;

    println!("Payment Status: {}", status_response.payment_status);
    Ok(())
}
```

## Function List

- `post_qr_request`: create QR and send request
- `post_qr_request_typed`: same as above, returns typed response
- `post_qr_request_status`: check QR status
- `get_qr_request_`: same as status check
- `get_qr_response`: open websocket listener
- `post_hmac_cred`: generate HMAC hash from key and message
- `get_hmac_cred`: get test credentials

## Test Credentials

```text
Merchant Code: fonepay123
Secret Key: fonepay
Username: bijayk
Password: password
```

## HMAC Format

QR request message format:

```text
AMOUNT,PRN,MERCHANT-CODE,REMARKS1,REMARKS2
```

Status check message format:

```text
PRN,MERCHANT-CODE
```

## Important Notes

1. Use unique PRN for every request.
2. Generate HMAC securely in backend only.
3. After websocket message, confirm with status API.
4. Do not URL-encode values used in HMAC input.