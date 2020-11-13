// Copyright (c) 2020 Delirious Penguin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Implements the Deferred Acceptance - Single Tie Break algorithm

use super::{Category, MatchResult, OrderedStudent, Student};
use rand::prelude::*;
use std::collections::HashMap;

/// Match students to more than one category
///
/// Use this function when a single student can be placed simultaniously
/// in more than one category
///
/// # Example
///
/// ```
/// use matchmaker::{Category, Student};
/// use matchmaker::da_stb::match_students_to_multiple_categories;
/// use rand::thread_rng;
/// use std::collections::VecDeque;
///
/// // Create categories
/// let cooking = Category::new("Cooking", 10);
/// let reading = Category::new("Reading", 10);
/// let walking = Category::new("Walking", 5);
///
/// // Create student Bert
/// // Bert wishes to be placed in category cooking or reading (in that order)
/// let bert = Student::new(
///     "Bert",
///     VecDeque::from(vec![cooking.clone(), reading.clone()]),
///     Vec::new(),
/// );
///
/// // Create student Suze
/// // Suze wishes to be placed in category cooking or reading (in that order),
/// // but does not wish to be placed in category walking
/// let suze = Student::new(
///     "Suze",
///     VecDeque::from(vec![cooking.clone(), reading.clone()]),
///     Vec::from([walking.clone()]),
/// );
///
/// let mut rng = thread_rng();
/// let categories = Vec::from([cooking, reading, walking]);
///
/// let match_result = match_students_to_multiple_categories(
///     Vec::from([bert, suze]),
///     &categories,
///     &mut rng);
///
//// println!("Students matched to categories:");
//// println!();
//// for category in &categories {
////     println!("{}:", &category.name);
////     for student in match_result
////         .placed
////         .get(&category.name)
////         .unwrap_or(&Vec::new())
////     {
////         println!(" - {}", &student.name);
////     }
//// }
////
//// if match_result.not_placable.is_empty() {
////     println!();
////     println!("All students could be placed.");
//// }
/// ```
///
/// The result will be something like this:
///
/// ```text
/// Students matched to categories:
///
/// Cooking:
///  - Suze
///  - Bert
/// Reading:
///  - Bert
///  - Suze
/// Walking:
///  - Bert
///
/// All students could be placed.
/// ```
pub fn match_students_to_multiple_categories(
    mut students: Vec<Student>,
    categories: &Vec<Category>,
    mut rng: &mut impl Rng,
) -> MatchResult {
    let mut match_result: MatchResult = MatchResult {
        placed: HashMap::new(),
        not_placable: Vec::new(),
    };
    let mut categories = categories.clone();
    let mut spots_available = categories.iter().map(|c| c.max_placements).sum();
    let mut previous_spots_available = usize::MAX;
    let mut first_round = true;

    // Keep going until there are no more spots or until no more new spots are filled.
    while spots_available > 0 && previous_spots_available > spots_available {
        let mut new_match_result = match_students(students.clone(), &categories, &mut rng);

        // Merge match_result.placable and prepare categories and students for next round.
        for category in categories.iter_mut() {
            if let Some(placed_students) = new_match_result.placed.remove(&category.name) {
                // Update the category with the amount of spots left.
                category.max_placements -= placed_students.len();

                // Add this category to match_result
                if match_result.placed.get(&category.name).is_none() {
                    match_result
                        .placed
                        .insert(category.name.clone(), Vec::new());
                }

                for ps in placed_students {
                    for student in students.iter_mut().filter(|s| s.name == ps.name) {
                        // Make sure students placed in this category can't be assigned to it in the next round.
                        student.exclude.push(category.clone());
                    }

                    // Add student to match_result (we can safely unwrap here, because we just added the category).
                    match_result
                        .placed
                        .get_mut(&category.name)
                        .unwrap()
                        .push(ps);
                }
            }
        }

        // Only use not_placable result from first round. Students later not placed are placed the first time.
        if first_round {
            match_result.not_placable = new_match_result.not_placable;
            first_round = false;
        }

        previous_spots_available = spots_available;
        spots_available = categories.iter().map(|c| c.max_placements).sum();
    }

    match_result
}

