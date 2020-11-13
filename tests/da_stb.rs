// Copyright (c) 2020 Delirious Penguin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use matchmaker::{
    da_stb::{match_students, match_students_to_multiple_categories},
    Category, Student,
};
use rand::rngs::mock::StepRng;
use std::collections::VecDeque;

fn get_data(
    cooking_mp: usize,
    reading_mp: usize,
    walking_mp: usize,
) -> (Vec<Student>, Vec<Category>) {
    let cooking = Category::new("Cooking", cooking_mp);
    let reading = Category::new("Reading", reading_mp);
    let walking = Category::new("Walking", walking_mp);

    let bert = Student::new(
        "Bert",
        VecDeque::from(vec![cooking.clone(), reading.clone(), walking.clone()]),
        Vec::new(),
    );
    let suze = Student::new(
        "Suze",
        VecDeque::from(vec![walking.clone(), cooking.clone()]),
        Vec::new(),
    );
    let kate = Student::new(
        "Kate",
        VecDeque::from(vec![walking.clone(), reading.clone()]),
        Vec::new(),
    );
    let harry = Student::new(
        "Harry",
        VecDeque::from(vec![walking.clone()]),
        vec![cooking.clone()],
    );
    let lisa = Student::new("Lisa", VecDeque::new(), Vec::new());

    let categories = vec![cooking, reading, walking];
    let students = vec![bert, suze, kate, harry, lisa];

    (students, categories)
}

#[test]
fn test_match_students() {
    let (students, categories) = get_data(3, 2, 1);
    let mut rng = StepRng::new(2, 0);

    let match_result = match_students(students.clone(), &categories, &mut rng);

    assert_eq!(
        match_result.placed.get(&categories[2].name).unwrap(),
        &vec![students[1].clone()],
        "Suze is in category walking"
    );
    assert_eq!(
        match_result.placed.get(&categories[1].name).unwrap(),
        &vec![students[2].clone(), students[3].clone()],
        "Kate is in category reading"
    );
    assert_eq!(
        match_result.placed.get(&categories[0].name).unwrap().len(),
        2,
        "Rest is in cooking"
    );
    assert!(match_result.not_placable.is_empty(), "Everyone is placed");
}

#[test]
fn test_match_students_not_enough_places() {
    let (students, categories) = get_data(1, 2, 1);
    let mut rng = StepRng::new(2, 0);

    let match_result = match_students(students.clone(), &categories, &mut rng);

    assert_eq!(
        match_result.placed.get(&categories[1].name).unwrap(),
        &vec![students[2].clone(), students[3].clone()],
        "Kate and Harry are in category reading"
    );
    assert_eq!(
        match_result.placed.get(&categories[2].name).unwrap(),
        &vec![students[1].clone()],
        "Suze is in category waling"
    );
    assert_eq!(
        match_result.placed.get(&categories[0].name).unwrap(),
        &vec![students[0].clone()],
        "Bert is in category cooking"
    );
    assert_eq!(
        match_result.not_placable,
        vec![students[4].clone()],
        "Lisa was not placable"
    );
}

#[test]
fn test_match_students_to_multiple_categories() {
    let (students, mut categories) = get_data(3, 1, 3);
    let mut rng = StepRng::new(2, 0);

    let match_result =
        match_students_to_multiple_categories(students.clone(), &mut categories, &mut rng);

    assert_eq!(
        match_result.placed.get(&categories[2].name).unwrap(),
        &vec![
            students[1].clone(),
            students[2].clone(),
            students[3].clone()
        ],
        "Suze, Kate and Harry are in category walking"
    );
    assert_eq!(
        match_result.placed.get(&categories[1].name).unwrap(),
        &vec![students[2].clone()],
        "Kate is in category reading"
    );
    assert_eq!(
        match_result.placed.get(&categories[0].name).unwrap(),
        &vec![
            students[0].clone(),
            students[4].clone(),
            students[1].clone()
        ],
        "Bert, Lisa and Suze are in category cooking"
    );
    assert!(match_result.not_placable.is_empty(), "Everyone is placed");
}

#[test]
fn test_match_students_to_multiple_categories_not_enough_places() {
    let (students, mut categories) = get_data(1, 2, 1);
    let mut rng = StepRng::new(2, 0);

    let match_result =
        match_students_to_multiple_categories(students.clone(), &mut categories, &mut rng);

    assert_eq!(
        match_result.placed.get(&categories[1].name).unwrap(),
        &vec![students[2].clone(), students[3].clone()],
        "Kate and Harry are in category reading"
    );
    assert_eq!(
        match_result.placed.get(&categories[2].name).unwrap(),
        &vec![students[1].clone()],
        "Suze is in category waling"
    );
    assert_eq!(
        match_result.placed.get(&categories[0].name).unwrap(),
        &vec![students[0].clone()],
        "Bert is in category cooking"
    );
    assert_eq!(
        match_result.not_placable,
        vec![students[4].clone()],
        "Lisa was not placable"
    );
}

#[test]
fn test_match_students_to_multiple_categories_more_than_enough_places() {
    let (students, mut categories) = get_data(30, 30, 30);
    let mut rng = StepRng::new(2, 0);

    let match_result =
        match_students_to_multiple_categories(students.clone(), &mut categories, &mut rng);

    assert_eq!(
        match_result.placed.get(&categories[1].name).unwrap(),
        &vec![
            students[3].clone(),
            students[4].clone(),
            students[0].clone(),
            students[1].clone(),
            students[2].clone()
        ],
        "Harry, Lisa, Bert, Suze and Kate are in category reading"
    );
    assert_eq!(
        match_result.placed.get(&categories[2].name).unwrap(),
        &vec![
            students[1].clone(),
            students[2].clone(),
            students[3].clone(),
            students[4].clone(),
            students[0].clone(),
        ],
        "Suze, Kate, Harry, Lisa and Bert are in category waling"
    );
    assert_eq!(
        match_result.placed.get(&categories[0].name).unwrap(),
        &vec![
            students[0].clone(),
            students[4].clone(),
            students[1].clone(),
            students[2].clone(),
        ],
        "Bert, Lisa, Suze and Kate are in category cooking"
    );
    assert_eq!(match_result.not_placable, vec![], "Everyone is placable");
}
