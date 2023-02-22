#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Date {
    pub day: i8,
    pub month: i8,
    pub year: i16,
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct StudentKey {
    pub group_id: i32,
    pub name: &'static str,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Student {
    pub group_id: i32,
    pub name: &'static str,
    pub hometown: &'static str,
    pub birth_date: Date,
    pub has_dormitory: bool,
    pub average_score: f64,
}
