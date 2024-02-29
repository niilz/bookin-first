use fitness_api::{
    http_client::ReqwestHttpClient, login_service::LoginService, request::EgymLoginRequest,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Login-User-Name of person who wants to book a class
    #[arg(short, long)]
    pub username: String,

    /// User-Password of the person who wants to book a class
    #[arg(short, long)]
    pub password: String,

    /// Client-ID, should be replaced by retrieving it programatically
    #[arg(short, long)]
    pub clientid: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let http_client = ReqwestHttpClient {
        client: reqwest::Client::new(),
    };
    let mut login_service = LoginService {
        token: None,
        http_client,
    };
    let login_request = EgymLoginRequest::new(&args.username, &args.password, &args.clientid);
    let response = login_service.do_login(login_request).await;

    println!("{:?}", login_service.token);
}
