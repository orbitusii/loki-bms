#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Auth(AuthMessage),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthMessage {
    pub username: Option<String>,
    pub password: Option<String>,
}
