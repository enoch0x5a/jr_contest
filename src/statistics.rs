use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::fmt;
use std::hash::BuildHasherDefault;

use hashers::fx_hash::FxHasher;

pub type User = i64;
pub type Task = i64;
pub type TimeAmount = i64;
pub type UserStats = HashMap<User, TaskStats, BuildHasherDefault<FxHasher>>;

#[derive(Debug)]
pub struct TaskStats {
    storage: HashMap<Task, TimeAmount, BuildHasherDefault<FxHasher>>,
}

impl TaskStats {
    pub fn new() -> Self {
        let map = HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default());
        Self {
            storage: map,
        }
    }

    pub fn from_data(task: Task, duration: TimeAmount) -> Self {
        let mut map = HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default());
        map.insert(task, duration);

        Self {
            storage: map,
        }
    }

    // Add time to existing Task record in storage or create new record with given duration
    pub fn add_time(&mut self, task: Task, time: TimeAmount) {
        self.storage
            .entry(task)
            .and_modify(|entry| *entry += time)
            .or_insert(time);
    }

    // returns a vector of top 10 tasks sorted by time spent
    pub fn top_10(&self) -> Vec<(Task, TimeAmount)> {
        let mut top_10: Vec<(Task, TimeAmount)> = Vec::with_capacity(self.storage.len());
        for (task, amount) in self.storage.iter() {
            top_10.push((*task, *amount));
            top_10.sort_unstable_by(|a, b| b.1.cmp(&a.1))
        }
        top_10.truncate(10);
        top_10.shrink_to_fit();
        top_10
    }

    pub fn iter(&self) -> Iter<Task, TimeAmount> {
        self.storage.iter()
    }
}

// impl fmt::Display for UserStats {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for (k,v) in self.storage.iter() {
//             write!(f, "{} {}", k, v).unwrap();
//         }
//         Result::Ok(())
//     }
// }
