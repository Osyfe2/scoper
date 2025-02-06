use std::sync::{LazyLock, Mutex, MutexGuard};

use crate::{
    Info, TimePoint,
    types::{BaseInfo, InstantScopeSize, Start, TaggedTrace, Trace, Value},
};

pub fn record_custom_scope(info: Info, start: TimePoint, end: TimePoint)
{
    SCOPES.push(Trace(BaseInfo::build(info, end), Start(start)));
}

pub fn record_custom_value<V: Into<Value>>(info: Info, value: V)
{
    let value = value.into();
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

static SCOPES: Buffer<Trace<Start>> = Buffer::init();
static COUNTERS: Buffer<Trace<Value>> = Buffer::init();
static INSTANCES: Buffer<Trace<InstantScopeSize>> = Buffer::init();

struct Buffer<Data>
{
    buffer: LazyLock<Mutex<Vec<Data>>>,
}

impl<Data> Buffer<Data>
{
    const fn init() -> Self
    {
        Self {
            buffer: LazyLock::new(const { || Mutex::new(Vec::new()) }),
        }
    }

    fn access<'a>(&'a self) -> MutexGuard<'a, Vec<Data>> { self.buffer.lock().expect("Could not get access") }

    fn push(&self, value: Data) { self.access().push(value); }

    fn flush(&self) -> Vec<Data> { std::mem::take(&mut self.access()) }
}
