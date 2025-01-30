use std::{
    sync::{LazyLock, Mutex, MutexGuard},
    thread::current,
};

use crate::{BaseInfo, CounterData, Info, InstantData, InstantScopeSize, ScopeData, TaggedTrace, TimePoint, Value};

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

static SCOPES: Buffer<ScopeData> = Buffer::init();
static COUNTERS: Buffer<CounterData> = Buffer::init();
static INSTANCES: Buffer<InstantData> = Buffer::init();

impl BaseInfo
{
    fn build(info: Info, start: TimePoint) -> Self
    {
        Self {
            thread_id: current().id(),
            info,
            start,
        }
    }

    fn build_now(info: Info) -> Self { Self::build(info, TimePoint::now()) }
}

pub fn record_custom_scope(info: Info, start: TimePoint, end: TimePoint)
{
    SCOPES.push(ScopeData {
        base: BaseInfo::build(info, start),
        end,
    });
}

pub fn record_custom_value<V: Into<Value>>(info: Info, value: V)
{
    let value = value.into();
    COUNTERS.push(CounterData {
        base: BaseInfo::build_now(info),
        value,
    });
}

pub fn record_custom_instant(info: Info, scope_size: InstantScopeSize)
{
    INSTANCES.push(InstantData {
        base: BaseInfo::build_now(info),
        scope_size,
    });
}

pub(super) fn flush_buffers() -> impl Iterator<Item = TaggedTrace>
{
    use TaggedTrace::*;
    SCOPES
        .flush()
        .into_iter()
        .map(|t| Scope(t))
        .chain(COUNTERS.flush().into_iter().map(|t| Counter(t)))
        .chain(INSTANCES.flush().into_iter().map(|t| Instant(t)))
}
