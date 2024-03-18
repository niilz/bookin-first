use super::course::Course;

#[derive(Debug)]
pub enum Response {
    Text(String),
    SessionSet,
    Courses(Vec<Course>),
}
