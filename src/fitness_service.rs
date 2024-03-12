use std::error::Error;

use crate::{
    http_client::{HttpClient, ReqwestHttpClient},
    login_service::LoginCreds,
    response::Response,
};

pub struct FitnessService<Creds, Client> {
    pub credendials: Creds,
    pub http_client: Client,
}

impl<Creds> FitnessService<Creds, ReqwestHttpClient>
where
    Creds: LoginCreds,
{
    pub async fn read_courses(&self) -> Result<Response, Box<dyn Error>> {
        self.http_client
            .read_courses(&self.credendials.get_session_id().unwrap())
            .await
    }
}