/// Match students to categories
///
/// Use this function if each student can only be placed in one category
///
/// # Example
///
/// ```
/// use matchmaker::da_stb::match_students;
/// use matchmaker::{Category, Student};
/// use rand::thread_rng;
/// use std::collections::VecDeque;
///
/// // Create categories
/// let cooking = Category::new("Cooking", 10);
/// let reading = Category::new("Reading", 10);
/// let walking = Category::new("Walking", 5);
///
/// // Create student Bert
/// // Bert wishes to be placed in category cooking or reading (in that order)
/// let bert = Student::new(
///     "Bert",
///     VecDeque::from(vec![cooking.clone(), reading.clone()]),
///     Vec::new(),
/// );
///
/// // Create student Suze
/// // Suze wishes to be placed in category cooking or reading (in that order),
/// // but does not wish to be placed in category walking
/// let suze = Student::new(
///     "Suze",
///     VecDeque::from(vec![reading.clone(), cooking.clone()]),
///     Vec::from([walking.clone()]),
/// );
///
/// let mut rng = thread_rng();
/// let categories = Vec::from([cooking, reading, walking]);
///
/// let match_result = match_students(Vec::from([bert, suze]), &categories, &mut rng);
///
/// println!("Students matched to categories:");
/// println!();
/// for category in &categories {
///     println!("{}:", &category.name);
///     for student in match_result
///         .placed
///         .get(&category.name)
///         .unwrap_or(&Vec::new())
///     {
///         println!(" - {}", &student.name);
///     }
/// }
///
/// if match_result.not_placable.is_empty() {
///     println!();
///     println!("All students could be placed.");
/// }
/// ```
///
/// This should the following result:
///
/// ```text
/// Students matched to categories:
///
/// Cooking:
///  - Bert
/// Reading:
///  - Suze
/// Walking:
///
/// All students could be placed.
/// ```
pub fn match_students(
    students: Vec<Student>,
    categories: &Vec<Category>,
    mut rng: &mut impl Rng,
) -> MatchResult {
    let mut unplaced_students = draw_order(students, &mut rng);
    let mut not_placable: Vec<OrderedStudent> = vec![];
    let mut placed: HashMap<String, Vec<OrderedStudent>> = HashMap::new();

    // Place students in categories based on preferences
    while !unplaced_students.is_empty() {
        place_students(unplaced_students, &mut placed, &mut not_placable);
        unplaced_students = truncate_categories(&mut placed, &categories)
    }

    // Randomly assign unplaced students among open spots in categories.
    let not_placable = assign_random(not_placable, &mut placed, &categories, &mut rng);

    MatchResult::from(placed, not_placable)
}

fn draw_order(mut students: Vec<Student>, mut rng: &mut impl Rng) -> Vec<OrderedStudent> {
    students.shuffle(&mut rng);

    students
        .into_iter()
        .enumerate()
        .map(|(i, s)| OrderedStudent {
            name: s.name,
            preferences: s.preferences,
            exclude: s.exclude,
            order: i,
        })
        .collect()
}

fn place_students(
    unplaced_students: Vec<OrderedStudent>,
    placed: &mut HashMap<String, Vec<OrderedStudent>>,
    not_placable: &mut Vec<OrderedStudent>,
) {
    for mut student in unplaced_students.into_iter() {
        if let Some(category) = student.preferences.pop_front() {
            if student.exclude.contains(&category) {
                not_placable.push(student);
            } else {
                match placed.get_mut(&category.name) {
                    Some(placed_students) => placed_students.push(student),
                    None => {
                        placed.insert(category.name, vec![student]);
                    }
                }
            }
        } else {
            not_placable.push(student);
        }
    }
}

fn truncate_categories(
    placed: &mut HashMap<String, Vec<OrderedStudent>>,
    categories: &Vec<Category>,
) -> Vec<OrderedStudent> {
    let mut unplaced_students: Vec<OrderedStudent> = Vec::new();

    for category in categories {
        if let Some(placed_students) = placed.get_mut(&category.name) {
            if placed_students.len() > category.max_placements {
                placed_students.sort();
                for student in placed_students.drain(category.max_placements..placed_students.len())
                {
                    unplaced_students.push(student);
                }
            }
        }
    }
    unplaced_students
}

