extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::ContentType;

pub struct MessagingService {
    pub username: String,
    pub password: String,
    pub sender: String,
}

impl MessagingService {
    pub fn new<T: Into<String>>(username: T, password: T, sender: T) -> MessagingService {
        MessagingService {
            username: username.into(),
            password: password.into(),
            sender: sender.into(),
        }
    }

    pub fn send_message(&self, recipient: &str, subject: &str, message: &str) -> Result<String, String> {
        let res = Client::new().post(
            "https://api.sendgrid.com/api/mail.send.json"
        ).header(
            ContentType(
            "application/x-www-form-urlencoded".parse().unwrap()
        )).body(&format!(
            "api_user={user}&api_key={key}&to={to}&subject={subject}&text={body}&from={from}",
            user = self.username,
            key = self.password,
            to = recipient,
            subject = subject,
            body = message,
            from = self.sender,
        )).send();

        match res {
            Ok(mut res) => {
                let mut buf = String::new();
                res.read_to_string(&mut buf).ok();
                Ok(buf)
            },
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
