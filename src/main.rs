use std::{error::Error, io::stdin, sync::Arc};

use fitness_api::{
    dto::{
        course::Course,
        request::{BookingRequest, EgymLoginRequest},
        slots::Slot,
    },
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
async fn main() -> Result<(), Box<dyn Error>> {
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

    let user_id = login_service.get_user_id()?;

    let session = cookie_jar.cookies(&Url::parse("https://mein.fitnessfirst.de").unwrap());
    println!("Session: {session:?}");

    let fitness_service = FitnessService::new(login_service, Arc::clone(&http_client));

    let courses = fitness_service.fetch_courses().await.expect("read courses");

    let course_choice = match &args.course_name {
        Some(course) => course.to_string(),
        None => course_input(&courses),
    };

    let course = courses
        .into_iter()
        .find(|c| {
            c.title
                .to_lowercase()
                .contains(&course_choice.to_lowercase())
        })
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

    println!("Start booking now");

    let slot_choice = match &args.course_name {
        Some(_) => todo!("select next possible slot automatically, or add date-input"),
        None => slot_input(&slots),
    };

    let booking = BookingRequest::new(user_id, slot_choice.id, course.id, course.title);

    let booking_res = fitness_service.book_course(booking).await?;

    println!("Booking: {booking_res:?}");

    Ok(())
}

fn course_input(courses: &Vec<Course>) -> String {
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

fn slot_input(slots: &Vec<Slot>) -> &'_ Slot {
    println!("Available Slots:");
    for (idx, slot) in slots.iter().enumerate() {
        println!(
            "{}. slot: {} - bookable at: {}",
            idx + 1,
            slot.start_date_time,
            slot.earliest_booking_date_time
        );
    }
    println!();
    println!("Please select a slot (enter slot number)");
    let mut slot_nr = String::new();
    let _ = stdin().read_line(&mut slot_nr).expect("read user input");
    let slot_nr = slot_nr.trim().to_string().parse::<usize>();
    let nr = match slot_nr {
        Ok(nr) => nr.saturating_sub(1),
        Err(_) => return slot_input(slots),
    };

    if nr >= slots.len() {
        return slot_input(slots);
    }

    &slots[nr]
}
