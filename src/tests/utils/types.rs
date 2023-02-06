use chrono::NaiveDateTime;

#[derive(Hash)]
pub struct StudentKey {
    pub group_id: i32,
    pub name: String,
}

pub struct Student {
    pub group_id: i32,
    pub name: String,
    pub hometown: String,
    pub birth_date: NaiveDateTime,
    pub has_dormitory: bool,
    pub average_score: f64,
}
