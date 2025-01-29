use std::cell::RefCell;

use crate::{TimePoint, TraceInfo, record_custom_scope};

type OpenScopes = Vec<TimePoint>;

thread_local! {
    static OPEN_SCOPES: RefCell<OpenScopes> = RefCell::default();
}

pub struct Scope
{
    key: Key,
}

impl Scope
{
    #[must_use]
    pub fn start(data: &'static TraceInfo) -> Self
    {
        open_scope();
        Self {
            key: Key::from_static(data),
        }
    }
}

impl Drop for Scope
{
    fn drop(&mut self) { close_scope(self); }
}

pub(super) fn open_scope() { OPEN_SCOPES.with_borrow_mut(|slots| slots.push(TimePoint::now())) }
fn pop_scope_opening_time() -> TimePoint
{
    OPEN_SCOPES.with_borrow_mut(|slots| slots.pop().unwrap())
    /*
    if let Some(mut x) = OPEN_SCOPES.with_borrow_mut(Vec::pop)
    {
    if key != x.0
        {
        OPEN_SCOPES.with_borrow_mut(|v| {
            if let Some(y) = v.iter_mut().rev().find(|s| s.0 == key)
            {
            std::mem::swap(&mut x, y);
                }
            });
        }
        x.1
    }
    else
    {
        debug_assert!(false);
        Instant::now()
    }
    */
}

pub(super) fn close_scope(Scope { key }: &Scope)
{
    let start = pop_scope_opening_time();
    record_custom_scope(key.read(), start, TimePoint::now());
}

#[repr(transparent)]
#[derive(Clone, Copy)]
struct Key(&'static TraceInfo<'static>);

impl Key
{
    fn from_static(info: &'static TraceInfo) -> Self { Self(info) }

    fn read(self) -> &'static TraceInfo<'static> { self.0 }
}
