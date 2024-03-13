use std::sync::Arc;

use fitness_api::{
    fitness_service::FitnessService, http_client::ReqwestHttpClient, login_service::LoginService,
    request::EgymLoginRequest,
};

use clap::Parser;
use reqwest::{
    cookie::{CookieStore, Jar},
    Url,
};

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

    let cookie_jar = Arc::new(Jar::default());
    let client = reqwest::Client::builder()
        .cookie_provider(Arc::clone(&cookie_jar))
        .build()
        .expect("Could not create client");
    let http_client = ReqwestHttpClient { client };
    let mut login_service = LoginService::new(http_client, Arc::clone(&cookie_jar));

    let login_request = EgymLoginRequest::new(&args.username, &args.password, &args.clientid);
    let response = login_service.do_login(login_request).await;

    let session = cookie_jar.cookies(&Url::parse("https://mein.fitnessfirst.de").unwrap());
    println!("Session: {session:?}");

    let http_client_2 = ReqwestHttpClient {
        client: reqwest::Client::new(),
    };

    let fitness_service = FitnessService {
        credendials: login_service,
        http_client: http_client_2,
    };

    let courses = fitness_service.read_courses().await;

    println!("Courses: {:#?}", courses);
}
