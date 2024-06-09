pub mod args;

use std::{io::stdin, sync::Arc};

use booking_first_lib::{
    booking_service::BookingService,
    dto::{course::Course, error::BoxDynError, request::BookingRequest, slots::Slot},
    http_client::HttpClientSend,
};

use self::args::Args;

pub async fn run_cli<ClientT>(http_client: ClientT, args: Args) -> Result<(), BoxDynError>
where
    ClientT: HttpClientSend,
{
    let http_client = Arc::new(http_client);

    let booking_service = BookingService::new(http_client);

    let login_credentials = booking_service
        .login(&args.username, &args.password)
        .await
        .expect("LoginCreds not present after login?");

    let course_response = booking_service
        .fetch_courses(&login_credentials.session)
        .await;

    let course_choice = match &args.course_name {
        Some(course) => course.to_string(),
        None => course_input(&course_response),
    };

    let course = course_response
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

    let slots = booking_service
        .fetch_slots(&course, &login_credentials)
        .await;

    println!();

    println!("Start booking now");

    let slot_choice = match &args.course_name {
        Some(_) => todo!("select next possible slot automatically, or add date-input"),
        None => slot_input(&slots),
    };

    let booking = BookingRequest::new(
        login_credentials.user_id,
        slot_choice.id,
        course.id,
        course.title,
    );

    let booking_res = booking_service
        .book_course(booking, &login_credentials)
        .await;

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
