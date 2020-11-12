//! This library can be used to fairly match students to categories (or activities). Or students to schools. Or anything to anything else.
//!
//! # Background
//!
//! The matching problem this library solves is mathematically know as the residence problem and is a subset of the [stable marriage problem](https://en.wikipedia.org/wiki/Stable_marriage_problem).
//! There are sereval known algorithms that solve this problem, each with there own pros and cons. For more information about the subject see also [Matching algorithms for the secondary school admission problem in Amsterdam](https://staff.fnwi.uva.nl/b.bredeweg/pdf/BSc/20152016/Klijnsma.pdf).
//!
//! # Algorithm
//!
//! At this time this library only implements the `Deferred Acceptance - Single Tie Break` algorithm. The library has been designed to make the implementation of other algorithms possible (it just needs to be done ;).
//!
//! # Usage
//!
//! ## Default matching
//!
//! Students are distributed over multiple categories, but each student can only be placed once.
//!
//! ```
//! use matchmaker::da_stb::match_students;
//! use matchmaker::{Category, Student};
//! use rand::thread_rng;
//! use std::collections::VecDeque;
//!
//! // Create categories
//! let cooking = Category::new("Cooking", 10);
//! let reading = Category::new("Reading", 10);
//! let walking = Category::new("Walking", 5);
//!
//! // Create student Bert
//! // Bert wishes to be placed in category cooking or reading (in that order)
//! let bert = Student::new(
//!     "Bert",
//!     VecDeque::from(vec![cooking.clone(), reading.clone()]),
//!     Vec::new(),
//! );
//!
//! // Create student Suze
//! // Suze wishes to be placed in category cooking or reading (in that order),
//! // but does not wish to be placed in category walking
//! let suze = Student::new(
//!     "Suze",
//!     VecDeque::from(vec![reading.clone(), cooking.clone()]),
//!     Vec::from([walking.clone()]),
//! );
//!
//! let mut rng = thread_rng();
//! let categories = Vec::from([cooking, reading, walking]);
//!
//! let match_result = match_students(Vec::from([bert, suze]), &categories, &mut rng);
//!
//! println!("Students matched to categories:");
//! println!();
//! for category in &categories {
//!     println!("{}:", &category.name);
//!     for student in match_result
//!         .placed
//!         .get(&category.name)
//!         .unwrap_or(&Vec::new())
//!     {
//!         println!(" - {}", &student.name);
//!     }
//! }
//!
//! if match_result.not_placable.is_empty() {
//!     println!();
//!     println!("All students could be placed.");
//! }
//! ```
//!
//! This should the following result:
//!
//! ```text
//! Students matched to categories:
//!
//! Cooking:
//!  - Bert
//! Reading:
//!  - Suze
//! Walking:
//!
//! All students could be placed.
//! ```
//!
//! ## Place students in multiple categories
//!
//! Students are distributed over multiple categories. A single student can be placed
//! in more than one category.
//!
//! # Example
//!
//! ```
//! use matchmaker::{Category, Student};
//! use matchmaker::da_stb::match_students_to_multiple_categories;
//! use rand::thread_rng;
//! use std::collections::VecDeque;
//!
//! // Create categories
//! let cooking = Category::new("Cooking", 10);
//! let reading = Category::new("Reading", 10);
//! let walking = Category::new("Walking", 5);
//!
//! // Create student Bert
//! // Bert wishes to be placed in category cooking or reading (in that order)
//! let bert = Student::new(
//!     "Bert",
//!     VecDeque::from(vec![cooking.clone(), reading.clone()]),
//!     Vec::new(),
//! );
//!
//! // Create student Suze
//! // Suze wishes to be placed in category cooking or reading (in that order),
//! // but does not wish to be placed in category walking
//! let suze = Student::new(
//!     "Suze",
//!     VecDeque::from(vec![cooking.clone(), reading.clone()]),
//!     Vec::from([walking.clone()]),
//! );
//!
//! let mut rng = thread_rng();
//! let categories = Vec::from([cooking, reading, walking]);
//!
//! let match_result = match_students_to_multiple_categories(
//!     Vec::from([bert, suze]),
//!     &categories,
//!     &mut rng);
//!
//! println!("Students matched to categories:");
//! println!();
//! for category in &categories {
//!     println!("{}:", &category.name);
//!     for student in match_result
//!         .placed
//!         .get(&category.name)
//!         .unwrap_or(&Vec::new())
//!     {
//!         println!(" - {}", &student.name);
//!     }
//! }
//!
//! if match_result.not_placable.is_empty() {
//!     println!();
//!     println!("All students could be placed.");
//! }
//! ```
//!
//! This should the following result:
//!
//! ```text
//! Students matched to categories:
//!
//! Cooking:
//!  - Suze
//!  - Bert
//! Reading:
//!  - Bert
//!  - Suze
//! Walking:
//!  - Bert
//!
//! All students could be placed.
//! ```
use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

