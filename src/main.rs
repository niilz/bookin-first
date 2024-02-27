use fitness_api::{LoginRequest, LoginService, ReqwestHttpClient};

#[tokio::main]
async fn main() {
    let http_client = ReqwestHttpClient {
        client: reqwest::Client::new(),
    };
    let mut login_service = LoginService {
        token: None,
        http_client,
    };
    let login_request = LoginRequest::new("foo", "bar");
    login_service.do_login(login_request).await;
    println!("{:?}", login_service.token);
}
