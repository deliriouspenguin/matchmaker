use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

pub mod da_stb;

#[derive(Debug, Eq, Clone, Deserialize, Serialize)]
pub struct Student {
    pub name: String,
    pub preferences: VecDeque<Category>,
    pub exclude: Vec<Category>,
}

impl Student {
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

#[derive(Clone, Deserialize, Serialize)]
pub struct Category {
    pub name: String,
    pub max_placements: usize,
}

impl Category {
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MatchResult {
    pub placed: HashMap<String, Vec<Student>>,
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
