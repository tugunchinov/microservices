use serde_derive::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    pub day: i8,
    pub month: i8,
    pub year: i16,
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub struct StudentKey {
    pub group_id: i32,
    pub name: String,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub group_id: i32,
    pub name: String,
    pub hometown: String,
    pub birth_date: Date,
    pub has_dormitory: bool,
    pub average_score: f64,
}
