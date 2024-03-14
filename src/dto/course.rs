use serde::Deserialize;

#[derive(Deserialize)]
pub struct Course {
    /// Internal-Id of the course
    id: usize,
    /// Name of the course
    title: String,
    /// like "Studio"
    #[serde(rename = "type")]
    typ: String,
    /// Course-Duration in minutes
    duration: u32,
    /// like "freestyle Kleingruppentraining"
    category: String,
    description: String,
    /// Image-URL starts with "https://"
    #[serde(rename = "imgUrl")]
    image_url: String,
    /// Whether the course is bookable or not
    bookable: bool,
}
