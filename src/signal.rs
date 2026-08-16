use std::{cell::RefCell, rc::Rc};

use crate::DIRTY;

#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Flags {
    SIGNALED,
    UNSIGNALED,
}

pub struct Signal<T> {
    pub value: Rc<RefCell<T>>,
}

impl<T> Signal<T> {
    pub fn create(value: T) -> Self {
        let data = Rc::new(RefCell::new(value));

        Self { value: data }
    }

    pub fn value(&self) -> T
    where
        T: Clone,
    {
        self.value.borrow().clone()
    }

    pub fn set(&self, f: impl FnOnce(&mut T)) {
        {
            let mut borrow = self.value.borrow_mut();
            f(&mut *borrow);
        }

        DIRTY.with(|flags| flags.set(Flags::SIGNALED));
    }
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value),
        }
    }
}

#[derive(Clone)]
pub enum Value<T> {
    Static(T),
    Dynamic(Signal<T>),
}

impl<T: Clone> Value<T> {
    pub fn get(&self) -> T {
        match self {
            Self::Static(value) => value.to_owned(),
            Self::Dynamic(value) => value.value(),
        }
    }
}

impl<T> From<T> for Value<T> {
    fn from(value: T) -> Self {
        Self::Static(value)
    }
}

impl<T: Clone> From<&Value<T>> for Value<T> {
    fn from(value: &Value<T>) -> Self {
        value.clone()
    }
}

impl<T> From<&Signal<T>> for Value<T> {
    fn from(signal: &Signal<T>) -> Self {
        Self::Dynamic(signal.clone())
    }
}
