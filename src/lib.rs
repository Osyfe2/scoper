#![feature(thread_id_value)]
#![feature(adt_const_params)]
#![warn(clippy::all, clippy::perf, clippy::pedantic)]

use std::{num::NonZero, thread::ThreadId};
use eventtypes::EventType;
use slots::SlotIndex;

mod json;
mod slots;
mod recordscope;
mod eventtypes;

pub use recordscope::RecordScope;

pub(crate) use std::time::Instant as TimePoint;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Key(*const TraceInfo<'static>);

impl Key
{
    fn from_static(data: &'static TraceInfo) -> Self { 
        Self(std::ptr::from_ref::<TraceInfo>(data)) }

    fn read(self) -> &'static TraceInfo<'static> { unsafe { &*self.0 } }
}

unsafe impl Send for Key {}

pub mod macros
{
    pub use scoper_attr::record;

    pub use crate::record_scope;
    pub use crate::record_value;
    pub use crate::record_instant;
}

pub use global_scope::counter_event;
pub use global_scope::instant_event;

pub struct Scope
{
    level: SlotIndex, //pretty big but key requires align, could be smaller if additional data needed
    key: Key,
}

impl Scope
{
    #[must_use]
    pub fn start(data: &'static TraceInfo) -> Option<Self>
    {
        let level = global_scope::open_scope();
        Some(Self {
            key: Key::from_static(data),
            level,
        })
    }
}

impl Drop for Scope
{
    fn drop(&mut self) { 
        global_scope::close_scope(self); }
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
        match self {
            InstantScopeSize::Thread => 't',
            InstantScopeSize::Process => 'p',
            InstantScopeSize::Global => 'g',
        }
    }
}

union TraceAddition {
    end: TimePoint,
    int_float: (i64, f64),
    scope_size: InstantScopeSize,
}

struct Trace<const TYPE: EventType>
{
    thread_id: ThreadId, //All trace types 
    start: TimePoint, //All trace types 
    key: Key, //Key to static info
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
    ProcessName(Pid, String),        //__metadata M
    //ProcessSortIndex(Pid, usize),        //__metadata M todo!
    //ProcessLabels(Pid, String),        //__metadata M todo!
    ThreadName(Pid, Tid, String),    //__metadata M
    //ThreadSortIndex(Pid, Tid, usize),        //__metadata M todo!
    //ProcessUptimeSeconds(Pid, u128), //__metadata M Not in the doc
    //ActiveProcesses(Vec<Pid>, u128), //__metadata I s:g Not in the doc
}

mod global_scope
{
    use std::{
        cell::RefCell, sync::{LazyLock, Mutex, MutexGuard}, thread::{current, ThreadId}
    };

    use super::{EventType, Key, TimePoint, InstantScopeSize, Scope, Trace, TraceInfo, TraceAddition, slots::{Slots, SlotIndex}};

    type OpenScopes = Slots<TimePoint>;
    type Traces<const TYPE: EventType> = Vec<Trace<TYPE>>;

    thread_local! {
        static THREAD_ID: ThreadId = current().id();
        static OPEN_SCOPES: RefCell<OpenScopes> = RefCell::default();
    }
    static SCOPES: LazyLock<Mutex<Traces<{EventType::Scope}>>> = LazyLock::new(const { || Mutex::new(Traces::new()) });
    static COUNTERS: LazyLock<Mutex<Traces<{EventType::Counter}>>> = LazyLock::new(const { || Mutex::new(Traces::new()) });
    static INSTANCES: LazyLock<Mutex<Traces<{EventType::Instant}>>> = LazyLock::new(const { || Mutex::new(Traces::new()) });

    fn access_scopes<'a>() -> MutexGuard<'a, Traces<{EventType::Scope}>> { SCOPES.lock().expect("Could not get access") }
    fn access_counters<'a>() -> MutexGuard<'a, Traces<{EventType::Counter}>> { COUNTERS.lock().expect("Could not get access") }
    fn access_instances<'a>() -> MutexGuard<'a, Traces<{EventType::Instant}>> { INSTANCES.lock().expect("Could not get access") }

    pub(super) fn flush_traces() -> Traces<{EventType::Scope}> { std::mem::take(&mut access_scopes()) }
    pub(super) fn flush_counters() -> Traces< {EventType::Counter}> { std::mem::take(&mut access_counters()) }
    pub(super) fn flush_instances() -> Traces< {EventType::Instant}> { std::mem::take(&mut access_instances()) }

    pub(super) fn open_scope() -> SlotIndex
    {
        OPEN_SCOPES.with_borrow_mut(|slots| {
            slots.push(TimePoint::now())
        })
    }

    fn pop_scope_opening_time(level: SlotIndex) -> TimePoint
    {
        OPEN_SCOPES.with_borrow_mut(
            |slots|
            {
                slots.take(level)
            }
        )
        /*
        if let Some(mut x) = OPEN_SCOPES.with_borrow_mut(Vec::pop)
        {
        if key != x.0
            {
            OPEN_SCOPES.with_borrow_mut(|v| {
                if let Some(y) = v.iter_mut().rev().find(|s| s.0 == key)
                {
                std::mem::swap(&mut x, y);
                    }
                });
            }
            x.1
        }
        else
        {
            debug_assert!(false);
            Instant::now()
        }
        */
    }

    impl <const TYPE: EventType> Trace<TYPE>
    {
        fn build(info: &'static TraceInfo, addition: TraceAddition) -> Self
        {
            Self {
                thread_id: THREAD_ID.with(Clone::clone),
                key: Key::from_static(info),
                start: TimePoint::now(),
                addition,
            }
        }
    }

    pub(super) fn close_scope(Scope{key, level}: &Scope)
    {
        let start = pop_scope_opening_time(unsafe { level.duplicate() });
        let thread_id = THREAD_ID.with(Clone::clone);
        // Scopes always create EventType::Scope
        let addition = TraceAddition { end: TimePoint::now() };
        let trace = Trace {
            thread_id,
            key: *key,
            start,
            addition,
        };
        access_scopes().push(trace);
    }

    pub fn counter_event(info: &'static TraceInfo, int_float: (i64, f64))
    {
        let trace = Trace::build(info, TraceAddition{int_float});
        access_counters().push(trace);
    }

    pub fn instant_event(info: &'static TraceInfo, scope_size: InstantScopeSize)
    {
        access_instances().push(Trace::build(info, TraceAddition{scope_size}));
    }
}

pub mod macro_rules
{
    pub use const_format::str_replace;

    #[macro_export]
    macro_rules! record_scope {
        ($header: expr, $name: expr) => {
            static TRACE_SCOPE_INFO: $crate::TraceInfo = $crate::TraceInfo {
                name: $name,
                category: $crate::macro_rules::str_replace!(::std::module_path!(), "::", ","),
                header: $header,
                args: "",
            };
            #[allow(unused)]
            let _profiling_scope = $crate::Scope::start(&TRACE_SCOPE_INFO);
        };
        ($name: expr) => {
            record_scope!("", $name)
        };
    }

    #[macro_export]
    macro_rules! record_value {
        ($header: expr, $name: expr, $int: expr, $float: expr) => {
            {
                static TRACE_COUNTER_INFO: $crate::TraceInfo = $crate::TraceInfo {
                    name: $name,
                    category: $crate::macro_rules::str_replace!(::std::module_path!(), "::", ","),
                    header: $header,
                    args: "",
                };
                $crate::counter_event(&TRACE_COUNTER_INFO, ($int, $float));
            }
        };
        /*
        ($name: expr) => {
            record_value!("", $name)
        };
        */
    }

    #[macro_export]
    macro_rules! record_instant {
        ($header: expr, $name: expr, $scope_size: expr) => {
            {
                static TRACE_INSTANT_INFO: $crate::TraceInfo = $crate::TraceInfo {
                    name: $name,
                    category: $crate::macro_rules::str_replace!(::std::module_path!(), "::", ","),
                    header: $header,
                    args: "",
                };
                $crate::instant_event(&TRACE_INSTANT_INFO, $scope_size);
            }
        };
        ($name: expr, $scope_size: expr) => {
            record_instant!("", $name, $scope_size);
        };
        ($name: expr) => {
            record_instant!($name, $crate::InstantScopeSize::Process);
        };
    }
}

#[cfg(test)]
mod test
{
    use std::{path::Path, thread::sleep, time::Duration};

    use crate::{global_scope::counter_event, macros::*, RecordScope};
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
        counter_event(&SCOPE_INFO, (0, value));
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
        record.add_meta_data("test".to_string(), &String::from("SomeExtraInfoHere")).ok();
    }
}
