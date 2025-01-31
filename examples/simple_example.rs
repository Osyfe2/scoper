use std::{path::Path, thread::sleep, time::Duration};

use scoper::{RecordScope, macros::*};

#[record]
fn do_some_work() { sleep(Duration::from_millis(30)); }

#[record]
fn other_work() { sleep(Duration::from_millis(5)); }

fn main()
{
    let _record = RecordScope::start(Path::new("results/simple_example.json"));
    for _ in 0..6
    {
        do_some_work();
        other_work();
    }
}