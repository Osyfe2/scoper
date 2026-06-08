#![feature(adt_const_params)]
#![warn(clippy::all, clippy::perf, clippy::pedantic)]

mod global;
mod json;
mod macro_rules;
mod record_scope;
mod event_types;
mod scopes;
mod types;

pub use record_scope::RecordScope;
pub use global::{record_custom_instant, record_custom_scope, record_custom_value};
pub use scopes::Scope;

pub mod macros
{
    pub use scoper_attr::record;

    pub use crate::{record_instant, record_scope, record_value};

    #[doc(hidden)]
    pub mod hidden_reexport
    {
        pub use const_format::str_replace;

    }

}

use std::time::Instant as TimePoint;
