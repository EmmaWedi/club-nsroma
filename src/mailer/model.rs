use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MailerModel {
    pub subject: String,
    pub body: String,
    pub receiver: String,
    pub msg_type: MsgType
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum MsgType {
    HTML,
    TEXT,
}