pub mod statistics;
use std::{self, error::Error, io};

// CSV record structure
#[derive(Debug, serde::Deserialize)]
pub struct Record {
    user_id: statistics::User,
    task_id: statistics::Task,
    work_duration: statistics::TimeAmount,
    date: String,
    comment: String,
}

// process CSV record
fn process_stats_record(user_stats: &mut statistics::UserStats, tasks_stats: &mut statistics::TaskStats, record: Record) {
    // accumulate task stats for a user
    user_stats
        .entry(record.user_id)
        .and_modify(|entry| entry.add_time(record.task_id, record.work_duration))
        .or_insert(statistics::TaskStats::from_data(record.task_id, record.work_duration));

    // accumulate global task stats
    tasks_stats.add_time(record.task_id, record.work_duration);
}

pub fn solve(user_stats: &mut statistics::UserStats, task_stats: &mut statistics::TaskStats) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result?;
        process_stats_record(user_stats, task_stats, record);
    }
    Ok(())
}
