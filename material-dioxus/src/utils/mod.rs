// mod weak_component_link;
// pub use weak_component_link::*;

use std::rc::Rc;

use dioxus::prelude::*;

/// See <https://github.com/DioxusLabs/dioxus/issues/1374>
#[derive(Clone)]
pub struct StaticCallback<T> {
    #[allow(clippy::type_complexity)]
    inner: Rc<RefCell<Box<dyn FnMut(T)>>>,
}

impl<T> StaticCallback<T> {
    pub fn call(&self, arg: T) {
        (self.inner.borrow_mut())(arg)
    }
}

impl<F, T> From<F> for StaticCallback<T>
where
    F: FnMut(T) + 'static,
{
    fn from(f: F) -> Self {
        Self {
            inner: Rc::new(RefCell::new(Box::new(f))),
        }
    }
}
