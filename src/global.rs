use std::{
    sync::{LazyLock, Mutex, MutexGuard},
    thread::current,
};

use super::{Value, EventType, Info, InstantScopeSize, TimePoint, Trace, TraceAddition};

type Traces<const TYPE: EventType> = Vec<Trace<TYPE>>;

struct Buffer<const TYPE: EventType>
{
    buffer: LazyLock<Mutex<Traces<{ TYPE }>>>
}

impl <const TYPE: EventType> Buffer<TYPE>
{
    const fn init() -> Self
    {
        Self{
            buffer: LazyLock::new(const { || Mutex::new(Traces::new()) })
        }
    }

    fn access<'a>(&'a self) -> MutexGuard<'a, Traces<{ TYPE }>>{
        self.buffer.lock().expect("Could not get access") 
    }

    fn push(&self, trace: Trace<{TYPE}>)
    {
        self.access().push(trace);
    }

    pub(super) fn flush(&self) -> Traces<{ TYPE }> { std::mem::take(&mut self.access()) }
}

static SCOPES: Buffer<{EventType::Scope}> = Buffer::init();
static COUNTERS: Buffer<{EventType::Counter}> = Buffer::init();
static INSTANCES: Buffer<{EventType::Instant}> = Buffer::init();

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
    SCOPES.push(Trace::build(info, start, TraceAddition { end }));
}

pub fn record_custom_value<V: Into<Value>>(info: Info, value: V)
{
    let value = value.into();
    COUNTERS.push(Trace::build_now(info, TraceAddition { value }));
}

pub fn record_custom_instant(info: Info, scope_size: InstantScopeSize)
{
    INSTANCES.push(Trace::build_now(info, TraceAddition { scope_size }));
}

pub(super) enum TaggedTrace
{
    Scope(Trace<{EventType::Scope}>),
    Counter(Trace<{EventType::Counter}>),
    Instant(Trace<{EventType::Instant}>),
}

pub(super) fn flush_buffers() -> impl Iterator<Item = TaggedTrace>
{
    use TaggedTrace::*;
    SCOPES.flush().into_iter().map(|t| Scope(t))
    .chain(COUNTERS.flush().into_iter().map(|t| Counter(t)))
    .chain(INSTANCES.flush().into_iter().map(|t| Instant(t)))
}