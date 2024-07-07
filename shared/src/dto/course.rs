use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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
pub enum Course {
    Web(SimpleCourse),
    App(CourseWithSlot),
}

impl Course {
    pub fn name(&self) -> String {
        match self {
            Self::Web(course) => course.title.clone(),
            Self::App(course) => course.brief.name.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct SimpleCourse {
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct CourseWithSlot {
    pub brief: Brief,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Brief {
    /// User-Id in the form "12345678:12345678"
    pub id: String,
    /// Name of the course
    pub name: String,
    #[serde(rename = "startDateTime")]
    pub start_date_time: u64,
    #[serde(rename = "endDateTime")]
    pub end_date_time: u64,
    #[serde(rename = "maxCapacity")]
    pub max_capacity: u32,
    #[serde(rename = "totalBooked")]
    pub total_booked: u32,
    /*
      "waitlistCapacity": 0,
      "waitlistBooked": 0,
      "availableSpots": null,
      "instructor": {
        "id": "Fitness Coach",
        "fullName": "Fitness Coach",
        "customInfo": null
      },
      "activity": {
        "id": "freestyle Kleingruppentraining",
        "description": "freestyle Kleingruppentraining"
      },
      "free": true,
      "childCare": false,
      "reservable": false,
      "liveStreamClass": false,
      "availableOptions": null,
      "cancelled": false,
      "booked": false,
      "waitlisted": null,
      "customInfo": null,
      "clubUuid": "37d7c5d3-6594-4853-a1e6-c116ac084690"
    },
    "details": {
      "id": null,
      "description": "Du möchtest deinen gesamten Körper trainieren und deine Kraft, Ausdauer, Beweglichkeit und Koordination verbessern? Dann ist Move It Total Body genau das richtige für dich! Bei diesem freestyle-Kurs trainierst du alle Hauptmuskelgruppen des Körpers, sowohl mit Bodyweight Übungen, als auch mit Equipment, wie beispielsweise Kettlebells, Corebags oder Medizinbällen. So verbrennst du Kalorien, definierst deinen Körper und steigerst gleichzeitig deine Leistungsfähigkeit. Egal, ob Einsteiger oder Fortgeschrittener, mit Move It Total Body bieten wir dir ein abwechslungsreiches Allround-Workout, das dich so richtig in Bewegung bringt. ",
      "webCapacity": null,
      "webBooked": null,
      "room": {
        "id": "freestyle Fläche",
        "description": "freestyle Fläche",
        "roomPhotoUrl": null,
        "customInfo": null
      },
      "level": null,
      "pricing": null,
      "legalNotes": null,
      "additionalInfo": null,
      "customInfo": null,
      "cancellationWindowEnd": 1719653400000,
      "bookingWindowStart": 1719567000000,
      "bookingWindowEnd": null,
      "imageUrl": null,
      "videoUrl": null,
      "liveStreamLink": null
    },
    "attendeeDetails": null
    */
}
