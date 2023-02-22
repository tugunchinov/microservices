use crate::tests::utils::types::{Date, Student, StudentKey};

#[cfg(test)]
mod simple;

#[cfg(test)]
mod concurrent;

static KEY_1: StudentKey = StudentKey {
    group_id: 591,
    name: "Vasya Pukin",
};
static VALUE_1: Student = Student {
    group_id: 591,
    name: "Vasya Pukin",
    hometown: "Vasyuki",
    birth_date: Date {
        year: 1996,
        month: 4,
        day: 14,
    },
    has_dormitory: true,
    average_score: 7.8,
};

static KEY_2: StudentKey = StudentKey {
    group_id: 591,
    name: "Ahmad Ben Hafiz",
};
static VALUE_2: Student = Student {
    group_id: 591,
    name: "Ahmad Ben Hafiz",
    hometown: "Cairo",
    birth_date: Date {
        year: 1432,
        month: 9,
        day: 2,
    },
    has_dormitory: false,
    average_score: 3.3,
};

static KEY_3: StudentKey = StudentKey {
    group_id: 599,
    name: "John Smith",
};
static VALUE_3: Student = Student {
    group_id: 599,
    name: "John Smith",
    hometown: "Glasgow",
    birth_date: Date {
        year: 1874,
        month: 3,
        day: 8,
    },
    has_dormitory: true,
    average_score: 9.1,
};
