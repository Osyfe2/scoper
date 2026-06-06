use std::thread::{current, ThreadId};

use crate::{Info, TimePoint};

mod scopes;
mod value;

pub use scopes::Scope;
pub use value::Value;
pub(super) use scopes::Start;

pub(super) enum TaggedData
{
    Scope(Start),
    Counter(Value),
    Instant(InstantScopeSize),
}

impl From<Start> for TaggedData {
    fn from(value: Start) -> Self {
        Self::Scope(value)
    }
}

impl From<Value> for TaggedData {
    fn from(value: Value) -> Self {
        Self::Counter(value)
    }
}

impl From<InstantScopeSize> for TaggedData {
    fn from(value: InstantScopeSize) -> Self {
        Self::Instant(value)
    }
}

pub(super) struct Trace<Extra>(pub BaseInfo, pub Extra);
pub(super) type TaggedTrace = Trace<TaggedData>;

impl<Extra: Into<TaggedData>> Trace<Extra>
{
    pub(super) fn tag(self) -> TaggedTrace {
        Trace(self.0, self.1.into())
    }
}

pub(super) struct BaseInfo
{
    pub thread_id: ThreadId, //All trace types
    pub time_point: TimePoint,    //All trace types
    pub info: Info,          //static info
}

impl BaseInfo
{
    pub(crate) fn build(info: Info, time_point: TimePoint) -> Self
    {
        Self {
            thread_id: current().id(),
            info,
            time_point,
        }
    }

    pub(crate) fn build_now(info: Info) -> Self { Self::build(info, TimePoint::now()) }
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
    pub const fn code(self) -> char
    {
        match self
        {
            InstantScopeSize::Thread => 't',
            InstantScopeSize::Process => 'p',
            InstantScopeSize::Global => 'g',
        }
    }
}