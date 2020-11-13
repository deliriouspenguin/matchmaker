// Copyright (c) 2020 Delirious Penguin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use matchmaker::da_stb::match_students;
use matchmaker::{Category, Student};
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    // Create categories
    let cooking = Category::new("Cooking", 10);
    let reading = Category::new("Reading", 10);
    let walking = Category::new("Walking", 5);

    // Create student Bert
    // Bert wishes to be placed in category cooking or reading (in that order)
    let bert = Student::new(
        "Bert",
        VecDeque::from(vec![cooking.clone(), reading.clone()]),
        Vec::new(),
    );

    // Create student Suze
    // Suze wishes to be placed in category cooking or reading (in that order),
    // but does not wish to be placed in category walking
    let suze = Student::new(
        "Suze",
        VecDeque::from(vec![reading.clone(), cooking.clone()]),
        Vec::from([walking.clone()]),
    );

    let mut rng = thread_rng();
    let categories = Vec::from([cooking, reading, walking]);

    let match_result = match_students(Vec::from([bert, suze]), &categories, &mut rng);

    println!("Students matched to categories:");
    println!();
    for category in &categories {
        println!("{}:", &category.name);
        for student in match_result
            .placed
            .get(&category.name)
            .unwrap_or(&Vec::new())
        {
            println!(" - {}", &student.name);
        }
    }

    if match_result.not_placable.is_empty() {
        println!();
        println!("All students could be placed.");
    }
}
