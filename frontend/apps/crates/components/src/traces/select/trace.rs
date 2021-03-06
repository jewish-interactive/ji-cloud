use shared::domain::jig::module::body::{Transform, _groups::design::{Trace, TraceShape}};
use std::rc::Rc;
pub struct SelectTrace {
    pub inner: Trace,
    pub on_select: Rc<Box<dyn Fn()>>
}

impl SelectTrace {
    pub fn new(trace: Trace, on_select: impl Fn() + 'static) -> Self {
        Self {
            inner: trace,
            on_select: Rc::new(Box::new(on_select))
        }
    }
}
