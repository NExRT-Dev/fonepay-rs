# Dynamic QR - Rust

Version: 1.1
Date: July 2024

## Endpoint Summary

- post_qr_request: QR creation endpoint with request/response examples.
- post_qr_request_status: QR status check endpoint with request/response examples.
- get_qr_response: WebSocket verification and payment success/failure messages.
- get_qr_request_: alias for status check (same as post_qr_request_status).
- post_hmac_cred: HMAC signature generation format and sample code.
- get_hmac_cred: testing credentials (merchantCode, secretKey, username, password).

## 1) post_qr_request - Create QR

Endpoint:

POST /merchant/merchantDetailsForThirdParty/thirdPartyDynamicQrDownload

Request:

```json
{
  "amount": "50",
  "prn": "unique-prn-001",
  "merchantCode": "fonepay123",
  "remarks1": "test1",
  "remarks2": "test2",
  "dataValidation": "HMAC_SHA512_HASH",
  "username": "bijayk",
  "password": "password"
}
```

Response (success):

```json
{
  "message": "successfull",
  "qrMessage": "000201010212...",
  "status": "CREATED",
  "statusCode": 201,
  "success": true,
  "thirdpartyQrWebSocketUrl": "wss://dev-ws.fonepay.com/convergent-webSocket-web/merchantEndPoint/..."
}
```

## 2) post_qr_request_status - Check QR Status

Endpoint:

POST /merchant/merchantDetailsForThirdParty/thirdPartyDynamicQrGetStatus

Request:

```json
{
  "prn": "unique-prn-001",
  "merchantCode": "fonepay123",
  "dataValidation": "HMAC_SHA512_HASH",
  "username": "bijayk",
  "password": "password"
}
```

Response examples:

- Success: paymentStatus = success
- Failed: paymentStatus = failed
- Pending: paymentStatus = pending

## 3) get_qr_response - WebSocket Messages

Connect using thirdpartyQrWebSocketUrl from QR creation response.

Verification response:

```json
{
  "merchantId": 70,
  "deviceId": "Td35588c2d9a647f28f4959f96f905bec",
  "transactionStatus": "{\"success\":true,\"message\":\"VERIFIED\",\"QRVerified\":true}"
}
```

Payment success response:

```json
{
  "merchantId": 70,
  "deviceId": "Td35588c2d9a647f28f4959f96f905bec",
  "transactionStatus": "{\"traceId\":17015,\"amount\":\"50\",\"message\":\"Request Complete\",\"paymentSuccess\":true}"
}
```

## 4) get_qr_request_ - Alias for Status Check

Same as post_qr_request_status.

## 5) post_hmac_cred - Generate HMAC

Message format (QR request):

```text
AMOUNT,PRN,MERCHANT-CODE,REMARKS1,REMARKS2
```

Message format (status check):

```text
PRN,MERCHANT-CODE
```

Example:

- Key: fonepay
- Message: 50,unique-prn-001,fonepay123,test1,test2
- Result: f036eb09c5402a91e1926e904a423e66d041add18423959dad512b6f5fa37f1ea...

## 6) get_hmac_cred - Testing Credentials

```text
Merchant Code: fonepay123
Secret Key: fonepay
Username: bijayk
Password: password
```

## Important Notes

1. Unique PRN for each request.
2. Generate HMAC securely in backend.
3. WebSocket updates should always be confirmed using status API.
4. Do not URL-encode values used for HMAC input string.

## Crate Quick Mapping

If you are using this Rust crate directly, use these exact function names:

- post_qr_request
- post_qr_request_status
- get_qr_request_
- get_qr_response
- post_hmac_cred
- get_hmac_cred