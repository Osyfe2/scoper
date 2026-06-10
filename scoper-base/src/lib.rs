mod value;

pub use value::Value;

pub type Info = &'static TraceInfo<'static>;
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
    pub const fn new() -> Self
    {
        Self
        {
            name: "",
            category: "",
            header: "",
            args: "",
        }
    }

    pub const fn name(self, _name: &'a str) -> Self
    {
        self
    }

    pub const fn category(self, _category: &'a str) -> Self
    {
        self
    }

    pub const fn header(self, _header: &'a str) -> Self
    {
        self
    }

    pub const fn args(self, _args: &'a str) -> Self
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
