use std::cell::RefCell;

use crate::{record_custom_scope, Info, TimePoint};

type OpenScopes = Vec<TimePoint>;

thread_local! {
    static OPEN_SCOPES: RefCell<OpenScopes> = RefCell::default();
}

pub struct Scope
{
    info: Info,
}

impl Scope
{
    #[must_use]
    pub fn start(info: Info) -> Self
    {
        open_scope();
        Self {
            info,
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

pub(super) fn close_scope(Scope { info }: &Scope)
{
    let start = pop_scope_opening_time();
    record_custom_scope(info, start, TimePoint::now()); //might require a check to ensure the ends are sorted
}
