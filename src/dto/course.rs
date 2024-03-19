use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct CoursesResult {
    #[serde(rename = "result")]
    pub courses: Vec<Course>,
}

impl FromIterator<Course> for CoursesResult {
    fn from_iter<T: IntoIterator<Item = Course>>(iter: T) -> Self {
        Self {
            courses: iter.into_iter().collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Course {
    /// Internal-Id of the course
    pub id: usize,
    /// Name of the course
    pub title: String,
    /// like "Studio"
    #[serde(rename = "type")]
    pub typ: String,
    /// Course-Duration in minutes
    pub duration: u32,
    /// like "freestyle Kleingruppentraining"
    pub category: String,
    pub description: String,
    /// Image-URL starts with "https://"
    #[serde(rename = "imgUrl")]
    pub image_url: String,
    /// Whether the course is bookable or not
    pub bookable: bool,
}
