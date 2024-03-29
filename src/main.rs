use std::{io::stdin, sync::Arc};

use fitness_api::{
    dto::{course::Course, request::EgymLoginRequest},
    fitness_service::FitnessService,
    http_client::ReqwestHttpClient,
    login_service::{LoginCreds, LoginService},
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

    /// Optional Flag to signal that we want the following course without prompting the user
    #[arg(short, long)]
    pub course_name: Option<String>,
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

    let login_request = EgymLoginRequest::new(&args.username, &args.password);
    let _response = login_service.do_login(login_request).await;

    println!(
        "TODO: Build the booking reuquest. Meanwhile here is the user-ID: {:?}",
        login_service.get_user_id()
    );

    let session = cookie_jar.cookies(&Url::parse("https://mein.fitnessfirst.de").unwrap());
    println!("Session: {session:?}");

    let fitness_service = FitnessService::new(login_service, Arc::clone(&http_client));

    let courses = fitness_service.fetch_courses().await.expect("read courses");

    let course_choice = match args.course_name {
        Some(course) => course,
        None => handle_user_input(&courses),
    };

    let course = courses
        .iter()
        .find(|c| c.title.contains(&course_choice))
        .expect("find course");

    println!();
    println!("You're chosen course is:");
    println!();
    println!("Course: {course:#?}");

    let slots = fitness_service
        .fetch_slots(course.id)
        .await
        .expect("read slots");

    println!();
    println!("Available Slots:");
    for (idx, slot) in slots.iter().enumerate() {
        println!(
            "{} - {} - {}",
            idx + 1,
            slot.start_date_time,
            slot.earliest_booking_date_time
        );
    }
    println!();

    println!("Start booking now");

    /*
    let booking = BookingRequest {
        customer_id: ,
        slot_id: todo!(),
        course_id: todo!(),
        club_id: todo!(),
        club_name: todo!(),
        course_name: todo!(),
    };
    */
}

fn handle_user_input(courses: &Vec<Course>) -> String {
    println!("The courses are:");

    for (idx, course) in courses.iter().enumerate() {
        println!("{}: {}", idx + 1, course.title);
    }
    println!();
    println!("Insert Course Name");

    let mut user_input = String::new();
    let _ = stdin().read_line(&mut user_input).expect("read user input");
    user_input.trim().to_string()
}
