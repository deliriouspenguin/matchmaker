# Matchmaker

This library can be used to fairly match students to categories (or activities). Or students to schools. Or anything to anything else.

## Background

The matching problem this library solves is mathematically know as the residence problem and is a subset of the [stable marriage problem](https://en.wikipedia.org/wiki/Stable_marriage_problem).
There are sereval known algorithms that solve this problem, each with there own pros and cons. For more information about the subject see also [Matching algorithms for the secondary school admission problem in Amsterdam](https://staff.fnwi.uva.nl/b.bredeweg/pdf/BSc/20152016/Klijnsma.pdf).

## Algorithm

At this time this library only implements the `Deferred Acceptance - Single Tie Break` algorithm. The library has been designed to make the implementation of other algorithms possible (it just needs to be done ;).

## Usage

## Default matching

Students are distributed over multiple categories, but each student can only be placed once.

```rust
use matchmaker::da_stb::match_students;
use matchmaker::{Category, Student};
use rand::thread_rng;
use std::collections::VecDeque;

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
```

This should the following result:

```text
Students matched to categories:

Cooking:
 - Bert
Reading:
 - Suze
Walking:

All students could be placed.
```

## Place students in multiple categories

Students are distributed over multiple categories. A single student can be placed
in more than one category.

# Example

```rust
use matchmaker::{Category, Student};
use matchmaker::da_stb::match_students_to_multiple_categories;
use rand::thread_rng;
use std::collections::VecDeque;

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
    VecDeque::from(vec![cooking.clone(), reading.clone()]),
    Vec::from([walking.clone()]),
);

let mut rng = thread_rng();
let categories = Vec::from([cooking, reading, walking]);

let match_result = match_students_to_multiple_categories(
    Vec::from([bert, suze]),
    &categories,
    &mut rng);

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
```

This should the following result:

```text
Students matched to categories:

Cooking:
 - Suze
 - Bert
Reading:
 - Bert
 - Suze
Walking:
 - Bert

All students could be placed.
```

---

// Copyright (c) 2020 Delirious Penguin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.