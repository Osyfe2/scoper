use std::{
    sync::{LazyLock, Mutex, MutexGuard},
    thread::current,
};

use super::{EventType, Info, InstantScopeSize, TimePoint, Trace, TraceAddition};

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
    fn build(info: Info, start: TimePoint, addition: TraceAddition) -> Self
    {
        Self {
            thread_id: current().id(),
            info,
            start,
            addition,
        }
    }

    fn build_now(info: Info, addition: TraceAddition) -> Self { Self::build(info, TimePoint::now(), addition) }
}

pub fn record_custom_scope(info: Info, start: TimePoint, end: TimePoint)
{
    access_scopes().push(Trace::build(info, start, TraceAddition { end }));
}

pub fn record_custom_value(info: Info, int_float: (i64, f64))
{
    access_counters().push(Trace::build_now(info, TraceAddition { int_float }));
}

pub fn record_custom_instant(info: Info, scope_size: InstantScopeSize)
{
    access_instances().push(Trace::build_now(info, TraceAddition { scope_size }));
}
