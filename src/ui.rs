use reqwest::Url;
use std::{io::stdin, sync::Arc};
use wasm_bindgen::JsValue;

use crate::{
    args::Args,
    cookies::Cookie,
    dto::{
        course::Course,
        request::{BookingRequest, EgymLoginRequest},
        slots::Slot,
    },
    fitness_service::FitnessService,
    http_client::HttpClient,
    login_service::{LoginCreds, LoginService},
};

pub async fn run_ui<ClientT, CookieT>(
    http_client: ClientT,
    cookie_jar: Arc<CookieT>,
    args: Args,
) -> Result<(), JsValue>
where
    ClientT: HttpClient,
    CookieT: Cookie,
{
    let http_client = Arc::new(http_client);

    let mut login_service = LoginService::new(Arc::clone(&http_client), Arc::clone(&cookie_jar));

    let login_request = EgymLoginRequest::new(&args.username, &args.password);
    let _response = login_service.do_login(login_request).await;

    let user_id = login_service.get_user_id().expect("propagate ?"); //?;

    let session = cookie_jar.read_cookie("https://mein.fitnessfirst.de");
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

    let booking_res = fitness_service.book_course(booking).await.expect("?"); //?;

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
