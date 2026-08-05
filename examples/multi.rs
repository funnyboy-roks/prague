use std::{thread::JoinHandle, time::Duration};

use prog::{ProgressGroup, ProgressStyle};

fn main() {
    let mut threads = Vec::new();
    let group = ProgressGroup::builder()
        .style(ProgressStyle::builder())
        .build();

    let mut bar = |label: &'static str, (start, end): (u64, u64), time_ms: u64, delayed_ms: u64| {
        let mut prog = group
            .progress_builder()
            .label(label)
            .max(end)
            .init(start)
            .build()
            .unwrap();
        threads.push(std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(delayed_ms));
            for current in start..=end {
                prog.update(current);
                std::thread::sleep(Duration::from_millis(time_ms / (end - start)));
            }
        }));
    };

    bar("My progress bar", (0, 1000), 1000, 0);
    bar("My second progress bar", (0, 2000), 5000, 0);
    bar("My third progress bar", (0, 10), 2000, 1000);
    bar(
        "My very slow progress bar with a long name",
        (0, 10),
        5000,
        500,
    );

    threads.into_iter().try_for_each(JoinHandle::join).unwrap();

    drop(group);
}
