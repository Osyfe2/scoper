#![feature(thread_id_value)]
#![feature(adt_const_params)]
#![warn(clippy::all, clippy::perf, clippy::pedantic)]

use std::num::NonZero;

mod global;
mod json;
mod macro_rules;
mod recordscope;
mod eventtypes;

mod types;

pub use recordscope::RecordScope;
pub use global::{record_custom_instant, record_custom_scope, record_custom_value};
pub use types::Scope;
pub use types::InstantScopeSize;
pub type Info = &'static TraceInfo<'static>;

pub mod macros
{
    pub use scoper_attr::record;

    pub use crate::{record_instant, record_scope, record_value};
}

pub struct TraceInfo<'a>
{
    pub name: &'a str,
    pub category: &'a str,
    pub header: &'a str, //(PID)
    pub args: &'a str,
}

use std::time::Instant as TimePoint;


type Pid = &'static str;
type Tid = NonZero<u64>;

#[derive(Debug)]
pub enum MetaTrace
{
    ProcessName(Pid, String), //__metadata M
    //ProcessSortIndex(Pid, usize),        //__metadata M todo!
    //ProcessLabels(Pid, String),        //__metadata M todo!
    ThreadName(Pid, Tid, String), /* __metadata M
                                   *ThreadSortIndex(Pid, Tid, usize),        //__metadata M todo!
                                   *ProcessUptimeSeconds(Pid, u128), //__metadata M Not in the doc
                                   *ActiveProcesses(Vec<Pid>, u128), //__metadata I s:g Not in the doc */
}

#[cfg(test)]
mod test
{
    use std::{path::Path, thread::sleep, time::Duration};

    use crate::{InstantScopeSize, RecordScope, macros::*, record_custom_value};
    fn wait_30_ms()
    {
        use crate::{Info, Scope, TraceInfo};
        static SCOPE_INFO: Info = &TraceInfo {
            //event_typ: EventType::Scope,
            name: "30 Millis",
            category: "inlinetest",
            header: "30 Millis",
            args: "",
        };

        let _profiling_scope = Scope::start(&SCOPE_INFO);
        let value = 0.8;
        record_custom_value(SCOPE_INFO, value);
        sleep(Duration::from_millis(30));
    }

    #[record]
    fn wait_30_ms_macro() { sleep(Duration::from_millis(30)); }

    #[test]
    fn basic_test_explicit_drop()
    {
        let record = RecordScope::start(Path::new("results/basic_test.json"));
        for _ in 0..10
        {
            wait_30_ms();
            sleep(Duration::from_millis(5));
        }
        drop(record);
    }

    #[test]
    fn implicit_test_explicit_drop()
    {
        let _record = RecordScope::start(Path::new("results/implicit_drop.json"));
        for _ in 0..10
        {
            wait_30_ms();
            sleep(Duration::from_millis(5));
        }
    }

    #[test]
    fn macro_test()
    {
        let _record = RecordScope::start(Path::new("results/macro_test.json"));
        for i in 0_i32..10
        {
            wait_30_ms_macro();
            sleep(Duration::from_millis(5));
            record_value!("", "test_value", 0.1 * f64::from(i + 1));
        }
    }

    #[test]
    fn threads_test()
    {
        let mut record = RecordScope::start(Path::new("results/threads_test.json"));
        record
            .add_meta_data("test".to_string(), &String::from("SomeExtraInfoHere"))
            .ok();

        {
            record_scope!("Some setup");
            wait_30_ms();
        }

        record.set_starting_time();
        
        std::thread::scope(|s| {
            s.spawn(|| {
                for _ in 0..10
                {
                    record_scope!("Myheader", "Thread C");
                    wait_30_ms_macro();
                    sleep(Duration::from_millis(5));
                }
            });
            s.spawn(|| {
                for _ in 0..10
                {
                    record_scope!("Thread D");
                    wait_30_ms();
                    sleep(Duration::from_millis(5));
                }
            });
        });
        record_instant!("First join", InstantScopeSize::Process);
        std::thread::scope(|s| {
            s.spawn(|| {
                for _ in 0..10
                {
                    record_scope!("Thread E");
                    wait_30_ms_macro();
                    sleep(Duration::from_millis(5));
                }
            });
        });
        record
            .add_meta_data("test".to_string(), &String::from("SomeExtraInfoHere"))
            .ok();
    }
}
