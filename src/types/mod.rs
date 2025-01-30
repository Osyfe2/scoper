use std::thread::{current, ThreadId};

use crate::{Info, TimePoint};

mod scopes;
mod value;

pub use scopes::Scope;
pub use value::Value;
pub(super) use scopes::ScopeData;
pub(super) use value::CounterData;

pub(super) enum TaggedTrace
{
    Scope(ScopeData),
    Counter(CounterData),
    Instant(InstantData),
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

pub(crate) struct InstantData
{
    pub(crate) base: BaseInfo,
    pub(crate) scope_size: InstantScopeSize,
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