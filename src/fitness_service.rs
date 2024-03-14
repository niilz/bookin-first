use std::error::Error;

use crate::{http_client::HttpClient, login_service::LoginCreds, response::Response};

pub struct FitnessService<Creds, Client> {
    credendials: Creds,
    http_client: Client,
}

impl<Creds, Client> FitnessService<Creds, Client>
where
    Creds: LoginCreds,
    Client: HttpClient,
{
    pub fn new(credendials: Creds, http_client: Client) -> Self {
        Self {
            credendials,
            http_client,
        }
    }
    pub async fn read_courses(&self) -> Result<Response, Box<dyn Error>> {
        self.http_client
            .read_courses(&self.credendials.get_session_id().unwrap())
            .await
    }
}
