use hmac::{Hmac, Mac};
use sha2::Sha512;
use hex;

type HmacSha512 = Hmac<Sha512>;

pub fn generate_hmac(secret: &str, message: &str) -> String {
    let mut mac: HmacSha512 = HmacSha512::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(message.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}