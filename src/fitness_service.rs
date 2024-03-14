use std::error::Error;

use crate::{dto::response::Response, http_client::HttpClient, login_service::LoginCreds};

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

#[cfg(test)]
mod test {
    use crate::{mock_client, testutil::MockCall};

    #[tokio::test]
    async fn read_all_courses() {
        mock_client!(MockCall::None, MockCall::None, MockCall::None);
        let creds_mock = CredentialsMock;
        //FitnessService::new(client_mock);
    }
}
