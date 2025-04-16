use dotenvy::dotenv;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

use super::model::{MailerModel, MsgType};

pub async fn send_mail(data: MailerModel) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().unwrap();

    let username = std::env::var("MAIL_USER").unwrap();
    let password = std::env::var("MAIL_USER").unwrap();

    let content_type = match data.msg_type {
        MsgType::HTML => ContentType::TEXT_HTML,
        MsgType::TEXT => ContentType::TEXT_PLAIN,
    };

    let message = Message::builder()
        .from(format!("Club Nsroma <{}>", username).parse()?)
        .to(data.receiver.parse()?)
        .subject(data.subject)
        .header(content_type)
        .body(data.body)?;

    let creds = Credentials::new(username.clone(), password);

    let transport = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    match transport.send(&message) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
