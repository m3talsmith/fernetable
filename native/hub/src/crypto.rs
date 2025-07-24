use fernet::Fernet;
use rinf::{DartSignal, RustSignal};

use crate::signals::{Command, CommandRequest, CommandResponse};

pub async fn listen() {
    let command_reciever = CommandRequest::get_dart_signal_receiver();
    while let Some(request) = command_reciever.recv().await {
        match request.message.command {
            Command::EncryptConfig => {
                match encrypt_config(
                    request.message.key.unwrap(),
                    request.message.config.unwrap(),
                ) {
                    Ok(encrypted_config) => CommandResponse {
                        encrypted_config: Some(encrypted_config),
                        decrypted_config: None,
                        error: None,
                        key: None,
                    }
                    .send_signal_to_dart(),
                    Err(e) => CommandResponse {
                        error: Some(e),
                        encrypted_config: None,
                        decrypted_config: None,
                        key: None,
                    }
                    .send_signal_to_dart(),
                }
            }
            Command::DecryptConfig => {
                match decrypt_config(
                    request.message.key.unwrap(),
                    request.message.config.unwrap(),
                ) {
                    Ok(decrypted_config) => CommandResponse {
                        encrypted_config: None,
                        decrypted_config: Some(decrypted_config),
                        error: None,
                        key: None,
                    }
                    .send_signal_to_dart(),
                    Err(e) => CommandResponse {
                        error: Some(e),
                        encrypted_config: None,
                        decrypted_config: None,
                        key: None,
                    }
                    .send_signal_to_dart(),
                }
            }
            Command::GenerateKey => {
                let key = generate_key();
                CommandResponse {
                    key: Some(key),
                    encrypted_config: None,
                    decrypted_config: None,
                    error: None,
                }
                .send_signal_to_dart();
            }
        };
    }
}

fn generate_key() -> String {
    let key = Fernet::generate_key();
    key.to_string()
}

fn encrypt_config(key: String, config: String) -> Result<String, String> {
    let key = match Fernet::new(&key) {
        Some(key) => key,
        None => return Err("Invalid key".to_string()),
    };
    let encrypted_config = key.encrypt(config.as_bytes());
    Ok(encrypted_config)
}

fn decrypt_config(key: String, config: String) -> Result<String, String> {
    let key = match Fernet::new(&key) {
        Some(key) => key,
        None => return Err("Invalid key".to_string()),
    };
    let decrypted_config = match key.decrypt(config.as_str()) {
        Ok(decrypted_config) => String::from_utf8(decrypted_config).unwrap(),
        Err(e) => return Err(e.to_string()),
    };
    Ok(decrypted_config)
}
