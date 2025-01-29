#![feature(thread_id_value)]
#![feature(adt_const_params)]
#![warn(clippy::all, clippy::perf, clippy::pedantic)]

use std::{num::NonZero, thread::ThreadId};

use eventtypes::EventType;
use slots::SlotIndex;

mod eventtypes;
mod json;
mod macro_rules;
mod recordscope;
mod slots;
mod global;

pub use recordscope::RecordScope;
pub use global::{record_custom_instant, record_custom_value};

pub mod macros
{
    pub use scoper_attr::record;

    pub use crate::{record_instant, record_scope, record_value};
}

pub(crate) use std::time::Instant as TimePoint;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Key(*const TraceInfo<'static>);

impl Key
{
    fn from_static(data: &'static TraceInfo) -> Self { Self(std::ptr::from_ref::<TraceInfo>(data)) }

    fn read(self) -> &'static TraceInfo<'static> { unsafe { &*self.0 } }
}

unsafe impl Send for Key {}

pub struct Scope
{
    level: SlotIndex, //pretty big but key requires align, could be smaller if additional data needed
    key: Key,
}

impl Scope
{
    #[must_use]
    pub fn start(data: &'static TraceInfo) -> Self
    {
        let level = global::open_scope();
        Self {
            key: Key::from_static(data),
            level,
        }
    }
}

impl Drop for Scope
{
    fn drop(&mut self) { global::close_scope(self); }
}

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

union TraceAddition
{
    end: TimePoint,
    int_float: (i64, f64),
    scope_size: InstantScopeSize,
}

struct Trace<const TYPE: EventType>
{
    thread_id: ThreadId,     //All trace types
    start: TimePoint,        //All trace types
    key: Key,                //Key to static info
    addition: TraceAddition, //non static data
}

pub struct TraceInfo<'a>
{
    //pub event_typ: EventType,
    pub name: &'a str,
    pub category: &'a str,
    pub header: &'a str, //(PID)
    pub args: &'a str,
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
        use crate::{Scope, TraceInfo};
        static SCOPE_INFO: TraceInfo = TraceInfo {
            //event_typ: EventType::Scope,
            name: "30 Millis",
            category: "inlinetest",
            header: "30 Millis",
            args: "",
        };

        let _profiling_scope = Scope::start(&SCOPE_INFO);
        let value = 0.8;
        record_custom_value(&SCOPE_INFO, (0, value));
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
