use std::{io::stdin, sync::Arc};

use fitness_api::{
    dto::request::EgymLoginRequest, fitness_service::FitnessService,
    http_client::ReqwestHttpClient, login_service::LoginService,
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
    let http_client = Arc::new(ReqwestHttpClient { client });
    let mut login_service = LoginService::new(Arc::clone(&http_client), Arc::clone(&cookie_jar));

    let login_request = EgymLoginRequest::new(&args.username, &args.password, &args.clientid);
    let _response = login_service.do_login(login_request).await;

    let session = cookie_jar.cookies(&Url::parse("https://mein.fitnessfirst.de").unwrap());
    println!("Session: {session:?}");

    let fitness_service = FitnessService::new(login_service, Arc::clone(&http_client));

    let courses = fitness_service.read_courses().await.expect("read courses");

    println!("The courses are:");

    for (idx, course) in courses.iter().enumerate() {
        println!("{}: {}", idx + 1, course.title);
    }
    println!();
    println!("Insert Course Name");

    let mut user_input = String::new();
    let _ = stdin().read_line(&mut user_input).expect("read user input");
    let user_course = user_input.trim();

    let course = courses
        .iter()
        .find(|c| c.title.contains(user_course))
        .expect("find course");

    println!();
    println!("You're chosen course is:");
    println!();
    println!("Course: {course:#?}");

    // - find slots by course id
    // - implement course-booking with slot- and course id
}
