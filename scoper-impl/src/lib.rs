#![feature(thread_id_value)]
#![feature(adt_const_params)]
#![warn(clippy::all, clippy::perf, clippy::pedantic)]

mod global;
mod json;
mod macro_rules;
mod record_scope;
mod event_types;
mod types;

pub use record_scope::RecordScope;
pub use global::{record_custom_instant, record_custom_scope, record_custom_value};
pub use types::Scope;
pub use types::InstantScopeSize;
pub type Info = &'static TraceInfo<'static>;

pub mod macros
{
    pub use scoper_attr::record;

    pub use crate::{record_instant, record_scope, record_value};

    pub mod reexport
    {
        pub use const_format::str_replace;

    }

}

pub struct TraceInfo<'a>
{
    pub name: &'a str,
    pub category: &'a str,
    pub header: &'a str, //(PID)
    pub args: &'a str,
}

impl<'a> TraceInfo<'a>
{
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self
    {
        Self
        {
            name: "",
            category: "",
            header: "",
            args: "",
        }
    }

    #[must_use]
    pub fn name(mut self, name: &'a str) -> Self
    {
        self.name = name;
        self
    }

    #[must_use]
    pub fn category(mut self, category: &'a str) -> Self
    {
        self.category = category;
        self
    }

    #[must_use]
    pub fn header(mut self, header: &'a str) -> Self
    {
        self.header = header;
        self
    }

    #[must_use]
    pub fn args(mut self, args: &'a str) -> Self
    {
        self.args = args;
        self
    }
}

use std::time::Instant as TimePoint;
