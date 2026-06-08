use std::sync::{LazyLock, Mutex, MutexGuard};

use scoper_base::*;

use crate::{
    TimePoint, scopes::Start, types::{BaseInfo, TaggedTrace, Trace}
};

pub fn record_custom_scope(info: Info, start: TimePoint, end: TimePoint)
{
    SCOPES.push(Trace(BaseInfo::build(info, end), Start(start)));
}

pub fn record_custom_value(info: Info, value: Value)
{
    COUNTERS.push(Trace(BaseInfo::build_now(info), value));
}

pub fn record_custom_instant(info: Info, scope_size: InstantScopeSize) { INSTANCES.push(Trace(BaseInfo::build_now(info), scope_size)); }

pub(super) fn flush_buffers() -> impl Iterator<Item = TaggedTrace>
{
    SCOPES
        .flush()
        .into_iter()
        .map(Trace::tag)
        .chain(COUNTERS.flush().into_iter().map(Trace::tag))
        .chain(INSTANCES.flush().into_iter().map(Trace::tag))
}

static SCOPES: Buffer<Trace<Start>> = Buffer::init::<30_000>();
static COUNTERS: Buffer<Trace<Value>> = Buffer::init::<1024>();
static INSTANCES: Buffer<Trace<InstantScopeSize>> = Buffer::init::<128>();

struct Buffer<Data>
{
    buffer: LazyLock<Mutex<Vec<Data>>>,
}

impl<Data> Buffer<Data>
{
    const fn init<const CAPACITY: usize>() -> Self
    {
        Self {
            buffer: LazyLock::new(const { || Mutex::new(Vec::with_capacity(CAPACITY)) }),
        }
    }

    fn access(&self) -> MutexGuard<'_, Vec<Data>> { self.buffer.lock().expect("Could not get access") }

    fn push(&self, value: Data) { self.access().push(value); }

    fn flush(&self) -> Vec<Data> { std::mem::take(&mut self.access()) }
}