fn assign_random(
    mut not_placable: Vec<OrderedStudent>,
    placed: &mut HashMap<String, Vec<OrderedStudent>>,
    categories: &Vec<Category>,
    mut rng: &mut impl Rng,
) -> Vec<OrderedStudent> {
    // Sort in order so best lots gets selected first.
    not_placable.sort();

    let mut still_not_placable: Vec<OrderedStudent> = Vec::new();

    for student in not_placable.into_iter() {
        let open_categories: Vec<&Category> = categories
            .iter()
            .filter(|c| {
                placed
                    .get(&c.name)
                    .unwrap_or(&Vec::<OrderedStudent>::new())
                    .len()
                    < c.max_placements
            })
            .filter(|c| !student.exclude.contains(&c))
            .collect();

        if let Some(&category) = open_categories.iter().choose(&mut rng) {
            placed
                .entry(category.name.clone())
                .or_insert(Vec::<OrderedStudent>::new())
                .push(student);
        } else {
            still_not_placable.push(student);
        }
    }

    still_not_placable
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;
    use std::collections::VecDeque;

    #[test]
    fn test_draw_order() {
        let mut rng = StepRng::new(2, 0);

        let students = vec![
            Student {
                name: "Bert".into(),
                preferences: VecDeque::new(),
                exclude: Vec::new(),
            },
            Student {
                name: "Kate".into(),
                preferences: VecDeque::new(),
                exclude: Vec::new(),
            },
            Student {
                name: "Harry".into(),
                preferences: VecDeque::new(),
                exclude: Vec::new(),
            },
        ];

        let ordered_students = draw_order(students, &mut rng);

        let assert_ordered_students = vec![
            OrderedStudent {
                name: "Kate".into(),
                preferences: VecDeque::new(),
                exclude: Vec::new(),
                order: 0,
            },
            OrderedStudent {
                name: "Harry".into(),
                preferences: VecDeque::new(),
                exclude: Vec::new(),
                order: 1,
            },
            OrderedStudent {
                name: "Bert".into(),
                preferences: VecDeque::new(),
                exclude: Vec::new(),
                order: 2,
            },
        ];

        assert_eq!(ordered_students, assert_ordered_students);
    }

    #[test]
    fn test_place_students() {
        let cooking = Category {
            name: "Cooking".into(),
            max_placements: 3,
        };
        let reading = Category {
            name: "Reading".into(),
            max_placements: 2,
        };
        let walking = Category {
            name: "Walking".into(),
            max_placements: 1,
        };

        let mut bert = OrderedStudent {
            name: "Bert".into(),
            preferences: VecDeque::from(vec![cooking.clone(), reading.clone(), walking.clone()]),
            exclude: Vec::new(),
            order: 0,
        };
        let mut kate = OrderedStudent {
            name: "Kate".into(),
            preferences: VecDeque::from(vec![walking.clone()]),
            exclude: Vec::new(),
            order: 1,
        };
        let mut suze = OrderedStudent {
            name: "Suze".into(),
            preferences: VecDeque::from(vec![walking.clone(), cooking.clone()]),
            exclude: Vec::new(),
            order: 2,
        };
        let harry = OrderedStudent {
            name: "Harry".into(),
            preferences: VecDeque::new(),
            exclude: Vec::new(),
            order: 3,
        };

        let unplaced_students = vec![bert.clone(), kate.clone(), suze.clone(), harry.clone()];

        let mut placed: HashMap<String, Vec<OrderedStudent>> = HashMap::new();
        let mut not_placable: Vec<OrderedStudent> = Vec::new();

        place_students(unplaced_students, &mut placed, &mut not_placable);

        let mut assert_placed = HashMap::new();
        bert.preferences.remove(0);
        kate.preferences.remove(0);
        suze.preferences.remove(0);
        assert_placed.insert(cooking.name, vec![bert]);
        assert_placed.insert(walking.name, vec![kate, suze]);

        assert_eq!(placed, assert_placed);
        assert_eq!(not_placable, vec![harry]);
    }

    #[test]
    fn test_place_students_with_exclude() {
        let cooking = Category {
            name: "Cooking".into(),
            max_placements: 3,
        };
        let reading = Category {
            name: "Reading".into(),
            max_placements: 2,
        };

        let mut bert = OrderedStudent {
            name: "Bert".into(),
            preferences: VecDeque::from(vec![cooking.clone(), reading.clone()]),
            exclude: Vec::new(),
            order: 0,
        };
        let mut kate = OrderedStudent {
            name: "Kate".into(),
            preferences: VecDeque::from(vec![cooking.clone()]),
            exclude: Vec::from(vec![cooking.clone(), reading.clone()]),
            order: 1,
        };

        let unplaced_students = vec![bert.clone(), kate.clone()];

        let mut placed: HashMap<String, Vec<OrderedStudent>> = HashMap::new();
        let mut not_placable: Vec<OrderedStudent> = Vec::new();

        place_students(unplaced_students, &mut placed, &mut not_placable);

        let mut assert_placed = HashMap::new();
        bert.preferences.remove(0);
        kate.preferences.remove(0);
        assert_placed.insert(cooking.name, vec![bert]);

        assert_eq!(placed, assert_placed);
        assert_eq!(not_placable, vec![kate]);
    }

    #[test]
    fn test_truncate_categories() {
        let cooking = Category {
            name: "Cooking".into(),
            max_placements: 3,
        };
        let reading = Category {
            name: "Reading".into(),
            max_placements: 2,
        };
        let walking = Category {
            name: "Walking".into(),
            max_placements: 1,
        };

        let bert = OrderedStudent {
            name: "Bert".into(),
            preferences: VecDeque::from(vec![reading.clone(), walking.clone()]),
            exclude: Vec::new(),
            order: 0,
        };
        let kate = OrderedStudent {
            name: "Kate".into(),
            preferences: VecDeque::new(),
            exclude: Vec::new(),
            order: 1,
        };
        let suze = OrderedStudent {
            name: "Suze".into(),
            preferences: VecDeque::from(vec![cooking.clone()]),
            exclude: Vec::new(),
            order: 2,
        };
        let harry = OrderedStudent {
            name: "Harry".into(),
            preferences: VecDeque::from(vec![walking.clone()]),
            exclude: Vec::new(),
            order: 3,
        };

        let mut placed = HashMap::new();
        placed.insert(cooking.name.clone(), vec![bert]);
        placed.insert(
            walking.name.clone(),
            vec![kate, suze.clone(), harry.clone()],
        );

        let mut assert_placed = placed.clone();
        assert_placed.get_mut(&walking.name).unwrap().pop();
        assert_placed.get_mut(&walking.name).unwrap().pop();

        let categories: Vec<Category> = vec![cooking.clone(), reading.clone(), walking.clone()];

        let unplaced_students = truncate_categories(&mut placed, &categories);

        assert_eq!(placed, assert_placed);
        assert_eq!(unplaced_students, vec![suze, harry]);
    }

    #[test]
    fn test_assign_random() {
        let mut rng = StepRng::new(2, 0);

        let cooking = Category {
            name: "Cooking".into(),
            max_placements: 3,
        };
        let reading = Category {
            name: "Reading".into(),
            max_placements: 2,
        };
        let walking = Category {
            name: "Walking".into(),
            max_placements: 1,
        };

        let bert = OrderedStudent {
            name: "Bert".into(),
            preferences: VecDeque::from(vec![cooking.clone(), reading.clone(), walking.clone()]),
            exclude: Vec::new(),
            order: 0,
        };
        let kate = OrderedStudent {
            name: "Kate".into(),
            preferences: VecDeque::from(vec![walking.clone()]),
            exclude: Vec::new(),
            order: 1,
        };
        let suze = OrderedStudent {
            name: "Suze".into(),
            preferences: VecDeque::from(vec![walking.clone(), cooking.clone()]),
            exclude: Vec::new(),
            order: 2,
        };
        let harry = OrderedStudent {
            name: "Harry".into(),
            preferences: VecDeque::new(),
            exclude: Vec::new(),
            order: 3,
        };

        let not_placable: Vec<OrderedStudent> = vec![harry.clone()];

        let mut placed = HashMap::new();
        placed.insert(cooking.name.clone(), vec![bert]);
        placed.insert(walking.name.clone(), vec![kate, suze]);

        let mut assert_placed = placed.clone();
        assert_placed.get_mut(&cooking.name).unwrap().push(harry);

        let categories: Vec<Category> = vec![cooking.clone(), reading.clone(), walking.clone()];

        let not_placable = assign_random(not_placable, &mut placed, &categories, &mut rng);

        assert_eq!(not_placable, vec![]);
        assert_eq!(placed, assert_placed);
    }

    #[test]
    fn assign_random_full() {
        let mut rng = StepRng::new(2, 0);

        let cooking = Category {
            name: "Cooking".into(),
            max_placements: 1,
        };
        let reading = Category {
            name: "Reading".into(),
            max_placements: 1,
        };
        let walking = Category {
            name: "Walking".into(),
            max_placements: 2,
        };

        let bert = OrderedStudent {
            name: "Bert".into(),
            preferences: VecDeque::from(vec![cooking.clone(), reading.clone(), walking.clone()]),
            exclude: Vec::new(),
            order: 0,
        };
        let kate = OrderedStudent {
            name: "Kate".into(),
            preferences: VecDeque::from(vec![walking.clone()]),
            exclude: Vec::new(),
            order: 1,
        };
        let suze = OrderedStudent {
            name: "Suze".into(),
            preferences: VecDeque::from(vec![walking.clone(), cooking.clone()]),
            exclude: Vec::new(),
            order: 2,
        };
        let harry = OrderedStudent {
            name: "Harry".into(),
            preferences: VecDeque::new(),
            exclude: Vec::new(),
            order: 3,
        };
        let lisa = OrderedStudent {
            name: "Lisa".into(),
            preferences: VecDeque::new(),
            exclude: Vec::new(),
            order: 4,
        };

        let not_placable: Vec<OrderedStudent> = vec![harry.clone(), lisa.clone()];

        let mut placed = HashMap::new();
        placed.insert(cooking.name.clone(), vec![bert]);
        placed.insert(walking.name.clone(), vec![kate]);
        placed.insert(reading.name.clone(), vec![suze]);

        let mut assert_placed = placed.clone();
        assert_placed.get_mut(&walking.name).unwrap().push(harry);

        let categories: Vec<Category> = vec![cooking.clone(), reading.clone(), walking.clone()];

        let not_placable = assign_random(not_placable, &mut placed, &categories, &mut rng);

        // Lisa has the highest lot number, so should be not placable
        assert_eq!(not_placable, vec![lisa]);
        assert_eq!(placed, assert_placed);
    }

    #[test]
    fn test_assign_random_exclude() {
        let mut rng = StepRng::new(2, 0);

        let cooking = Category {
            name: "Cooking".into(),
            max_placements: 1,
        };
        let reading = Category {
            name: "Reading".into(),
            max_placements: 2,
        };

        let bert = OrderedStudent {
            name: "Bert".into(),
            preferences: VecDeque::from(vec![cooking.clone(), reading.clone()]),
            exclude: Vec::new(),
            order: 0,
        };
        let kate = OrderedStudent {
            name: "Kate".into(),
            preferences: VecDeque::new(),
            exclude: Vec::from(vec![reading.clone()]),
            order: 1,
        };
        let ludo = OrderedStudent {
            name: "Ludo".into(),
            preferences: VecDeque::new(),
            exclude: Vec::from(vec![reading.clone()]),
            order: 2,
        };

        let not_placable: Vec<OrderedStudent> = vec![kate.clone(), ludo.clone()];

        let mut placed = HashMap::new();
        placed.insert(cooking.name.clone(), vec![bert]);

        let assert_placed = placed.clone();

        let categories: Vec<Category> = vec![cooking.clone(), reading.clone()];

        let not_placable = assign_random(not_placable, &mut placed, &categories, &mut rng);

        assert_eq!(placed, assert_placed);
        assert_eq!(not_placable, vec![kate, ludo]);
    }
}
