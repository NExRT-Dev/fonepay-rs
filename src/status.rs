use serde::{Serialize, Deserialize};
use crate::hmac_utils::generate_hmac;

#[derive(Serialize)]
pub struct StatusRequest<'a> {
    pub prn: &'a str,
    #[serde(rename = "merchantCode")]
    pub merchant_code: &'a str,
    #[serde(rename = "dataValidation")]
    pub data_validation: String,
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Serialize)]
pub struct QrRequest<'a> {
    pub amount: &'a str,
    pub prn: &'a str,
    #[serde(rename = "merchantCode")]
    pub merchant_code: &'a str,
    pub remarks1: &'a str,
    pub remarks2: &'a str,
    #[serde(rename = "dataValidation")]
    pub data_validation: String,
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct QrResponse {
    pub message: String,
    #[serde(rename = "qrMessage")]
    pub qr_message: String,
    pub status: String,
    #[serde(rename = "statusCode")]
    pub status_code: i32,
    pub success: bool,
    #[serde(rename = "thirdpartyQrWebSocketUrl")]
    pub thirdparty_qr_web_socket_url: String,
}

#[derive(Deserialize, Debug)]
pub struct StatusResponse {
    #[serde(rename = "fonepayTraceId")]
    pub fonepay_trace_id: i32,
    #[serde(rename = "merchantCode")]
    pub merchant_code: String,
    #[serde(rename = "paymentStatus")]
    pub payment_status: String,
    pub prn: String,
}

pub fn build_status_request<'a>(
    prn: &'a str,
    merchant_code: &'a str,
    username: &'a str,
    password: &'a str,
    secret_key: &str,
) -> StatusRequest<'a> {
    let message = format!("{},{}", prn, merchant_code);
    let dv = generate_hmac(secret_key, &message);

    StatusRequest {
        prn,
        merchant_code,
        data_validation: dv,
        username,
        password,
    }
}

pub fn build_qr_request<'a>(
    amount: &'a str,
    prn: &'a str,
    merchant_code: &'a str,
    remarks1: &'a str,
    remarks2: &'a str,
    username: &'a str,
    password: &'a str,
    secret_key: &str,
) -> QrRequest<'a> {
    let message = format!("{},{},{},{},{}", amount, prn, merchant_code, remarks1, remarks2);
    let dv = generate_hmac(secret_key, &message);

    QrRequest {
        amount,
        prn,
        merchant_code,
        remarks1,
        remarks2,
        data_validation: dv,
        username,
        password,
    }
}

pub async fn check_status(base_url: &str, req: &StatusRequest<'_>) -> Result<StatusResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post(format!("{}/api/merchant/merchantDetailsForThirdParty/thirdPartyDynamicQrGetStatus", base_url))
        .json(req)
        .send()
        .await?;

    Ok(res.json().await?)
}

pub async fn download_qr(base_url: &str, req: &QrRequest<'_>) -> Result<QrResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post(format!("{}/api/merchant/merchantDetailsForThirdParty/thirdPartyDynamicQrDownload", base_url))
        .json(req)
        .send()
        .await?;

    Ok(res.json().await?)
}