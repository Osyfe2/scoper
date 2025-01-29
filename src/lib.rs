#![feature(thread_id_value)]
#![feature(adt_const_params)]
#![warn(clippy::all, clippy::perf, clippy::pedantic)]

use std::{num::NonZero, thread::ThreadId};

use eventtypes::EventType;

mod eventtypes;
mod json;
mod macro_rules;
mod recordscope;
mod global;
mod scopes;

pub use recordscope::RecordScope;
pub use scopes::Scope;
pub use global::{record_custom_instant, record_custom_value, record_custom_scope};

pub mod macros
{
    pub use scoper_attr::record;

    pub use crate::{record_instant, record_scope, record_value};
}

pub(crate) use std::time::Instant as TimePoint;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum InstantScopeSize
{
    Thread,
    Process,
    Global,
}

impl InstantScopeSize
{
    #[must_use]
    pub fn code(self) -> char
    {
        match self
        {
            InstantScopeSize::Thread => 't',
            InstantScopeSize::Process => 'p',
            InstantScopeSize::Global => 'g',
        }
    }
}

struct Trace<const TYPE: EventType>
{
    thread_id: ThreadId,     //All trace types
    start: TimePoint,        //All trace types
    info: Info,              //static info
    addition: TraceAddition, //non static data
}

pub type Info = &'static TraceInfo<'static>;

pub struct TraceInfo<'a>
{
    //pub event_typ: EventType,
    pub name: &'a str,
    pub category: &'a str,
    pub header: &'a str, //(PID)
    pub args: &'a str,
}

union TraceAddition
{
    end: TimePoint,
    int_float: (i64, f64),
    scope_size: InstantScopeSize,
}

type Pid = &'static str;
type Tid = NonZero<u64>;

#[derive(Debug)]
pub enum MetaTrace
{
    ProcessName(Pid, String), //__metadata M
    //ProcessSortIndex(Pid, usize),        //__metadata M todo!
    //ProcessLabels(Pid, String),        //__metadata M todo!
    ThreadName(Pid, Tid, String), /* M */
                                  /*ThreadSortIndex(Pid, Tid, usize),        //__metadata M todo!
                                   *ProcessUptimeSeconds(Pid, u128), //__metadata M Not in the doc
                                   *ActiveProcesses(Vec<Pid>, u128), //__metadata I s:g Not in the doc */
}

#[cfg(test)]
mod test
{
    use std::{path::Path, thread::sleep, time::Duration};

    use crate::{macros::*, record_custom_value, RecordScope};
    fn wait_30_ms()
    {
        use crate::{Scope, TraceInfo, Info};
        static SCOPE_INFO: Info = &TraceInfo {
            //event_typ: EventType::Scope,
            name: "30 Millis",
            category: "inlinetest",
            header: "30 Millis",
            args: "",
        };

        let _profiling_scope = Scope::start(&SCOPE_INFO);
        let value = 0.8;
        record_custom_value(SCOPE_INFO, (0, value));
        sleep(Duration::from_millis(30));
    }

    #[record]
    fn wait_30_ms_macro() { sleep(Duration::from_millis(30)); }

    #[test]
    fn basic_test_explicit_drop()
    {
        let record = RecordScope::start(Path::new("../results/basic_test.json"));
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
        let _record = RecordScope::start(Path::new("../results/implicit_drop.json"));
        for _ in 0..10
        {
            wait_30_ms();
            sleep(Duration::from_millis(5));
        }
    }

    #[test]
    fn macro_test()
    {
        let _record = RecordScope::start(Path::new("../results/macro_test.json"));
        for i in 0_i32..10
        {
            wait_30_ms_macro();
            sleep(Duration::from_millis(5));
            record_value!("", "test_value", i.into(), 0.1 * f64::from(i + 1));
        }
    }

    #[test]
    fn threads_test()
    {
        let mut record = RecordScope::start(Path::new("../results/threads_test.json"));
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
        record_instant!("First join");
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
