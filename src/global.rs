

use std::{
        cell::RefCell,
        sync::{LazyLock, Mutex, MutexGuard},
        thread::{ThreadId, current},
    };

    use super::{
        EventType, InstantScopeSize, Key, Scope, TimePoint, Trace, TraceAddition, TraceInfo,
    };

    type OpenScopes = Vec<TimePoint>;
    
    thread_local! {
        static THREAD_ID: ThreadId = current().id();
        static OPEN_SCOPES: RefCell<OpenScopes> = RefCell::default();
    }
    pub(super) fn open_scope() { OPEN_SCOPES.with_borrow_mut(|slots| slots.push(TimePoint::now())) }
    fn pop_scope_opening_time() -> TimePoint
    {
        OPEN_SCOPES.with_borrow_mut(|slots| slots.pop().unwrap())
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

    pub(super) fn close_scope(Scope { key }: &Scope)
    {
        let start = pop_scope_opening_time();
        record_custom_scope(key.read(), start, TimePoint::now());
    }


    type Traces<const TYPE: EventType> = Vec<Trace<TYPE>>;
    static SCOPES: LazyLock<Mutex<Traces<{ EventType::Scope }>>> = LazyLock::new(const { || Mutex::new(Traces::new()) });
    static COUNTERS: LazyLock<Mutex<Traces<{ EventType::Counter }>>> = LazyLock::new(const { || Mutex::new(Traces::new()) });
    static INSTANCES: LazyLock<Mutex<Traces<{ EventType::Instant }>>> = LazyLock::new(const { || Mutex::new(Traces::new()) });

    fn access_scopes<'a>() -> MutexGuard<'a, Traces<{ EventType::Scope }>> { SCOPES.lock().expect("Could not get access") }
    fn access_counters<'a>() -> MutexGuard<'a, Traces<{ EventType::Counter }>> { COUNTERS.lock().expect("Could not get access") }
    fn access_instances<'a>() -> MutexGuard<'a, Traces<{ EventType::Instant }>> { INSTANCES.lock().expect("Could not get access") }

    pub(super) fn flush_traces() -> Traces<{ EventType::Scope }> { std::mem::take(&mut access_scopes()) }
    pub(super) fn flush_counters() -> Traces<{ EventType::Counter }> { std::mem::take(&mut access_counters()) }
    pub(super) fn flush_instances() -> Traces<{ EventType::Instant }> { std::mem::take(&mut access_instances()) }

    impl<const TYPE: EventType> Trace<TYPE>
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

    pub fn record_custom_scope(info: &'static TraceInfo, start: TimePoint, end: TimePoint)
    {
        //let start = pop_scope_opening_time();
        let addition = TraceAddition { end };
        let trace = Trace {
            thread_id: THREAD_ID.with(Clone::clone),
            key: Key::from_static(info),
            start,
            addition,
        };
        access_scopes().push(trace);
    }

    pub fn record_custom_value(info: &'static TraceInfo, int_float: (i64, f64))
    {
        let trace = Trace::build(info, TraceAddition { int_float });
        access_counters().push(trace);
    }

    pub fn record_custom_instant(info: &'static TraceInfo, scope_size: InstantScopeSize)
    {
        access_instances().push(Trace::build(info, TraceAddition { scope_size }));
    }