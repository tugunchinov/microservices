use crate::tests::utils::types::{Date, Student, StudentKey};
use lazy_static::lazy_static;
use std::sync::Once;

static INIT: Once = Once::new();

fn setup() {
    INIT.call_once(|| {
        env_logger::init();
    });
}

#[cfg(test)]
mod simple;

#[cfg(test)]
mod concurrent;

lazy_static! {
    pub static ref KEY_1: StudentKey = StudentKey {
        group_id: 591,
        name: "Vasya Pukin".to_string(),
    };
    pub static ref VALUE_1: Student = Student {
        group_id: 591,
        name: "Vasya Pukin".to_string(),
        hometown: "Vasyuki".to_string(),
        birth_date: Date {
            year: 1996,
            month: 4,
            day: 14,
        },
        has_dormitory: true,
        average_score: 7.8,
    };
    pub static ref KEY_2: StudentKey = StudentKey {
        group_id: 591,
        name: "Ahmad Ben Hafiz".to_string(),
    };
    pub static ref VALUE_2: Student = Student {
        group_id: 591,
        name: "Ahmad Ben Hafiz".to_string(),
        hometown: "Cairo".to_string(),
        birth_date: Date {
            year: 1432,
            month: 9,
            day: 2,
        },
        has_dormitory: false,
        average_score: 3.3,
    };
    pub static ref KEY_3: StudentKey = StudentKey {
        group_id: 599,
        name: "John Smith".to_string(),
    };
    pub static ref VALUE_3: Student = Student {
        group_id: 599,
        name: "John Smith".to_string(),
        hometown: "Glasgow".to_string(),
        birth_date: Date {
            year: 1874,
            month: 3,
            day: 8,
        },
        has_dormitory: true,
        average_score: 9.1,
    };
}
