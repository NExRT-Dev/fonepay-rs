use serde::Serialize;
use crate::hmac_utils::generate_hmac;

#[derive(Serialize)]
pub struct QrRequest<'a> {
    pub amount: &'a str,
    #[serde(rename = "remarks1")]
    pub remarks1: &'a str,
    #[serde(rename = "remarks2")]
    pub remarks2: &'a str,
    #[serde(rename = "prn")]
    pub prn: &'a str,
    #[serde(rename = "merchantCode")]
    pub merchant_code: &'a str,
    #[serde(rename = "dataValidation")]
    pub data_validation: String,
    #[serde(rename = "username")]
    pub username: &'a str,
    #[serde(rename = "password")]
    pub password: &'a str,
}

pub fn create_qr_request<'a>(
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
        remarks1,
        remarks2,
        prn,
        merchant_code,
        data_validation: dv,
        username,
        password,
    }
}

pub async fn send_qr_request(base_url: &str, req: &QrRequest<'_>) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post(format!("{}/api/merchant/merchantDetailsForThirdParty/thirdPartyDynamicQrDownload", base_url))
        .json(req)
        .send()
        .await?;

    Ok(res.text().await?)
}