pub mod da_stb;

/// Holds a student
#[derive(Debug, Eq, Clone, Deserialize, Serialize)]
pub struct Student {
    /// Name of the student (must be unique)
    pub name: String,
    /// Categories the student wishes to be placed in, in order of preference
    pub preferences: VecDeque<Category>,
    /// Categories the student wishes *not* to be placed in
    pub exclude: Vec<Category>,
}

impl Student {
    /// Return a new Student
    ///
    /// # Arguments
    ///
    /// * `name` - A &`str` that holds the name of the student (must be unique)
    /// * `preferences` - A `VecDeque` of [`Category`]s the student wishes to be placed in, in order of preference
    /// * `exclude` - A `Vec` of [`Category`]s the student wishes *not* to be placed in
    ///
    /// [`Category`]: struct.Category.html
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::VecDeque;
    /// use matchmaker::{Category, Student};
    ///
    /// // Create categories
    /// let cooking = Category::new("Cooking", 10);
    /// let reading = Category::new("Reading", 10);
    ///
    /// // Create student Bert
    /// // Bert wishes to be placed in category cooking or reading (in that order)
    /// let bert = Student::new(
    ///     "Bert",
    ///     VecDeque::from(vec![cooking, reading]),
    ///     Vec::new(),
    /// );
    /// ```
    ///
    /// ```
    /// use std::collections::VecDeque;
    /// use matchmaker::{Category, Student};
    ///
    /// let cooking = Category::new("Cooking", 10);
    /// let reading = Category::new("Reading", 10);
    /// let walking = Category::new("Walking", 5);
    ///
    /// // Create student Suze
    /// // Suze wishes to be placed in category cooking or reading (in that order),
    /// // but does not wish to be placed in category walking
    /// let suze = Student::new(
    ///     "Suze",
    ///     VecDeque::from(vec![cooking, reading]),
    ///     Vec::from([walking]),
    /// );
    /// ```
    pub fn new(name: &str, preferences: VecDeque<Category>, exclude: Vec<Category>) -> Self {
        Student {
            name: name.into(),
            preferences,
            exclude,
        }
    }
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl From<OrderedStudent> for Student {
    fn from(os: OrderedStudent) -> Self {
        Student {
            name: os.name,
            preferences: os.preferences,
            exclude: os.exclude,
        }
    }
}

impl Ord for Student {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct OrderedStudent {
    name: String,
    preferences: VecDeque<Category>,
    exclude: Vec<Category>,
    order: usize,
}

impl Ord for OrderedStudent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialOrd for OrderedStudent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Holds a category
#[derive(Clone, Deserialize, Serialize)]
pub struct Category {
    /// Name of the category (must be unique)
    pub name: String,
    /// Maximum number of students that can be placed in category this category
    pub max_placements: usize,
}

impl Category {
    /// Return a new `Category`
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the category (must be unique)
    /// * `max_placements` - Maximum number of students that can be placed in category this category
    ///
    /// # Example
    ///
    /// ```
    /// use matchmaker::Category;
    ///
    /// // Category cooking with capacity of 10 placements
    /// let cooking = Category::new("Cooking", 10);
    ///
    /// // Category reading with capacity of 5 placements
    /// let reading = Category::new("Reading", 5);
    /// ```
    pub fn new(name: &str, max_placements: usize) -> Self {
        Category {
            name: name.into(),
            max_placements,
        }
    }
}

impl std::hash::Hash for Category {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Category {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Category {}

impl Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} ({})", self.name, self.max_placements))
    }
}

/// Holds the result of a match
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MatchResult {
    /// Hashmap containing a list of placed students per category name
    pub placed: HashMap<String, Vec<Student>>,
    /// List of students that could not be placed in any category
    pub not_placable: Vec<Student>,
}

impl MatchResult {
    fn from(
        mut placed: HashMap<String, Vec<OrderedStudent>>,
        not_placable: Vec<OrderedStudent>,
    ) -> Self {
        let mut new_placed = HashMap::with_capacity(placed.capacity());
        let mut new_not_placable = Vec::with_capacity(not_placable.capacity());

        for (key, value) in placed.iter_mut() {
            let ordered_students = std::mem::replace(value, Vec::new());
            let students: Vec<Student> = ordered_students.into_iter().map(|os| os.into()).collect();
            new_placed.insert(key.clone(), students);
        }

        for np in not_placable.into_iter() {
            new_not_placable.push(np.into());
        }

        MatchResult {
            placed: new_placed,
            not_placable: new_not_placable,
        }
    }
}
