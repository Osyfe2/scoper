use std::time::Instant;

use scoper_base::*;

pub mod macros
{
    #[macro_export]
    macro_rules! record_scope
    {
        ($header: expr, $name: expr) => {};
        ($name: expr) => {};
    }

    #[macro_export]
    macro_rules! record_value
    {
        ($header: expr, $name: expr, $value: expr) =>{};
    }

    #[macro_export]
    macro_rules! record_instant
    {
        ($header: expr, $name: expr, $scope_size: expr) => {};
        ($name: expr, $scope_size: expr) => {};
        ($name: expr) => {};
    }

    pub use record_scope;
    pub use record_value;
    pub use record_instant;

    pub use scoper_attr::record;
}

//() for not constructable
pub struct Scope(());

impl Scope
{
    pub fn start(_info: Info) -> Self
    {
        Self(())
    }
}

impl Drop for Scope
{
    fn drop(&mut self) {}
}

pub fn record_custom_instant(_info: Info, _scope_size: InstantScopeSize) {}
pub fn record_custom_scope(_info: Info, _start: Instant, _end: Instant) {}
pub fn record_custom_value(_info: Info, _value: Value) {}
