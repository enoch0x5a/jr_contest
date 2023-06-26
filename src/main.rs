// Read the file, calculate values, and print the result to stdout:

// - How much time did each user spent on each task?: struct UserStats
// - The top 10 tasks with the most extended duration: struct TaskStats
//
// UserStats - structure for per-User statistics on time spend on each of his Tasks (TaskTime)
// UserStats:
// User ->* TaskTime
//
// User: i64
//
// TaskTime:
// Task -> TimeAmount
//
// Task: i64
// TimeAmount: i64

// TaskStats - structure for per-Task statistics on time spent on each Task
// Task -> Time amount

// user ->* task -> time amount
// User {
//   id
//   task_list: [ id -> task ]
// }

// Task {
//   task_id
//   time_spent
// }

use jr_csv::{self, statistics};
use std::process;

fn main() {
    let mut user_stats = statistics::UserStats::default();
    let mut top_10_task_stats = statistics::TaskStats::new();
    if let Err(err) = jr_csv::solve(&mut user_stats, &mut top_10_task_stats) {
        println!("error running example: {}", err);
        process::exit(1);
    }

    for (id, time) in top_10_task_stats.top_10() {
        println!("Task #{} => {}", id, time);
    }

    println!("");

    for (user, stats) in user_stats.iter() {
        println!("User #{user}");
        for (id, time) in stats.iter() {
            println!(" Task #{} => {}", id, time);
        }
    }
}
