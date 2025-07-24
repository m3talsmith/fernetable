use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, SignalPiece)]
pub enum Command {
    EncryptConfig,
    DecryptConfig,
    GenerateKey,
}

#[derive(Deserialize, DartSignal)]
pub struct CommandRequest {
    pub command: Command,
    pub config: Option<String>,
    pub key: Option<String>,
}

#[derive(Serialize, RustSignal)]
pub struct CommandResponse {
    pub error: Option<String>,
    pub encrypted_config: Option<String>,
    pub decrypted_config: Option<String>,
    pub key: Option<String>,
}
