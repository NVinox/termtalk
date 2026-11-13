use base64::{Engine, engine::general_purpose::STANDARD};
use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    aead::{Aead, Generate, KeyInit},
};
use sha2::{Digest, Sha256};

pub struct Crypto {
    cipher: ChaCha20Poly1305,
}

impl Crypto {
    pub fn new(password: &str) -> Self {
        let mut hasher = Sha256::new();

        hasher.update(password.as_bytes());

        let key = hasher.finalize();
        let cipher = ChaCha20Poly1305::new(&key);

        Self { cipher }
    }

    pub fn encrypt(&self, plain_text: &str) -> Result<String, String> {
        let nonce = Nonce::generate();
        let ciphertext = self
            .cipher
            .encrypt(&nonce, plain_text.as_bytes())
            .map_err(|_| "Encryption error".to_string())?;
        let mut combined = nonce.to_vec();

        combined.extend_from_slice(&ciphertext);

        Ok(STANDARD.encode(combined))
    }

    pub fn decrypt(&self, encoded_text: &str) -> Result<String, String> {
        let decoded = STANDARD
            .decode(encoded_text)
            .map_err(|_| "Incorrect format Base64".to_string())?;

        if decoded.len() < 12 {
            return Err("Packet is too short".to_string());
        }

        let (nonce_bytes, ciphertext) = decoded.split_at(12);
        let nonce_array: [u8; 12] = nonce_bytes
            .try_into()
            .map_err(|_| "Incorrect nonce length".to_string())?;
        let nonce = Nonce::from(nonce_array);
        let plain_bytes = self
            .cipher
            .decrypt(&nonce, ciphertext)
            .map_err(|_| "Failed to decrypt the message".to_string())?;

        String::from_utf8(plain_bytes).map_err(|_| "UTF-8 encoding error".to_string())
    }
}
