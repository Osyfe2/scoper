use std::time::Instant;

pub mod macros
{
    pub mod reexport
    {
        pub use const_format::str_replace;
    }

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

pub struct TraceInfo<'a>
{
    pub name: &'a str,
    pub category: &'a str,
    pub header: &'a str,
    pub args: &'a str,
}

impl<'a> TraceInfo<'a>
{
    #[allow(clippy::new_without_default)]
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

    pub fn name(self, _name: &'a str) -> Self
    {
        self
    }

    pub fn category(self, _category: &'a str) -> Self
    {
        self
    }

    pub fn header(self, _header: &'a str) -> Self
    {
        self
    }

    pub fn args(self, _args: &'a str) -> Self
    {
        self
    }
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
    pub const fn code(self) -> char
    {
        0u8 as char
    }
}

pub fn record_custom_instant(_info: Info, _scope_size: InstantScopeSize) {}
pub fn record_custom_scope(_info: Info, _start: Instant, _end: Instant) {}
pub fn record_custom_value<V: Into<Value>>(_info: Info, _value: V) {}

pub type Info = &'static TraceInfo<'static>;

//somehow does not appear in the scoper_impl doc but is part of the public API via `record_custom_value`
pub enum Value
{
    UInt(u64),
    IInt(i64),
    Float(f64),
}
