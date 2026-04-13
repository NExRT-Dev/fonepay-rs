mod hmac_utils;
mod qr_request;
mod status;
mod websocket;

pub use status::{QrResponse, StatusResponse};

#[derive(Clone, Copy, Debug)]
pub struct TestCredentials {
    pub merchant_code: &'static str,
    pub secret_key: &'static str,
    pub username: &'static str,
    pub password: &'static str,
}

pub const TEST_MERCHANT_CODE: &str = "fonepay123";
pub const TEST_SECRET_KEY: &str = "fonepay";
pub const TEST_USERNAME: &str = "bijayk";
pub const TEST_PASSWORD: &str = "password";

pub fn get_hmac_cred() -> TestCredentials {
    TestCredentials {
        merchant_code: TEST_MERCHANT_CODE,
        secret_key: TEST_SECRET_KEY,
        username: TEST_USERNAME,
        password: TEST_PASSWORD,
    }
}

pub fn post_hmac_cred(secret_key: &str, message: &str) -> String {
    hmac_utils::generate_hmac(secret_key, message)
}

pub async fn post_qr_request(
    base_url: &str,
    amount: &str,
    prn: &str,
    merchant_code: &str,
    remarks1: &str,
    remarks2: &str,
    username: &str,
    password: &str,
    secret_key: &str,
) -> Result<String, reqwest::Error> {
    let req = qr_request::create_qr_request(
        amount,
        prn,
        merchant_code,
        remarks1,
        remarks2,
        username,
        password,
        secret_key,
    );

    qr_request::send_qr_request(base_url, &req).await
}

pub async fn post_qr_request_typed(
    base_url: &str,
    amount: &str,
    prn: &str,
    merchant_code: &str,
    remarks1: &str,
    remarks2: &str,
    username: &str,
    password: &str,
    secret_key: &str,
) -> Result<QrResponse, reqwest::Error> {
    let req = qr_request::create_qr_request(
        amount,
        prn,
        merchant_code,
        remarks1,
        remarks2,
        username,
        password,
        secret_key,
    );

    status::download_qr(base_url, &req).await
}

pub async fn post_qr_request_status(
    base_url: &str,
    prn: &str,
    merchant_code: &str,
    username: &str,
    password: &str,
    secret_key: &str,
) -> Result<StatusResponse, reqwest::Error> {
    let req = status::build_status_request(prn, merchant_code, username, password, secret_key);
    status::check_status(base_url, &req).await
}

pub async fn get_qr_request_(
    base_url: &str,
    prn: &str,
    merchant_code: &str,
    username: &str,
    password: &str,
    secret_key: &str,
) -> Result<StatusResponse, reqwest::Error> {
    let req = status::build_status_request(prn, merchant_code, username, password, secret_key);
    status::check_status(base_url, &req).await
}

pub async fn get_qr_response(ws_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    websocket::listen_websocket(ws_url).await
}
